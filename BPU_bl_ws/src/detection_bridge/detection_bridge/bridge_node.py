#!/usr/bin/env python3
"""
DNN 检测结果 → detection_interfaces 桥接节点
订阅: hobot_dnn_detection (ai_msgs/PerceptionTargets)
发布: /detection_result (detection_interfaces/DetectionArray)
"""

import rclpy
from rclpy.node import Node
from ai_msgs.msg import PerceptionTargets
from detection_interfaces.msg import DetectionResult, DetectionArray


class DetectionBridge(Node):
    def __init__(self):
        super().__init__('detection_bridge')

        self.ai_sub = self.create_subscription(
            PerceptionTargets, 'hobot_dnn_detection', self.ai_cb, 10)
        self.pub = self.create_publisher(DetectionArray, '/detection_result', 10)

        self.get_logger().info('detection_bridge ready')

    def ai_cb(self, msg: PerceptionTargets):
        out = DetectionArray()
        out.header = msg.header

        for target in msg.targets:
            for roi in target.rois:
                r = DetectionResult()
                r.name = target.type
                r.confidence = roi.confidence
                r.x1 = float(roi.rect.x_offset)
                r.y1 = float(roi.rect.y_offset)
                r.x2 = float(roi.rect.x_offset + roi.rect.width)
                r.y2 = float(roi.rect.y_offset + roi.rect.height)
                out.detections.append(r)

        self.pub.publish(out)


def main():
    rclpy.init()
    node = DetectionBridge()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
