#!/usr/bin/env python3
"""
QR 码解码节点
订阅 /qr_crop → 缩小 → 找角点 → 透视矫正 → 解码
发布: /qr_result (解码文本)
"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from cv_bridge import CvBridge
from std_msgs.msg import String


MAX_SIDE = 300  # 缩放到 300px 以内，QR 足够识别且速度提升数倍


class QRDecode(Node):
    def __init__(self):
        super().__init__('qr_decode')
        self.bridge = CvBridge()
        self.detector = cv2.QRCodeDetector()
        self.busy = False
        self.last_result = None

        self.crop_sub = self.create_subscription(
            Image, '/qr_crop', self.crop_cb, 10)
        self.result_pub = self.create_publisher(String, '/qr_result', 10)

        self.get_logger().info('qr_decode ready')

    def resize(self, img):
        h, w = img.shape[:2]
        scale = min(1.0, MAX_SIDE / max(h, w))
        if scale < 1.0:
            return cv2.resize(img, (int(w * scale), int(h * scale)))
        return img

    def decode_once(self, img):
        """单次解码：缩小→找角点→透视矫正→解码"""
        img = self.resize(img)

        # 直接解码
        data, points, _ = self.detector.detectAndDecode(img)
        if data:
            return data

        # 找角点
        ret, corners = self.detector.detect(img)
        if not ret or corners is None:
            gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
            clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
            ret, corners = self.detector.detect(clahe.apply(gray))
        if not ret or corners is None:
            return None

        # 透视矫正
        corners = corners.reshape(4, 2).astype(np.float32)
        side = int(max(
            np.linalg.norm(corners[1] - corners[0]),
            np.linalg.norm(corners[2] - corners[3]),
            np.linalg.norm(corners[3] - corners[0]),
            np.linalg.norm(corners[2] - corners[1])))
        side = max(side, 100)

        dst = np.array([[0, 0], [side, 0], [side, side], [0, side]],
                       dtype=np.float32)
        warped = cv2.warpPerspective(
            img, cv2.getPerspectiveTransform(corners, dst), (side, side))

        data, _, _ = self.detector.detectAndDecode(warped)
        return data if data else None

    def crop_cb(self, msg: Image):
        if self.busy:
            return
        self.busy = True

        img = self.bridge.imgmsg_to_cv2(msg, 'bgr8')
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
