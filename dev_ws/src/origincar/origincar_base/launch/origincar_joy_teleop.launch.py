"""
OriginCar 手柄遥控启动文件
用法:
  ros2 launch origincar_base origincar_joy_teleop.launch.py

参数:
  max_speed_mps     - 最大线速度 (m/s), 默认 1.5
  max_steering_rad  - 最大转向角 (rad), 默认 0.5
  speed_gear        - 初始档位 (low/normal/high), 默认 normal
  joy_dev           - 手柄设备路径, 默认 /dev/input/js0
"""

import os
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
import launch_ros.actions


def generate_launch_description():
    # ---- 参数声明 ----
    joy_dev_arg = DeclareLaunchArgument(
        'joy_dev', default_value='/dev/input/js0',
        description='Joystick device path'
    )
    max_speed_arg = DeclareLaunchArgument(
        'max_speed_mps', default_value='1.5',
        description='Max linear speed (m/s)'
    )
    max_steering_arg = DeclareLaunchArgument(
        'max_steering_rad', default_value='0.5',
        description='Max steering angle (rad)'
    )
    speed_gear_arg = DeclareLaunchArgument(
        'speed_gear', default_value='normal',
        description='Initial speed gear: low / normal / high'
    )
    deadzone_arg = DeclareLaunchArgument(
        'deadzone', default_value='0.05',
        description='Joystick deadzone ratio (0.0 ~ 1.0)'
    )

    # ---- joy_linux_node: 手柄驱动 → /joy ----
    joy_node = launch_ros.actions.Node(
        package='joy_linux',
        executable='joy_linux_node',
        name='joy_linux_node',
        parameters=[{
            'dev': LaunchConfiguration('joy_dev'),
            'deadzone': 0.01,
            'autorepeat_rate': 20.0,      # 20Hz 重复发送
            'coalesce_interval': 0.02,    # 50Hz 合并间隔
        }],
    )

    # ---- origincar_joy_teleop: Joy → Twist ----
    teleop_node = launch_ros.actions.Node(
        package='origincar_base',
        executable='origincar_joy_teleop.py',
        name='origincar_joy_teleop',
        parameters=[{
            'max_speed_mps': LaunchConfiguration('max_speed_mps'),
            'max_steering_rad': LaunchConfiguration('max_steering_rad'),
            'speed_gear': LaunchConfiguration('speed_gear'),
            'deadzone': LaunchConfiguration('deadzone'),
        }],
    )

    return LaunchDescription([
        joy_dev_arg,
        max_speed_arg,
        max_steering_arg,
        speed_gear_arg,
        deadzone_arg,
        joy_node,
        teleop_node,
    ])
