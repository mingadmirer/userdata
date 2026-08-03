"""
Task1 — 巡线 + QR 解码停止
  1. 订阅 /detection_result，找到 "line" 目标，P 控制跟随
  2. 同时订阅 /qr_result（qr_decode 解码节点输出），拿到解码文本
  3. /qr_result 输出非空 → 停车 → 输出解码结果 → 任务完成
"""

from task_controller.state_machine import BaseTask, TaskState
from geometry_msgs.msg import Twist
from std_msgs.msg import String
from detection_interfaces.msg import DetectionArray


class Task1LineFollow(BaseTask):
    def __init__(self, node, name: str = "task1_line_follow"):
        super().__init__(node, name)

        # ── 参数 ────────────────────────────────────────
        self.node.declare_parameter("task1.linear_speed", 0.5)
        self.node.declare_parameter("task1.kp", 2.0)
        self.node.declare_parameter("task1.miss_timeout", 1.0)
        self.node.declare_parameter("task1.timer_period", 1.0 / 30.0)

        self.linear_speed = self.node.get_parameter("task1.linear_speed").value
        self.kp = self.node.get_parameter("task1.kp").value
        self.miss_timeout = self.node.get_parameter("task1.miss_timeout").value
        self.timer_period = self.node.get_parameter("task1.timer_period").value

        # ── 运行时状态 ──────────────────────────────────
        self._latest_cx = None
        self._miss_count = 0
        self._miss_limit = int(self.miss_timeout / self.timer_period)
        self._qrcode_text = None         # 解码节点输出的文字
        self._done = False

        # ── 发布器 ──────────────────────────────────────
        self.cmd_pub = self.node.create_publisher(Twist, "cmd_vel", 10)

        # ── 订阅器 (on_enter 时才创建) ──────────────────
        self._det_sub = None
        self._qr_sub = None
        self._timer = None

    # ── 生命周期 ────────────────────────────────────────
    def on_enter(self):
        self._latest_cx = None
        self._miss_count = 0
        self._qrcode_text = None
        self._done = False

        # 订阅 DNN 检测结果 → 巡线
        self._det_sub = self.node.create_subscription(
            DetectionArray, "/detection_result", self._det_callback, 10)

        # 订阅 QR 解码节点输出 → 结束条件
        self._qr_sub = self.node.create_subscription(
            String, "/qr_result", self._qr_callback, 10)

        self._timer = self.node.create_timer(self.timer_period, self._control_loop)

        self.node.get_logger().info(
            f"[{self.name}] 巡线启动 — speed={self.linear_speed:.1f} m/s, "
            f"kp={self.kp:.1f}, timeout={self.miss_timeout:.1f}s")
        self.node.get_logger().info(
            f"[{self.name}] 等待 /qr_result 解码输出...")

    def on_update(self):
        pass

    def on_exit(self):
        self._publish_stop()
        for sub in ("_det_sub", "_qr_sub"):
            s = getattr(self, sub, None)
            if s:
                self.node.destroy_subscription(s)
                setattr(self, sub, None)
        if self._timer:
            self.node.destroy_timer(self._timer)
            self._timer = None

        if self._qrcode_text:
            self.node.get_logger().info(
                f"[{self.name}]  QR 解码结果: {self._qrcode_text} ")

        self.node.get_logger().info(f"[{self.name}] 已退出")

    def check_done(self) -> bool:
        return self._done

    # ── 回调 ────────────────────────────────────────────
    def _qr_callback(self, msg: String):
        """QR 解码节点输出 → 立即停车 → 标记完成"""
        text = msg.data.strip()
        if not text:
            return
        self._qrcode_text = text
        self._done = True
        self._result_msg = text
        self._publish_stop()  # 立即发停止指令，不等下一个 timer 周期
        self.node.get_logger().info(
            f"[{self.name}] >>> /qr_result 解码成功: \"{text}\", 已停车")

    def _det_callback(self, msg: DetectionArray):
        """只找 line 目标"""
        if self._done:
            return

        best_line_cx = None
        best_line_cy = 2.0          # Y 越小越远，初始 > 1 保证能匹配

        for d in msg.detections:
            if d.name != "line":
                continue
            cy = (d.y1 + d.y2) / 2.0
            cx = (d.x1 + d.x2) / 2.0
            if cy < best_line_cy:   # 取 Y 最小的（图像上方，最远的线）
                best_line_cy = cy
                best_line_cx = cx

        if best_line_cx is not None:
            self._latest_cx = best_line_cx
            self._miss_count = 0
        else:
            self._miss_count += 1

    def _control_loop(self):
        if self.state != TaskState.RUNNING or self._done:
            return

        twist = Twist()

        if self._latest_cx is None or self._miss_count >= self._miss_limit:
            twist.linear.x = 0.0
            twist.angular.z = 0.0
            if self._miss_count > 0 and self._miss_count % self._miss_limit == 0:
                self.node.get_logger().warn(
                    f"[{self.name}] 未检测到 line ({self._miss_count}/{self._miss_limit})")
        else:
            error = self._latest_cx - 0.5
            twist.linear.x = self.linear_speed
            twist.angular.z = -error * self.kp

        self.cmd_pub.publish(twist)

    def _publish_stop(self):
        self.cmd_pub.publish(Twist())

    @property
    def qrcode_text(self) -> str:
        return self._qrcode_text or ""
