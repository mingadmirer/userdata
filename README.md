# OriginBot 智能小车 — 地平线 RDK X5

## 硬件

| 部件 | 型号 | 接口 |
|------|------|------|
| 主控 | 地平线 RDK X5 (arm64) | — |
| BPU | 内置 BPU (hobot-dnn 3.0.1, hbrt 3.15.54) | — |
| 下位机 | STM32 底盘 | `/dev/ttyACM0` (115200) |
| USB 相机 | USB 2.0 Camera | `/dev/video0` (MJPEG 640×480@30fps) |
| 深度相机 | Deptrum Aurora930 | USB (libusb) |
| 激光雷达 | LSLiDAR N10 串口版 | `/dev/wheeltec_lidar` |

## 目录结构

```
/userdata/
├── dev_ws/               ← 底盘驱动与控制
├── BPU_bl_ws/            ← BPU 推理 (YOLO/FCOS + QR)
├── driver_ws/            ← 传感器驱动 (深度相机 + 雷达)
└── yolov5s_v2.0_config/  ← 自定义 4 类模型 (line/p/qrcode/zt)
```

## 工作空间总览

### dev_ws — 底盘驱动

```bash
cd /userdata/dev_ws
ros2 launch origincar_base origincar_bringup.launch.py   # 启动底盘
ros2 run teleop_twist_keyboard teleop_twist_keyboard      # 键盘控制
```

| 包名 | 说明 |
|------|------|
| `origincar_base` | STM32 串口通信、里程计、IMU |
| `origincar_bringup` | 启动文件集合 |
| `origincar_description` | URDF 模型 |
| `origincar_msg` | 自定义消息 |
| `line_follower_pkg` | 巡线 (OpenCV) |
| `utils` | 图像传输工具 |

### BPU_bl_ws — 视觉推理

```bash
cd /userdata/BPU_bl_ws
source install/setup.bash

# 默认 FCOS 80 类检测
ros2 launch bpu_bringup usb_camera_yolo.launch.py

# 4 类障碍物检测 + QR
ros2 launch bpu_bringup usb_camera_yolo.launch.py \
    dnn_example_config_file:=config/custom_4/model.json

# QR 裁切 + 解码 (需先启动 DNN)
ros2 launch qr_decoder qr_pipeline.launch.py
```

| 模型 | 配置 | 类别 |
|------|------|------|
| FCOS 512×512 | `config/fcosworkconfig.json` (默认) | COCO 80 |
| YOLOv5x 672×672 | `config/coco_80/model.json` | COCO 80 |
| YOLOv5s v2.0 | `config/custom_4/model.json` | line / p / qrcode / zt |

浏览器查看：`http://<IP>:8000` → web展示端，检测框带置信度。

| 话题 | 类型 | 说明 |
|------|------|------|
| `hobot_dnn_detection` | `ai_msgs/PerceptionTargets` | BPU 原始检测结果 |
| `/detection_result` | `detection_interfaces/DetectionArray` | 标准化检测消息 |
| `/qr_crop` | `sensor_msgs/Image` | QR 码裁切图 |
| `/qr_result` | `std_msgs/String` | QR 解码文本 |

### driver_ws — 传感器驱动

```bash
cd /userdata/driver_ws
source install/setup.bash

# 深度相机
ros2 launch deptrum-ros-driver-aurora930 aurora930_launch.py

# 激光雷达
ros2 launch lslidar_driver lsn10_launch.py
```

| 话题 | 说明 |
|------|------|
| `/aurora/rgb/image_raw` | RGB 彩色图 640×400@15fps |
| `/aurora/depth/image_raw` | 深度图 16bit |
| `/aurora/points2` | 深度点云 |
| `/scan` | 2D 激光扫描 |
| `lslidar_point_cloud` | 3D 雷达点云 |

## 数据管道

```
┌─────────────────────────────────────────────────────┐
│ USB 相机 (/dev/video0)                              │
│   │ MJPEG 640×480                                   │
│   ├──→ hobot_codec_decode → /hbmem_img (NV12)       │
│   │         │                                       │
│   │         ├──→ dnn_node (BPU) → hobot_dnn_detection│
│   │         │         │                             │
│   │         │         ├──→ detection_bridge → /detection_result│
│   │         │         ├──→ websocket → 浏览器        │
│   │         │         └──→ qr_cropper → /qr_crop    │
│   │         │                   │                   │
│   │         │                   └──→ qr_decode      │
│   │         │                         → /qr_result  │
│   │         │                                       │
│   └──→ (原始 MJPEG /image, QR 裁切用)               │
├─────────────────────────────────────────────────────┤
│ STM32 底盘 ←→ /dev/ttyACM0 ←→ origincar_base       │
├─────────────────────────────────────────────────────┤
│ Aurora930 深度相机 → /aurora/depth/...               │
│ LSLiDAR N10 雷达   → /scan                          │
└─────────────────────────────────────────────────────┘
```

## 环境

所有工作空间共用 TogetheROS (地平线 ROS2 Humble 发行版)：

```bash
source /opt/tros/humble/setup.bash
```

编译时根据工作空间选择：

```bash
# dev_ws — 标准 ROS2
colcon build --symlink-install

# BPU_bl_ws — 标准 ROS2
colcon build --symlink-install

# driver_ws — 需指定深度相机型号
colcon build --symlink-install --cmake-args -DSTREAM_SDK_TYPE=AURORA930
```
