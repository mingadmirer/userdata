#!/usr/bin/env python3
"""
简易巡线 Demo
朝 Y 坐标最大的 line 目标前进，10Hz 更新，1 秒没看到线就停车
"""

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from detection_interfaces.msg import DetectionArray


class LineFollower(Node):
    def __init__(self):
        super().__init__('line_follower')
        self.pub = self.create_publisher(Twist, 'cmd_vel', 10)
        self.sub = self.create_subscription(
            DetectionArray, '/detection_result', self.callback, 10)
        self.timer = self.create_timer(0.1, self.publish_cmd)
        self.latest_cx = None
        self.miss_count = 0

        self.get_logger().info('line_follower ready')

    def callback(self, msg: DetectionArray):
        best = None
        for d in msg.detections:
            if d.name != 'line':
                continue
            cy = (d.y1 + d.y2) / 2
            cx = (d.x1 + d.x2) / 2
            if best is None or cy > best[0]:
                best = (cy, cx)
        if best:
            self.latest_cx = best[1]
            self.miss_count = 0
        else:
            self.miss_count += 1

    def publish_cmd(self):
        msg = Twist()

        if self.miss_count >= 10 or self.latest_cx is None:
            msg.linear.x = 0.0
            msg.angular.z = 0.0
        else:
            error = self.latest_cx - 0.5
            msg.linear.x = 0.5           # 前进速度 m/s
            msg.angular.z = -error * 2.0  # 转向角速度 rad/s, P=2.0

        self.pub.publish(msg)


def main():
    rclpy.init()
    node = LineFollower()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
