#!/usr/bin/env python3
import os
import base64
import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, QoSReliabilityPolicy, QoSHistoryPolicy
from sensor_msgs.msg import CompressedImage
import cv2
import numpy as np
from volcenginesdkarkruntime import Ark

# ===================== 固定配置 =====================
api_key = os.getenv('ARK_API_KEY')
MODEL_ENDPOINT = "ep-20260811023426-jh85k"
BASE_URL = 'https://ark.cn-beijing.volces.com/api/v3'

IMAGE_QOS = QoSProfile(
    reliability=QoSReliabilityPolicy.BEST_EFFORT,
    history=QoSHistoryPolicy.KEEP_LAST,
    depth=1
)
# ==============================================================

class VisionOnceNode(Node):
    def __init__(self):
        super().__init__('robot_vision_ark')
        self.is_finish = False

        self.subscription = self.create_subscription(
            CompressedImage,
            '/image',
            self.image_callback,
            qos_profile=IMAGE_QOS
        )

        if not api_key:
            self.get_logger().error("❌ 未加载ARK_API_KEY环境变量")
            raise SystemExit(1)

        self.client = Ark(base_url=BASE_URL, api_key=api_key)
        self.get_logger().info("✅ 已订阅 /image CompressedImage 压缩图像，等待首帧推理")

    def image_callback(self, msg):
        if self.is_finish:
            return
        self.is_finish = True
        self.get_logger().info("📸 收到ROS MJPEG压缩图像，开始解码调用Seed2.1-Pro视觉模型")

        try:
            np_buf = np.frombuffer(msg.data, np.uint8)
            frame = cv2.imdecode(np_buf, cv2.IMREAD_COLOR)
            if frame is None:
                self.get_logger().error("❌ 图像解码失败，压缩数据异常")
                self.destroy_node()
                return

            ret, jpg_bytes = cv2.imencode(".jpg", frame)
            if not ret:
                self.get_logger().error("❌ JPG二次编码失败")
                self.destroy_node()
                return

            b64_encode = base64.b64encode(jpg_bytes).decode("utf-8")
            img_data_url = f"data:image/jpeg;base64,{b64_encode}"

            response = self.client.responses.create(
                model=MODEL_ENDPOINT,
                thinking={"type": "disabled"},
                input=[
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "input_image",
                                "image_url": img_data_url
                            },
                            {
                                "type": "input_text",
                                "text": "用100字以内只描述画面中出现的印刷品、卡片、海报或插画的具体内容，包括人物、物体、文字、颜色、构图。不要描述室内环境。"
                            }
                        ]
                    }
                ],
                max_output_tokens=256
            )

            # 提取响应文本：兼容多种响应格式
            full_text = ""
            for item in response.output:
                t = getattr(item, 'type', '')
                # 格式1: type='text' → item.text
                if t == 'text':
                    full_text += getattr(item, 'text', '')
                # 格式2: type='reasoning' → item.summary[].text (深度思考)
                elif t == 'reasoning' and hasattr(item, 'summary'):
                    for s in item.summary:
                        full_text += getattr(s, 'text', '')
                # 格式3: type='message' → item.content[].text (关闭深度思考)
                elif t == 'message' and hasattr(item, 'content'):
                    for c in item.content:
                        full_text += getattr(c, 'text', '')

            print("\n==================== Seed 2.1-Pro 视觉识别输出 ====================")
            print(full_text.strip() or "(空)")
            print("===================================================================\n")
            self.get_logger().info("🎉 大模型调用完成，程序自动关闭")

        except Exception as err:
            self.get_logger().error(f"⚠️ API调用异常：{str(err)}")
        finally:
            self.destroy_node()


def main(args=None):
    rclpy.init(args=args)
    vision_node = VisionOnceNode()
    try:
        rclpy.spin(vision_node)
    except KeyboardInterrupt:
        vision_node.get_logger().info("ℹ️ 手动Ctrl+C终止程序")
    except rclpy.executors.ExternalShutdownException:
        pass
    finally:
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
