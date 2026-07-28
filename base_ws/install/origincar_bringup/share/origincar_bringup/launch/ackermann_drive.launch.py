"""
阿克曼模式 — ROS 端做 Twist→Ackermann 转换, STM32 直接用转向角
(适用于固件非 Akm_Car 模式的底盘)
"""

from launch import LaunchDescription
import launch_ros.actions

robot_parameters = [
    {'usart_port_name': '/dev/ttyACM0',
     'serial_baud_rate': 115200,
     'robot_frame_id': 'base_footprint',
     'odom_frame_id': 'odom_combined',
     'cmd_vel': 'cmd_vel',
     'product_number': 0,
     'akm_cmd_vel': 'ackermann_cmd'}
]


def generate_launch_description():
    return LaunchDescription([
        launch_ros.actions.Node(
            package='origincar_base',
            executable='origincar_base_node',
            parameters=robot_parameters,
            remappings=[('/cmd_vel', 'cmd_vel')],
        ),
        launch_ros.actions.Node(
            package='origincar_base',
            executable='cmd_vel_to_ackermann_drive.py',
            name='cmd_vel_to_ackermann_drive',
        ),
    ])
