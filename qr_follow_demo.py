#!/usr/bin/env python3
"""
路点1 → 连续5帧二维码 → 追码 → QR解码 → 路点2(要对准)
  /odom_debug  → 导航
  /detection_result → "qrcode" 连续5帧确认
  /qr_result → 解码 → 切换路点2
"""

import math
from enum import Enum
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from std_msgs.msg import String
from navi_bringup.msg import OdomDebug
from detection_interfaces.msg import DetectionArray


class State(Enum):
    NAVIGATE1 = "navigate1"     # 朝路点1跑
    SEARCH = "search"           # 到路点1没看到码 → 慢速右前找码
    QR_FOLLOW = "qr_follow"     # 追着二维码跑
    TURN_TO_WP2 = "turn_to_wp2" # 倒车对准路点2
    NAVIGATE2 = "navigate2"     # 朝路点2跑
    ALIGN_YAW = "align_yaw"     # 到了路点2后原地对准航向


class QRFollowDemo(Node):
    def __init__(self):
        super().__init__("qr_follow_demo")

        # ── 路点1 ────────────────────────────────────
        self.declare_parameter("wp1_x", 2.0)
        self.declare_parameter("wp1_y", 0.7)

        # ── 路点2 (要对准) ────────────────────────────
        self.declare_parameter("wp2_x", 2.2)
        self.declare_parameter("wp2_y", 1.7)
        self.declare_parameter("wp2_yaw", 91.0)            # 度

        # ── 精度阈值 ────────────────────────────────
        self.declare_parameter("wp1_search_radius", 0.5)   # 路点1附近搜索范围 m
        self.declare_parameter("pos_tol", 0.3)             # 路点2对准范围 m
        self.declare_parameter("yaw_tol", 5.0)             # 航向容忍 度

        # ── 控制参数 ────────────────────────────────
        self.declare_parameter("linear_speed", 0.5)
        self.declare_parameter("kp_nav", 0.8)
        self.declare_parameter("kp_qr", 2.0)
        self.declare_parameter("qr_confirm", 5)            # 连续N帧确认
        self.declare_parameter("timer_period", 0.033)

        # 读取参数
        self.wp1_x = self.get_parameter("wp1_x").value
        self.wp1_y = self.get_parameter("wp1_y").value
        self.wp2_x = self.get_parameter("wp2_x").value
        self.wp2_y = self.get_parameter("wp2_y").value
        self.wp2_yaw = self.get_parameter("wp2_yaw").value
        self.wp1_radius = self.get_parameter("wp1_search_radius").value
        self.pos_tol = self.get_parameter("pos_tol").value
        self.yaw_tol = self.get_parameter("yaw_tol").value
        self.linear_speed = self.get_parameter("linear_speed").value
        self.kp_nav = self.get_parameter("kp_nav").value
        self.kp_qr = self.get_parameter("kp_qr").value
        self.qr_confirm_num = self.get_parameter("qr_confirm").value

        # ── 状态 ────────────────────────────────────
        self.state = State.NAVIGATE1
        self._odom = OdomDebug()
        self._qr_cx = None
        self._qr_count = 0               # 连续看到二维码帧数
        self._done = False

        # ── 发布 / 订阅 ─────────────────────────────
        self.cmd_pub = self.create_publisher(Twist, "cmd_vel", 10)
        self._odom_sub = self.create_subscription(
            OdomDebug, "/odom_debug", self._odom_cb, 10)
        self._det_sub = self.create_subscription(
            DetectionArray, "/detection_result", self._det_cb, 10)
        self._qr_sub = self.create_subscription(
            String, "/qr_result", self._qr_cb, 10)
        timer_period = self.get_parameter("timer_period").value
        self._timer = self.create_timer(timer_period, self._control_loop)

        self.get_logger().info(
            f"ready — wp1({self.wp1_x},{self.wp1_y}) wp2({self.wp2_x},{self.wp2_y},{self.wp2_yaw}°)")

    # ── 回调 ────────────────────────────────────────
    def _qr_cb(self, msg: String):
        text = msg.data.strip()
        if not text or self.state != State.QR_FOLLOW:
            return
        self.state = State.TURN_TO_WP2
        self._qr_cx = None
        self._qr_count = 0
        self.get_logger().info(f">>> QR: \"{text}\" — 停车, 原地转向路点2")

    def _odom_cb(self, msg: OdomDebug):
        self._odom = msg

    def _det_cb(self, msg: DetectionArray):
        if self._done:
            return

        best_cx, best_cy = None, 2.0
        for d in msg.detections:
            if d.name != "qrcode":
                continue
            cy = (d.y1 + d.y2) / 2.0
            cx = (d.x1 + d.x2) / 2.0
            if cy < best_cy:
                best_cy = cy
                best_cx = cx

        self._qr_cx = best_cx

        # 只有在搜索状态才累计二维码帧数
        if self.state != State.SEARCH:
            return

        if best_cx is not None:
            self._qr_count += 1
        else:
            self._qr_count = 0

        if self._qr_count >= self.qr_confirm_num:
            self.state = State.QR_FOLLOW
            self.get_logger().info(f"连续{self.qr_confirm_num}帧确认二维码 → 开始追码")

    # ── 控制主循环 30Hz ─────────────────────────────
    def _control_loop(self):
        if self._done:
            return

        twist = Twist()

        # ── NAVIGATE1: 朝路点1 ───────────────────────
        if self.state == State.NAVIGATE1:
            twist = self._nav_to(self.wp1_x, self.wp1_y, None)
            # 大概到路点1附近(0.5m)还没看到码 → 切慢速搜索
            dx = abs(self._odom.x - self.wp1_x)
            dy = abs(self._odom.y - self.wp1_y)
            if dx < self.wp1_radius and dy < self.wp1_radius:
                self.state = State.SEARCH
                self.get_logger().info("到达路点1 → 慢速右前搜索二维码")

        # ── SEARCH: 慢速右前找码 ──────────────────────
        elif self.state == State.SEARCH:
            twist.linear.x = 0.2
            twist.angular.z = 3.0

        # ── QR_FOLLOW: 追码 ──────────────────────────
        elif self.state == State.QR_FOLLOW:
            if self._qr_cx is not None:
                error = self._qr_cx - 0.5
                twist.linear.x = self.linear_speed
                twist.angular.z = -error * self.kp_qr
            else:
                twist.angular.z = 0.5

        # ── TURN_TO_WP2: 倒车右转对准路点2方向 ────────
        elif self.state == State.TURN_TO_WP2:
            dx = self.wp2_x - self._odom.x
            dy = self.wp2_y - self._odom.y
            desired = math.degrees(math.atan2(dy, dx))
            error = self._normalize_angle(math.radians(desired - self._odom.yaw))
            twist.linear.x = -0.15           # 小倒车, 舵机才能动
            twist.angular.z = 3.0 if error > 0 else -3.0   # 朝误差方向转满
            if abs(error) < math.radians(8.0):
                self.state = State.NAVIGATE2
                twist.linear.x = 0.0
                twist.angular.z = 0.0
                self.get_logger().info(f"已对准路点2方向 → 开始前进")

        # ── NAVIGATE2: 朝路点2动态导航 ───────────────
        elif self.state == State.NAVIGATE2:
            dx = self.wp2_x - self._odom.x
            dy = self.wp2_y - self._odom.y
            dist = math.sqrt(dx * dx + dy * dy)
            desired = math.atan2(dy, dx)
            error = self._normalize_angle(desired - math.radians(self._odom.yaw))
            twist.linear.x = min(self.linear_speed, dist * 0.5)
            twist.angular.z = error * self.kp_nav

            # x,y 到位 → 切换对准航向
            if abs(self._odom.x - self.wp2_x) < self.pos_tol and abs(self._odom.y - self.wp2_y) < self.pos_tol:
                self.state = State.ALIGN_YAW
                self.get_logger().info(
                    f"到达路点2位置 x={self._odom.x:.3f} y={self._odom.y:.3f} → 对准航向")

        # ── ALIGN_YAW: 对准航向后就停, 不管xy ──────────
        elif self.state == State.ALIGN_YAW:
            error = self._normalize_angle(math.radians(self.wp2_yaw - self._odom.yaw))
            twist.linear.x = -0.1
            twist.angular.z = 3.0 if error > 0 else -3.0
            if abs(error) < math.radians(self.yaw_tol):
                self.cmd_pub.publish(Twist())   # 先发停车, 再标记完成
                self._done = True
                self.get_logger().info(
                    f"路点2完成! x={self._odom.x:.3f} y={self._odom.y:.3f} yaw={self._odom.yaw:.1f}°")
                return

        self.cmd_pub.publish(twist)

    def _nav_to(self, tx: float, ty: float, _tyaw_deg_unused) -> Twist:
        """朝 (tx, ty) 动态导航: 每帧根据里程计重新计算航向"""
        dx = tx - self._odom.x
        dy = ty - self._odom.y
        dist = math.sqrt(dx * dx + dy * dy)
        desired = math.atan2(dy, dx)
        error = self._normalize_angle(desired - math.radians(self._odom.yaw))
        t = Twist()
        t.linear.x = min(self.linear_speed, dist * 0.5)
        t.angular.z = error * self.kp_nav
        return t

    @staticmethod
    def _normalize_angle(a: float) -> float:
        while a > math.pi:
            a -= 2 * math.pi
        while a < -math.pi:
            a += 2 * math.pi
        return a


def main():
    rclpy.init()
    node = QRFollowDemo()
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
