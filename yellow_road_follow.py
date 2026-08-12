#!/usr/bin/env python3
"""
黄绿道路区分循线 Demo
黄色 = 行驶路面, 绿色 = 非行驶区 → 朝黄色质心方向走，避开绿色
订阅 /image (CompressedImage MJPEG) → HSV 双掩膜 → cmd_vel
"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, QoSReliabilityPolicy, QoSHistoryPolicy
from sensor_msgs.msg import CompressedImage
from geometry_msgs.msg import Twist


class YellowRoadFollower(Node):
    def __init__(self):
        super().__init__("yellow_road_follower")

        # ── HSV: 黄色 (行驶路面) ────────────────────────
        self.declare_parameter("yellow_low_h", 20)
        self.declare_parameter("yellow_low_s", 60)
        self.declare_parameter("yellow_low_v", 80)
        self.declare_parameter("yellow_high_h", 35)
        self.declare_parameter("yellow_high_s", 255)
        self.declare_parameter("yellow_high_v", 255)

        # ── HSV: 绿色 (非行驶区) ────────────────────────
        self.declare_parameter("green_low_h", 40)
        self.declare_parameter("green_low_s", 40)
        self.declare_parameter("green_low_v", 40)
        self.declare_parameter("green_high_h", 85)
        self.declare_parameter("green_high_s", 255)
        self.declare_parameter("green_high_v", 255)

        # ── 控制 ───────────────────────────────────────
        self.declare_parameter("linear_speed", 0.3)
        self.declare_parameter("kp", 0.008)
        self.declare_parameter("roi_fraction", 0.33)

        self._load_params()

        qos = QoSProfile(
            reliability=QoSReliabilityPolicy.BEST_EFFORT,
            history=QoSHistoryPolicy.KEEP_LAST,
            depth=1,
        )
        self.sub = self.create_subscription(CompressedImage, "/image", self.callback, qos)
        self.pub = self.create_publisher(Twist, "cmd_vel", 10)
        self.get_logger().info("yellow_road_follower ready")

    def _load_params(self):
        self.yellow_low = np.array([self.get_parameter(f"yellow_low_{c}").value for c in ("h", "s", "v")])
        self.yellow_high = np.array([self.get_parameter(f"yellow_high_{c}").value for c in ("h", "s", "v")])
        self.green_low = np.array([self.get_parameter(f"green_low_{c}").value for c in ("h", "s", "v")])
        self.green_high = np.array([self.get_parameter(f"green_high_{c}").value for c in ("h", "s", "v")])
        self.linear_speed = self.get_parameter("linear_speed").value
        self.kp = self.get_parameter("kp").value
        self.roi_frac = self.get_parameter("roi_fraction").value

    def callback(self, msg: CompressedImage):
        np_arr = np.frombuffer(msg.data, np.uint8)
        frame = cv2.imdecode(np_arr, cv2.IMREAD_COLOR)
        if frame is None:
            return

        h, w = frame.shape[:2]
        roi = frame[int(h * (1 - self.roi_frac)):h, :]
        hsv = cv2.cvtColor(roi, cv2.COLOR_BGR2HSV)

        # 黄色掩膜 (路面可行驶)
        yellow_mask = cv2.inRange(hsv, self.yellow_low, self.yellow_high)
        yellow_mask = cv2.erode(yellow_mask, None, iterations=1)
        yellow_mask = cv2.dilate(yellow_mask, None, iterations=2)

        # 绿色掩膜 (非行驶区)
        green_mask = cv2.inRange(hsv, self.green_low, self.green_high)
        green_mask = cv2.erode(green_mask, None, iterations=1)
        green_mask = cv2.dilate(green_mask, None, iterations=2)

        # 黄色质心 → 导航目标方向
        M_y = cv2.moments(yellow_mask)
        twist = Twist()

        # 检查画面中心底部是否踩到绿色 (危险)
        ch, cw = roi.shape[:2]
        center_bottom = green_mask[ch - ch // 4:ch, cw // 3:2 * cw // 3]
        on_green = cv2.countNonZero(center_bottom) / (center_bottom.size) > 0.15

        if on_green:
            # 即将压绿 → 后退 + 转向
            twist.linear.x = -0.1
            twist.angular.z = 0.5
        elif M_y["m00"] > 200:
            # 黄色路面 → 朝质心走
            cx = int(M_y["m10"] / M_y["m00"])
            error = cx - w // 2
            twist.linear.x = self.linear_speed
            twist.angular.z = -float(error) * self.kp
        else:
            # 看不到路 → 右转找
            twist.linear.x = 0.0
            twist.angular.z = 0.5

        self.pub.publish(twist)


def main():
    rclpy.init()
    node = YellowRoadFollower()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.pub.publish(Twist())  # 停车
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
