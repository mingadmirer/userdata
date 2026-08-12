#!/usr/bin/env python3
"""
巡线 + zt 避障 + QR 码停止 Demo
  /detection_result → "line" 巡线 / "zt" 避障
  /qr_result → 解码文本 → 停车退出
  避障转向后朝反方向回找线
"""

from enum import Enum
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from std_msgs.msg import String
from detection_interfaces.msg import DetectionArray


class State(Enum):
    FOLLOW = "follow"       # 巡线
    AVOID = "avoid"         # 避障转向
    COUNTER = "counter"     # 反方向固定时长
    RECOVER = "recover"     # 反方向找线 (不限时)


class LineAvoidDemo(Node):
    def __init__(self):
        super().__init__("line_avoid_demo")

        # ── 参数 ────────────────────────────────────
        self.declare_parameter("linear_speed", 0.5)
        self.declare_parameter("kp_line", 2.0)
        self.declare_parameter("miss_timeout", 1.0)
        self.declare_parameter("zt_area_ratio", 0.12)       # zt 占画面比 > 此值触发避障
        self.declare_parameter("avoid_duration", 0.6)         # 避障/回打固定转向秒
        self.declare_parameter("recover_timeout", 5.0)       # 找线最大时长 (兜底)
        self.declare_parameter("timer_period", 0.033)        # 30Hz

        self.linear_speed = self.get_parameter("linear_speed").value
        self.kp_line = self.get_parameter("kp_line").value
        self.zt_ratio = self.get_parameter("zt_area_ratio").value
        self.avoid_dur = self.get_parameter("avoid_duration").value
        self.recover_timeout = self.get_parameter("recover_timeout").value
        timer_period = self.get_parameter("timer_period").value
        miss_timeout = self.get_parameter("miss_timeout").value

        self._miss_limit = int(miss_timeout / timer_period)

        # ── 运行时状态 ──────────────────────────────
        self.state = State.FOLLOW
        self._state_timer = 0.0
        self._avoid_dir = 0.0        # 避障转向方向 (+1 右转, -1 左转)

        self._latest_cx = None
        self._miss_count = 0
        self._zt_overlap = False     # 当前帧 zt 是否过大
        self._zt_cx = 0.0

        self._done = False
        self._qrcode_text = None

        # ── 发布 / 订阅 ─────────────────────────────
        self.cmd_pub = self.create_publisher(Twist, "cmd_vel", 10)
        self._det_sub = self.create_subscription(
            DetectionArray, "/detection_result", self._det_cb, 10)
        self._qr_sub = self.create_subscription(
            String, "/qr_result", self._qr_cb, 10)
        self._timer = self.create_timer(timer_period, self._control_loop)

        self.get_logger().info("line_avoid_demo ready — 巡线 + zt避障 + QR停止")

    # ── 回调 ────────────────────────────────────────
    def _qr_cb(self, msg: String):
        text = msg.data.strip()
        if not text or self._done:
            return
        self._qrcode_text = text
        self._done = True
        self.cmd_pub.publish(Twist())
        self.get_logger().info(f">>> QR: \"{text}\" — 任务完成")

    def _det_cb(self, msg: DetectionArray):
        if self._done:
            return

        best_cx, best_cy = None, 2.0
        zt_max_area = 0.0
        zt_cx = 0.5

        for d in msg.detections:
            area = (d.x2 - d.x1) * (d.y2 - d.y1)

            if d.name == "line":
                cy = (d.y1 + d.y2) / 2.0
                cx = (d.x1 + d.x2) / 2.0
                if cy < best_cy:
                    best_cy = cy
                    best_cx = cx

            elif d.name == "zt":
                if area > zt_max_area:
                    zt_max_area = area
                    zt_cx = (d.x1 + d.x2) / 2.0

        # 更新线
        if best_cx is not None:
            self._latest_cx = best_cx
            self._miss_count = 0
        else:
            self._miss_count += 1

        # 更新 zt
        self._zt_overlap = zt_max_area > self.zt_ratio
        self._zt_cx = zt_cx

        # 避障立即触发 (不等到 control_loop)
        if self._zt_overlap and self.state == State.FOLLOW:
            self._avoid_dir = -1.0 if self._zt_cx > 0.5 else 1.0
            self.state = State.AVOID
            self._state_timer = 0.0
            dir_name = "左" if self._avoid_dir < 0 else "右"
            self.get_logger().warn(f"zt 障碍过大 → {dir_name}转避障")

    # ── 控制主循环 30Hz ─────────────────────────────
    def _control_loop(self):
        if self._done:
            return

        self._state_timer += self.get_parameter("timer_period").value
        twist = Twist()

        # ── 状态: 避障转向 (固定时长) ───────────────
        if self.state == State.AVOID:
            twist.linear.x = self.linear_speed
            twist.angular.z = self._avoid_dir * 3.0
            if self._state_timer >= self.avoid_dur:
                self.state = State.COUNTER
                self._state_timer = 0.0
                self.get_logger().info("避障完成 → 反方向回打")

        # ── 状态: 反方向固定时长 ─────────────────────
        elif self.state == State.COUNTER:
            twist.linear.x = self.linear_speed
            twist.angular.z = -self._avoid_dir * 3.0
            if self._state_timer >= self.avoid_dur:
                self.state = State.RECOVER
                self._state_timer = 0.0
                self.get_logger().info("回打完成 → 开始找线")

        # ── 状态: 反方向找线 → 转到线居中 (不限时) ───
        elif self.state == State.RECOVER:
            twist.linear.x = self.linear_speed
            twist.angular.z = -self._avoid_dir * 3.0
            if self._latest_cx is not None and abs(self._latest_cx - 0.5) < 0.30:
                self.state = State.FOLLOW
                self._state_timer = 0.0
                self._miss_count = 0
                self.get_logger().info("线已居中 → 回到巡线")
            elif self._state_timer >= self.recover_timeout:
                self.state = State.FOLLOW
                self._state_timer = 0.0
                self._miss_count = 0
                self.get_logger().warn("找线超时 → 强制回巡线")

        # ── 状态: 巡线 ──────────────────────────────
        else:
            if self._latest_cx is not None and self._miss_count < self._miss_limit:
                error = self._latest_cx - 0.5
                twist.linear.x = self.linear_speed
                twist.angular.z = -error * self.kp_line
            else:
                twist.linear.x = 0.0
                twist.angular.z = 0.0

        self.cmd_pub.publish(twist)


def main():
    rclpy.init()
    node = LineAvoidDemo()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.cmd_pub.publish(Twist())
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
