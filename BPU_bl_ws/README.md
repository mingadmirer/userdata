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
│   ├── qr_decoder/               ← QR 码裁切 + 解码
│   │   ├── launch/
│   │   │   └── qr_pipeline.launch.py       ← 一键启动裁切+解码
│   │   └── qr_decoder/
│   │       ├── qr_decoder_node.py          ← DNN bbox → 裁切 /qr_crop
│   │       └── qr_decode_node.py           ← 矫正 + 解码 → /qr_result
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

## QR 码检测

当使用 4 类模型（含 `qrcode`）时，可以启动 QR 码裁切和解码节点：

```bash
# 终端1：DNN 推理（默认用4类模型）
ros2 launch bpu_bringup usb_camera_yolo.launch.py \
    dnn_example_config_file:=config/custom_4/model.json

# 终端2：QR 裁切 + 解码
ros2 launch qr_decoder qr_pipeline.launch.py

# 终端3：查看解码结果
ros2 topic echo /qr_result
```

### 解码流程

```
/qr_crop 图像 (DNN bbox裁切)
  │
  ├─① CLAHE 增强对比度 → detectAndDecode 直接解
  ├─② 失败 → detect() 拿4角点 → warpPerspective 拉正 → decode
  └─③ 还失败 → 增强图也透视矫正 → 再 decode
  │
  └─→ /qr_result (std_msgs/String) + 终端日志
```

| 话题 | 类型 | 说明 |
|------|------|------|
| `/qr_crop` | `sensor_msgs/Image` | DNN 检测 qrcode 后裁切的码图 |
| `/qr_result` | `std_msgs/String` | 解码出的文本内容 |

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

  │ (qrcode 检测到后)
  ▼
qr_cropper → /qr_crop (裁切图)
  │
  ▼
qr_decode → /qr_result (解码文本)
```
