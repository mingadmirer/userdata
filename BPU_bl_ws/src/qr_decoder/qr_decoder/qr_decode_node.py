#!/usr/bin/env python3
"""
QR 码解码节点 (WeChat QR)
订阅 /qr_crop → BGR 直解 → CLAHE → 解码
发布: /qr_result
"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from std_msgs.msg import String


def imgmsg_to_numpy(msg):
    if msg.encoding == 'bgr8':
        return np.frombuffer(msg.data, dtype=np.uint8).reshape(msg.height, msg.width, 3)
    raise ValueError(f'unsupported encoding: {msg.encoding}')


class QRDecode(Node):
    def __init__(self):
        super().__init__('qr_decode')
        self.detector = cv2.wechat_qrcode_WeChatQRCode()
        self.busy = False
        self.last_result = None

        self.crop_sub = self.create_subscription(
            Image, '/qr_crop', self.crop_cb, 10)
        self.result_pub = self.create_publisher(String, '/qr_result', 10)

        self.get_logger().info('qr_decode ready')

    def decode_once(self, img):
        # BGR 直接解
        data, _ = self.detector.detectAndDecode(img)
        if data and data[0]:
            return data[0]

        # CLAHE 增强
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
        enhanced = cv2.createCLAHE(2.0, (8, 8)).apply(gray)
        data, _ = self.detector.detectAndDecode(enhanced)
        if data and data[0]:
            return data[0]
        return None

    def crop_cb(self, msg: Image):
        if self.busy:
            return
        self.busy = True

        img = imgmsg_to_numpy(msg)
        data = self.decode_once(img)

        if data and data != self.last_result:
            self.last_result = data
            self.get_logger().info(f'QR: {data}')
            self.result_pub.publish(String(data=data))
        elif not data:
            self.last_result = None

        self.busy = False


def main():
    rclpy.init()
    node = QRDecode()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
