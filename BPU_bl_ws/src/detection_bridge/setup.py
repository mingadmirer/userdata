import os
from glob import glob
from setuptools import setup

package_name = 'detection_bridge'

setup(
    name=package_name,
    version='0.1.0',
    packages=[package_name],
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        (os.path.join('share', package_name, 'launch'),
            glob('launch/*.launch.py')),
        (os.path.join('lib', package_name), [
            'scripts/detection_bridge_node',
        ]),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='root',
    maintainer_email='root@todo.todo',
    description='Bridge from hobot_dnn_detection to detection_interfaces',
    license='Apache-2.0',
    entry_points={
        'console_scripts': [
            'detection_bridge_node = detection_bridge.bridge_node:main',
        ],
    },
)
