#!/usr/bin/env python3
"""
阿克曼底盘测试脚本 — 直接发包到 /ackermann_cmd，不经过 Twist 转换。

用法:
    ros2 run origincar_base akm_test.py

按键:
    w / s    前进 / 后退 (每次 ±0.1 m/s)
    a / d    左转 / 右转 (每次 ±0.1 rad)
    空格     紧急停止
    r        回正舵机 (steering = 0)
    1~5      设置固定速度: 0.1, 0.2, 0.3, 0.5, 1.0 m/s
    q        退出

运行时持续发布 AckermannDriveStamped 到 /ackermann_cmd (50Hz 保持连接)。
"""

import sys
import select
import termios
import tty

import rclpy
from rclpy.node import Node
from ackermann_msgs.msg import AckermannDriveStamped


BANNER = """
╔══════════════════════════════════════╗
║     Ackermann 底盘直驱测试           ║
║──────────────────────────────────────║
║  w/s  前进/后退  │ ±0.1 m/s         ║
║  a/d  左转/右转  │ ±0.1 rad          ║
║  空格  紧急停止                       ║
║  r     回正舵机  │ steering = 0      ║
║  1~5   固定速度  │ .1 .2 .3 .5 1.0  ║
║  q     退出                          ║
╚══════════════════════════════════════╝
"""


def get_key() -> str:
    """非阻塞读取单个按键。"""
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setraw(fd)
        if select.select([sys.stdin], [], [], 0.01)[0]:
            ch = sys.stdin.read(1)
            return ch
        return ""
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)


class AkmTester(Node):
    def __init__(self):
        super().__init__("akm_tester")
        self.pub = self.create_publisher(AckermannDriveStamped, "/ackermann_cmd", 10)
        self.speed = 0.0
        self.steering = 0.0
        self.paused = True  # 初始静止, 需按 w 才开始走

        # 50Hz 持续发包, 保持和下位机的心跳连接
        self.timer = self.create_timer(0.02, self._publish)

        print(BANNER)

    def _publish(self):
        msg = AckermannDriveStamped()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = "base_link"
        msg.drive.speed = self.speed
        msg.drive.steering_angle = self.steering
        self.pub.publish(msg)

    def _print_state(self):
        print(
            f"\r  speed={self.speed:+.2f} m/s  "
            f"steering={self.steering:+.3f} rad  "
            f"{'[停]' if self.paused else '[运动中]'}   ",
            end="",
            flush=True,
        )

    def step_speed(self, delta: float):
        self.speed += delta
        self._print_state()

    def step_steering(self, delta: float):
        self.steering += delta
        self._print_state()

    def emergency_stop(self):
        self.speed = 0.0
        self.steering = 0.0
        self.paused = True
        print("\r  ⬛ 紧急停止!                                ")

    def center_steering(self):
        self.steering = 0.0
        self._print_state()

    def set_speed(self, s: float):
        self.speed = s
        self._print_state()

    def run(self):
        self._print_state()
        while rclpy.ok():
            key = get_key()
            if not key:
                rclpy.spin_once(self, timeout_sec=0.01)
                continue

            if key == "w":
                self.paused = False
                self.step_speed(+0.1)
            elif key == "s":
                self.paused = False
                self.step_speed(-0.1)
            elif key == "a":
                self.paused = False
                self.step_steering(+0.1)
            elif key == "d":
                self.paused = False
                self.step_steering(-0.1)
            elif key == " ":
                self.emergency_stop()
            elif key == "r":
                self.center_steering()
            elif key in "12345":
                values = {"1": 0.1, "2": 0.2, "3": 0.3, "4": 0.5, "5": 1.0}
                self.paused = False
                self.set_speed(values[key])
            elif key == "q":
                self.emergency_stop()
                self._publish()
                print("\n  退出测试")
                break
            elif key == "\x03":  # Ctrl+C
                break

            rclpy.spin_once(self, timeout_sec=0.01)


def main():
    rclpy.init()
    node = AkmTester()
    try:
        node.run()
    except KeyboardInterrupt:
        pass
    finally:
        node.emergency_stop()
        node._publish()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
