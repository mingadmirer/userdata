#!/usr/bin/env python3
"""
QR 码裁切节点
订阅: /image (MJPEG) + hobot_dnn_detection
检测到 qrcode 时裁出 bbox 区域，发布裁切图
发布: /qr_crop
"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import CompressedImage, Image
from ai_msgs.msg import PerceptionTargets


def numpy_to_imgmsg(arr, stamp, frame_id='camera'):
    """手动构造 Image 消息, 不依赖 cv_bridge"""
    msg = Image()
    msg.header.stamp = stamp
    msg.header.frame_id = frame_id
    msg.height = arr.shape[0]
    msg.width = arr.shape[1]
    msg.encoding = 'bgr8'
    msg.is_bigendian = 0
    msg.step = arr.shape[1] * 3
    msg.data = arr.tobytes()
    return msg


class QRCropper(Node):
    def __init__(self):
        super().__init__('qr_cropper')
        self.latest_img = None

        self.img_sub = self.create_subscription(
            CompressedImage, '/image', self.img_cb, 10)
        self.ai_sub = self.create_subscription(
            PerceptionTargets, 'hobot_dnn_detection', self.ai_cb, 10)
        self.crop_pub = self.create_publisher(Image, '/qr_crop', 10)

        self.get_logger().info('qr_cropper ready')

    def img_cb(self, msg: CompressedImage):
        np_arr = np.frombuffer(msg.data, np.uint8)
        self.latest_img = cv2.imdecode(np_arr, cv2.IMREAD_COLOR)

    def ai_cb(self, msg: PerceptionTargets):
        if self.latest_img is None:
            return
        h, w = self.latest_img.shape[:2]

        for target in msg.targets:
            if target.type != 'qrcode':
                continue
            for roi in target.rois:
                x1 = max(0, int(roi.rect.x_offset))
                y1 = max(0, int(roi.rect.y_offset))
                x2 = min(w, int(roi.rect.x_offset + roi.rect.width))
                y2 = min(h, int(roi.rect.y_offset + roi.rect.height))
                if x2 <= x1 or y2 <= y1:
                    continue

                crop = self.latest_img[y1:y2, x1:x2]
                img_msg = numpy_to_imgmsg(crop, self.get_clock().now().to_msg())
                self.crop_pub.publish(img_msg)


def main():
    rclpy.init()
    node = QRCropper()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
