"""
Task1 启动文件 — 巡线 + QR 码识别停止
用法: ros2 launch task_controller task1_launch.py
"""

from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package="task_controller",
            executable="main_controller_node",
            name="task1_controller",
            output="screen",
            parameters=[
                {"task_id": 1},
                {"task1.linear_speed": 0.5},
                {"task1.kp": 2.0},
                {"task1.miss_timeout": 1.0},
                {"task1.timer_period": 0.033},
            ],
        ),
    ])
