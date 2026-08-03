# control_ws — 状态机任务控制器

基于状态机的 OriginBot 任务调度系统。所有 task 继承 `BaseTask`，统一生命周期管理。

## 架构

```
MainController (task_id 选 task)
  │
  └── BaseTask (IDLE → RUNNING → COMPLETED/FAILED)
        │
        ├── Task1LineFollow   ← 巡线 + QR 解码停止
        ├── Task2 (待扩展)
        └── Task3 (待扩展)
```

**状态流转**: `IDLE` → `start()` → `RUNNING` → `check_done()==True` → `stop()` → `COMPLETED`

**基类接口** (`state_machine.py`):

| 方法 | 说明 |
|------|------|
| `on_enter()` | 进入任务，初始化订阅/定时器 |
| `on_update()` | 每周期由主循环调用（可选） |
| `on_exit()` | 退出任务，清理资源 |
| `check_done()` | 返回 True 表示任务目标达成 |

## 目录

```
control_ws/
└── src/task_controller/
    ├── package.xml
    ├── setup.py
    ├── resource/
    │   └── task_controller           # ament index 标记
    ├── scripts/
    │   └── main_controller_node      # 可执行入口
    ├── launch/
    │   └── task1_launch.py
    └── task_controller/
        ├── __init__.py
        ├── state_machine.py          # 状态机基类
        ├── task1_line_follow.py      # Task1: 巡线+QR
        └── main_controller.py        # 主调度入口
```

## 编译

```bash
cd /userdata/control_ws
source /opt/tros/humble/setup.bash
colcon build --symlink-install
source install/setup.bash
```

## 运行

```bash
# Task1 — 巡线 (默认)
ros2 launch task_controller task1_launch.py

# 等价于
ros2 run task_controller main_controller_node --ros-args -p task_id:=1
```

### Task1 参数

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `task1.linear_speed` | 0.5 | 巡线前进速度 m/s |
| `task1.kp` | 2.0 | P 控制增益 |
| `task1.miss_timeout` | 1.0 | 丢线超时停止秒 |
| `task1.timer_period` | 0.1 | 控制周期秒 |

可通过 launch 文件或命令行覆盖：

```bash
ros2 launch task_controller task1_launch.py \
    task1.linear_speed:=0.3 \
    task1.kp:=1.5
```

## Task1 — 巡线 + QR 解码停止

### 数据流

```
/detection_result (DetectionArray)
  │  只取 "line" 类 → Y最大目标中心点 → P控制 → cmd_vel
  │
/qr_result (String, 来自 qr_decode 解码节点)
  │  非空 → 停车 → 打印解码结果 → 任务完成
```

### 前置依赖

Task1 依赖以下节点已在运行：

| 节点 | 包 | 说明 |
|------|-----|------|
| `origincar_base_node` | origincar_base | 底盘驱动，订阅 `cmd_vel` |
| `usb_camera_yolo.launch.py` | bpu_bringup | DNN 检测 + 桥接 → `/detection_result` |
| `qr_pipeline.launch.py` | qr_decoder | QR 裁切 + 解码 → `/qr_result` |

启动顺序（新终端中分别执行）：

```bash
# 1. 底盘
cd /userdata/dev_ws && source install/setup.bash
ros2 launch origincar_base origincar_bringup.launch.py

# 2. 视觉（DNN + QR 管线）
cd /userdata/BPU_bl_ws && source install/setup.bash
ros2 launch bpu_bringup usb_camera_yolo.launch.py \
    dnn_example_config_file:=config/custom_4/model.json
ros2 launch qr_decoder qr_pipeline.launch.py

# 3. 任务控制
cd /userdata/control_ws && source install/setup.bash
ros2 launch task_controller task1_launch.py
```

### 运行过程

1. 小车巡线前进（P 控制跟随 line 目标）
2. BPU 检测到 qrcode → qr_cropper 裁切 → qr_decode 3轮解码
3. `/qr_result` 输出解码字符串 → Task1 收到 → 停车 → 打印结果 → 退出

日志示例：

```
[task1_line_follow] 巡线启动 — speed=0.5 m/s, kp=2.0, timeout=1.0s
[task1_line_follow] 等待 /qr_result 解码输出...
...
[task1_line_follow] >>> /qr_result 解码成功: "123456"
[task1_line_follow] >>> ========== QR 解码结果: 123456 ==========
[main_controller] 任务[task1_line_follow] 执行完成!
```

## 扩展 Task

在 `main_controller.py` 的 `TASK_REGISTRY` 中注册：

```python
TASK_REGISTRY = {
    1: Task1LineFollow,
    2: Task2Xxx,      # 新增
}
```

新 task 继承 `BaseTask`，实现四个方法即可：

```python
from task_controller.state_machine import BaseTask

class Task2Xxx(BaseTask):
    def on_enter(self):
        ...  # 初始化

    def on_exit(self):
        ...  # 清理

    def check_done(self) -> bool:
        ...  # 完成条件
```
