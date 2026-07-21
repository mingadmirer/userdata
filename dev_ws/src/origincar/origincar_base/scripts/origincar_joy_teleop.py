#!/usr/bin/env python3
"""
OriginCar 手柄遥控节点 (阿克曼模式)
将 Xbox 360 手柄输入转换为 Twist 消息,再经 cmd_vel_to_ackermann_drive 转为阿克曼指令

Xbox 360 按键映射 (joy_linux_node 归一化后 axes/buttons 均为 [0,1] 或 0/1):
  - 左摇杆水平 (axis 0)  → 转向 (steering)
  - 左扳机      (axis 2)  → 前进油门
  - 右扳机      (axis 5)  → 后退/刹车
  - A 键        (btn  0)  → 急停/恢复
  - B 键        (btn  1)  → 切换速度档位
  - X 键        (btn  2)  → 原地左转 (差速)
  - Y 键        (btn  3)  → 原地右转 (差速)
"""

import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Joy
from geometry_msgs.msg import Twist


class OrigincarJoyTeleop(Node):
    def __init__(self):
        super().__init__('origincar_joy_teleop')

        # ---------- 参数声明 ----------
        self.declare_parameter('max_speed_mps', 1.5)          # 最大线速度 m/s
        self.declare_parameter('max_steering_rad', 0.5)       # 最大转向角 rad
        self.declare_parameter('speed_gear', 'normal')        # 初始档位: low / normal / high

        # 轴索引 (joy_linux_node 归一化后: 摇杆 [-1,1]，扳机 [0,1])
        self.declare_parameter('axis_steering', 0)             # 转向 — 左摇杆水平
        self.declare_parameter('axis_throttle', 2)             # 油门 — 左扳机 LT
        self.declare_parameter('axis_brake', 5)                # 刹车 — 右扳机 RT

        # 按键索引
        self.declare_parameter('btn_deadman', 0)               # 急停切换 A
        self.declare_parameter('btn_gear', 1)                  # 档位切换 B
        self.declare_parameter('btn_rotate_left', 2)           # 原地左转 X
        self.declare_parameter('btn_rotate_right', 3)          # 原地右转 Y

        self.declare_parameter('deadzone', 0.05)               # 转向死区比例
        self.declare_parameter('rotate_speed_rad', 1.0)        # 原地旋转角速度 rad/s

        # 读取参数值
        self.max_speed      = self.get_parameter('max_speed_mps').value
        self.max_steering   = self.get_parameter('max_steering_rad').value
        self.deadzone       = self.get_parameter('deadzone').value
        self.rotate_speed   = self.get_parameter('rotate_speed_rad').value

        # 轴/按键索引
        self.idx_steering    = self.get_parameter('axis_steering').value
        self.idx_throttle    = self.get_parameter('axis_throttle').value
        self.idx_brake       = self.get_parameter('axis_brake').value
        self.idx_deadman     = self.get_parameter('btn_deadman').value
        self.idx_gear        = self.get_parameter('btn_gear').value
        self.idx_rot_left    = self.get_parameter('btn_rotate_left').value
        self.idx_rot_right   = self.get_parameter('btn_rotate_right').value

        # 档位配置
        self.gear_ratios = {'low': 0.5, 'normal': 1.0, 'high': 1.5}
        self.speed_gear = self.get_parameter('speed_gear').value
        if self.speed_gear not in self.gear_ratios:
            self.get_logger().warn(f'Unknown gear "{self.speed_gear}", fallback to normal')
            self.speed_gear = 'normal'
        self.gear_ratio = self.gear_ratios[self.speed_gear]

        # 急停 & 按键去抖
        self.e_stop = False
        self.last_buttons = {}  # btn_id → last_state

        # ---------- 发布/订阅 ----------
        self.twist_pub = self.create_publisher(Twist, 'cmd_vel', 10)
        self.joy_sub   = self.create_subscription(Joy, 'joy', self.joy_callback, 10)

        self.get_logger().info(
            f'OriginCar Joy Teleop started | '
            f'Speed: {self.max_speed:.1f} m/s | '
            f'Steering: {self.max_steering:.2f} rad | '
            f'Gear: {self.speed_gear} (x{self.gear_ratio}) | '
            f'Deadzone: {self.deadzone}'
        )

    # ----------------------------------------------------------------
    def _safe_axis(self, msg: Joy, idx: int) -> float:
        """安全读取轴值，越界返回 0"""
        if 0 <= idx < len(msg.axes):
            return float(msg.axes[idx])
        return 0.0

    def _safe_btn(self, msg: Joy, idx: int) -> int:
        """安全读取按键值，越界返回 0"""
        if 0 <= idx < len(msg.buttons):
            return int(msg.buttons[idx])
        return 0

    def _btn_rising_edge(self, btn_id: int, state: int) -> bool:
        """检测按键上升沿 (按下瞬间)"""
        last = self.last_buttons.get(btn_id, 0)
        self.last_buttons[btn_id] = state
        return (state == 1 and last == 0)

    # ----------------------------------------------------------------
    def apply_deadzone(self, value: float) -> float:
        """
        摇杆死区: [-deadzone, deadzone] → 0
        超出部分线性映射到 [-1, 1]
        """
        if abs(value) < self.deadzone:
            return 0.0
        sign = 1.0 if value > 0 else -1.0
        return sign * (abs(value) - self.deadzone) / (1.0 - self.deadzone)

    # ----------------------------------------------------------------
    def joy_callback(self, msg: Joy):
        """手柄回调: Joy → Twist (经由 cmd_vel_to_ackermann_drive 转为阿克曼)"""

        # ---- 读轴 ----
        # 摇杆: joy_linux 已归一化到 [-1, 1], 0=居中
        raw_steering = self._safe_axis(msg, self.idx_steering)

        # 扳机: joy_linux 已归一化到 [0, 1], 0=未按, 1=全按
        raw_throttle = self._safe_axis(msg, self.idx_throttle)
        raw_brake    = self._safe_axis(msg, self.idx_brake)

        # ---- 处理按键 ----
        # A 键: 急停切换
        btn_deadman = self._safe_btn(msg, self.idx_deadman)
        if self._btn_rising_edge(self.idx_deadman, btn_deadman):
            self.e_stop = not self.e_stop
            self.get_logger().warn(
                f'[E-STOP] {"ACTIVATED" if self.e_stop else "RELEASED"}',
                throttle_duration_sec=1.0
            )

        # B 键: 档位切换
        btn_gear = self._safe_btn(msg, self.idx_gear)
        if self._btn_rising_edge(self.idx_gear, btn_gear):
            gears = ['low', 'normal', 'high']
            idx = gears.index(self.speed_gear)
            self.speed_gear = gears[(idx + 1) % len(gears)]
            self.gear_ratio = self.gear_ratios[self.speed_gear]
            self.get_logger().info(
                f'[GEAR] → {self.speed_gear.upper()} (x{self.gear_ratio})',
                throttle_duration_sec=1.0
            )

        # ---- 计算 Twist ----
        twist = Twist()

        if self.e_stop:
            # 急停 → 全零
            self.twist_pub.publish(twist)
            return

        # X / Y 键: X原地左转 / Y原地右转 (差速模式, 无视油门刹车)
        btn_rot_left  = self._safe_btn(msg, self.idx_rot_left)
        btn_rot_right = self._safe_btn(msg, self.idx_rot_right)

        if btn_rot_left:
            twist.angular.z = self.rotate_speed          # 正值 → 左转
        elif btn_rot_right:
            twist.angular.z = -self.rotate_speed         # 负值 → 右转
        else:
            # 正常阿克曼模式: 转向 = 左摇杆
            steering = self.apply_deadzone(raw_steering)  # [-1, 1]
            twist.angular.z = steering * self.max_steering

        # 速度: 油门 - 刹车（扳机已归一化到 [0,1]）
        speed_cmd = (raw_throttle - raw_brake) * self.gear_ratio * self.max_speed
        twist.linear.x = speed_cmd

        self.twist_pub.publish(twist)


def main():
    rclpy.init()
    node = OrigincarJoyTeleop()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
