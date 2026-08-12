#!/usr/bin/env python3
"""保存 QR 裁切图 — 不依赖 cv_bridge"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image


class SaveQRCrop(Node):
    def __init__(self):
        super().__init__('save_qr_crop')
        self.sub = self.create_subscription(Image, '/qr_crop', self.cb, 10)
        self.get_logger().info('waiting for /qr_crop...')

    def cb(self, msg: Image):
        arr = np.frombuffer(msg.data, dtype=np.uint8).reshape(msg.height, msg.width, -1)
        cv2.imwrite('/userdata/qr_crop.png', arr)
        h, w = arr.shape[:2]
        self.get_logger().info(f'saved /userdata/qr_crop.png ({w}x{h})')
        raise SystemExit


rclpy.init()
node = SaveQRCrop()
try:
    rclpy.spin(node)
except SystemExit:
    pass
