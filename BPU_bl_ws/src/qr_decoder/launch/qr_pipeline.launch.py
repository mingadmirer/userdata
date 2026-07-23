from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='qr_decoder',
            executable='qr_cropper_node',
            name='qr_cropper',
            output='screen',
        ),
        Node(
            package='qr_decoder',
            executable='qr_decode_node',
            name='qr_decode',
            output='screen',
        ),
    ])
