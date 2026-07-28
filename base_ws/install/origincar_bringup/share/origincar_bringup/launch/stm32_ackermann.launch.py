"""
差速模式 — ROS 发 Twist, STM32 自己处理 (Akm_Car 固件用 Vz_to_Akm_Angle)
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
     'akm_cmd_vel': 'none'}
]


def generate_launch_description():
    return LaunchDescription([
        launch_ros.actions.Node(
            package='origincar_base',
            executable='origincar_base_node',
            parameters=robot_parameters,
        ),
    ])
