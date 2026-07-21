# BPU_bl_ws — 地平线 RDK X5 BPU 推理工作空间

基于地平线 RDK X5 的智能小车视觉检测管道：USB 相机 → MJPEG 解码 → BPU DNN 推理 → WebSocket 推流展示。

## 硬件平台

- **板卡**: 地平线 RDK X5 (arm64)
- **BPU 运行时**: `hobot-dnn 3.0.1` / `hbrt 3.15.54`
- **USB 相机**: `/dev/video0` (MJPEG 640×480@30fps)
- **下位机串口**: `/dev/ttyACM0` (CH343, 115200bps)

## 工作空间结构

```
BPU_bl_ws/
├── src/
│   ├── bpu_bringup/              ← 启动与配置包
│   │   ├── launch/
│   │   │   └── usb_camera_yolo.launch.py   ← 主启动文件
│   │   └── config/
│   │       ├── coco_80/                    ← COCO 80 类配置
│   │       │   └── model.json
│   │       └── custom_4/                   ← 4 类障碍物检测配置
│   │           ├── model.json
│   │           └── custom_4class.list
│   └── utils/                   ← symlink → ../dev_ws/src/origincar/utils
├── build/
├── install/
└── log/
```

## 可用模型

| 模型 | 输入尺寸 | 类别 | 配置文件 |
|------|---------|------|---------|
| FCOS EfficientNetB0 | 512×512 | COCO 80 | `config/fcosworkconfig.json` (默认) |
| YOLOv5x | 672×672 | COCO 80 | `config/coco_80/model.json` |
| YOLOv5s v2.0 | 672×672 | line / p / qrcode / zt | `config/custom_4/model.json` |

## 编译

```bash
cd /userdata/BPU_bl_ws
source /opt/tros/humble/setup.bash
colcon build --symlink-install
```

## 启动

```bash
cd /userdata/BPU_bl_ws
source install/setup.bash

# FCOS 80类检测（默认，最快）
ros2 launch bpu_bringup usb_camera_yolo.launch.py

# YOLOv5x 80类检测
ros2 launch bpu_bringup usb_camera_yolo.launch.py \
    dnn_example_config_file:=config/coco_80/model.json

# 4类障碍物检测（line / p / qrcode / zt）
ros2 launch bpu_bringup usb_camera_yolo.launch.py \
    dnn_example_config_file:=config/custom_4/model.json
```

## 查看结果

同一网络 PC 浏览器打开 `http://<RDK_X5_IP>:8000`，选择 **web展示端** 即可看到实时画面与检测框。

检测框格式：`类别名(置信度)`，如 `line(0.923)`。左侧面板可控制各类可视化开关。

## 添加自己的模型

1. 将 RTM `.bin` 模型文件放到 `/opt/hobot/model/x5/custom/`
2. 在 `src/bpu_bringup/config/` 下新建子目录，包含：
   - `model.json` — 模型配置（参考现有文件）
   - `<name>.list` — 类别标签（一行一个，顺序与训练时一致）
3. `colcon build --symlink-install --packages-select bpu_bringup`
4. 启动时指定 `dnn_example_config_file:=config/<你的目录>/model.json`

### model.json 关键字段

| 字段 | 说明 |
|------|------|
| `model_file` | RTM 模型绝对路径 |
| `dnn_Parser` | 解析器类型：`fcos` / `yolov5` / `yolov5x` / `yolov8` / `yolov10` |
| `class_num` | 类别数量 |
| `cls_names_list` | 类别标签文件（相对路径，基于启动目录下的 `config/`） |
| `strides` | 检测头下采样倍率 |
| `anchors_table` | 先验框（仅 YOLO 系列需要） |
| `score_threshold` | 置信度阈值 |
| `nms_threshold` | NMS 阈值 |

## 管道数据流

```
USB相机 (/dev/video0)
  │ MJPEG 640×480
  ▼
hobot_usb_cam → /image
  │
  ▼
hobot_codec_decode → /hbmem_img (NV12, 共享内存)
  │
  ▼
dnn_node_example → /hobot_dnn_detection (BPU 推理 + 检测结果)
  │
  ▼
websocket → 浏览器 (MJPEG + AI 检测数据 WebSocket 推流)
```
