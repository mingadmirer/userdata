import os
from glob import glob
from setuptools import setup

package_name = 'task_controller'

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
            'scripts/main_controller_node',
        ]),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='mingadmirer',
    maintainer_email='user@example.com',
    description='State-machine task controller for OriginBot',
    license='MIT',
    entry_points={
        'console_scripts': [
            'main_controller_node = task_controller.main_controller:main',
        ],
    },
)
