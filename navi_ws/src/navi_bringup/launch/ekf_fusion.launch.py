"""
IMU + 轮式里程计 EKF 融合
输入: /imu/data_raw + /odom → 输出: /odom_combined (tf: odom_combined→base_footprint)
"""

import os
from launch import LaunchDescription
from launch_ros.actions import Node
from ament_index_python import get_package_share_directory

pkg_share = get_package_share_directory('navi_bringup')


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='robot_localization',
            executable='ekf_node',
            name='ekf_filter_node',
            output='screen',
            remappings=[('odometry/filtered', 'odom_combined')],
            parameters=[os.path.join(pkg_share, 'config', 'ekf.yaml')],
        ),
    ])
