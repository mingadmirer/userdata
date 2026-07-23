
from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='detection_bridge',
            executable='detection_bridge_node',
            name='detection_bridge',
            output='screen',
        ),
    ])
