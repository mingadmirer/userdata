import os
from glob import glob
from setuptools import setup

package_name = 'qr_decoder'

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
        (os.path.join('lib', package_name),
            [f'scripts/{s}' for s in ['qr_cropper_node', 'qr_decode_node']]),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='root',
    maintainer_email='root@todo.todo',
    description='QR code detector with DNN bbox + perspective correction',
    license='Apache-2.0',
    entry_points={
        'console_scripts': [
            'qr_cropper_node = qr_decoder.qr_decoder_node:main',
            'qr_decode_node = qr_decoder.qr_decode_node:main',
        ],
    },
)
