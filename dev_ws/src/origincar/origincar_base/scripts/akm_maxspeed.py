#!/usr/bin/env python3
"""
极限速度测试 — 直接发包到 /ackermann_cmd。
用法: ros2 run origincar_base akm_maxspeed.py [速度m/s] 默认3.0
      按 Ctrl+C 发停车指令后退出。
"""

import sys
import signal
import rclpy
from rclpy.node import Node
from ackermann_msgs.msg import AckermannDriveStamped


class MaxSpeedTest(Node):
    def __init__(self, target_speed: float):
        super().__init__("akm_maxspeed")
        self.pub = self.create_publisher(AckermannDriveStamped, "/ackermann_cmd", 10)
        self.speed = target_speed
        self.stopped = False

        # 只发一次就够了（下位机没看门狗）
        self._send(self.speed, 0.0)
        print(f"  ▸ 已发送 speed={self.speed:.2f} m/s  steering=0.0 rad")
        print(f"  ▸ 按 Ctrl+C 发送停车指令")

    def _send(self, speed: float, steering: float):
        msg = AckermannDriveStamped()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = "base_link"
        msg.drive.speed = speed
        msg.drive.steering_angle = steering
        self.pub.publish(msg)

    def stop(self):
        if not self.stopped:
            self._send(0.0, 0.0)
            self.stopped = True
            print("\n  ⬛ 已发送停车指令")


def main():
    rclpy.init()

    speed = float(sys.argv[1]) if len(sys.argv) > 1 else 3.0
    # 固件硬限制 3.5 m/s
    if speed > 3.5:
        print(f"  ⚠ {speed} 超过固件限幅 3.5 m/s，实际会被钳在 3.5")
    if speed > 3.5:
        speed = 3.5

    node = MaxSpeedTest(speed)

    def handler(sig, frame):
        node.stop()
        node.destroy_node()
        rclpy.shutdown()
        sys.exit(0)

    signal.signal(signal.SIGINT, handler)

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.stop()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
