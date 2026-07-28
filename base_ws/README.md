# base_ws — 底盘驱动工作空间

阿克曼底盘 STM32 串口驱动，拆分为两种启动模式。

## 两种模式

| 启动文件 | 模式 | ROS 行为 | STM32 固件要求 |
|---------|------|---------|--------------|
| `stm32_ackermann.launch.py` | STM32 阿克曼 | ROS 发 Twist，STM32 用 `Vz_to_Akm_Angle` 自转转向角 | **Akm_Car**（当前固件） |
| `ros_ackermann.launch.py` | ROS 阿克曼 | ROS 端 `cmd_vel_to_ackermann_drive` 转转向角再发 | 非 Akm_Car（如 Diff_Car） |

## 工作空间结构

```
base_ws/
├── src/
│   └── origincar_bringup/
│       └── launch/
│           ├── stm32_ackermann.launch.py    ← 推荐：STM32 做阿克曼
│           └── ros_ackermann.launch.py      ← ROS 做阿克曼
├── build/
├── install/
└── log/
```

## 编译

```bash
cd /userdata/base_ws
source /opt/tros/humble/setup.bash
source /userdata/dev_ws/install/setup.bash   # 依赖 origincar_base
colcon build --symlink-install
```

## 启动

```bash
source /userdata/base_ws/install/setup.bash

# STM32 固件阿克曼模式（当前使用）
ros2 launch origincar_bringup stm32_ackermann.launch.py

# ROS 端阿克曼模式
ros2 launch origincar_bringup ros_ackermann.launch.py
```

### stm32_ackermann.launch.py 详解

```
启动节点: origincar_base_node
参数: akm_cmd_vel='none'
订阅话题: /cmd_vel (geometry_msgs/Twist)
串口发送: tx[3-4]=linear.x*1000, tx[5-6]=linear.y*1000, tx[7-8]=angular.z*1000
STM32接收: Vz→Vz_to_Akm_Angle(Vx,Vz)→舵机转向角
```

**发指令** — 直接 pub Twist：

```bash
# 前进 0.5m/s
ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist \
    "{linear: {x: 0.5}, angular: {z: 0.0}}"

# 前进 0.5m/s + 左转 0.3rad/s
ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist \
    "{linear: {x: 0.5}, angular: {z: 0.3}}"

# 停车
ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist \
    "{linear: {x: 0.0}, angular: {z: 0.0}}"
```

```python
# 代码调用
from geometry_msgs.msg import Twist
msg = Twist()
msg.linear.x = 0.5       # 前进速度 m/s
msg.angular.z = 0.3      # 角速度 rad/s (正=左转)
pub.publish(msg)
```

### ros_ackermann.launch.py 详解

```
启动节点: origincar_base_node + cmd_vel_to_ackermann_drive
参数: akm_cmd_vel='ackermann_cmd'
cmd_vel_to_ackermann_drive: 订阅 /cmd_vel(Twist)→ 发布 /ackermann_cmd(AckermannDriveStamped)
origincar_base_node: 订阅 /ackermann_cmd→ 串口发送 steering_angle*1000/2
```

**发指令** — 两种方式都行：

```bash
# 方式1: 发 Twist (cmd_vel_to_ackermann_drive 自动转换)
ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist \
    "{linear: {x: 0.5}, angular: {z: 0.3}}"

# 方式2: 直接发阿克曼指令
ros2 topic pub --once /ackermann_cmd ackermann_msgs/msg/AckermannDriveStamped \
    "{drive: {speed: 0.5, steering_angle: 0.2}}"
```

```python
# 代码调用 — 直发阿克曼
from ackermann_msgs.msg import AckermannDriveStamped
msg = AckermannDriveStamped()
msg.drive.speed = 0.5              # m/s
msg.drive.steering_angle = 0.2     # rad (正=左转)
pub.publish(msg)
```

### 该用哪个

```
底盘电位器 = Akm_Car → stm32_ackermann.launch.py (ROS 发 Twist)
底盘电位器 = Diff_Car 等 → ros_ackermann.launch.py (ROS 做转换)

串口协议不变(0x7B...0x7D, 11字节), 区别只是谁算 atan 公式。
```

## 巡线 Demo

```bash
# 先启动 DNN 推理
ros2 launch bpu_bringup usb_camera_yolo.launch.py \
    dnn_example_config_file:=config/custom_4/model.json

# 再启动巡线
python3 /userdata/line_follower_demo.py
```

调参位置：`line_follower_demo.py` 中的 `msg.linear.x`（速度）和 `msg.angular.z`（转向 P 值）。

## 测试

### 串口直驱

```bash
python3 /userdata/serial_test.py
```
绕过 ROS 直接发串口帧，测试 STM32 硬件响应。

按键：
- `w/s` 前进/后退 ±0.1 m/s
- `a/d` 左转/右转 ±0.2 rad/s
- `空格` 回正
- `q` 退出

### 阿克曼指令测试

```bash
ros2 run origincar_base akm_test.py
```
50Hz 直发 AckermannDriveStamped 到 `/ackermann_cmd`，测试 ROS→串口链路。

按键：
- `w/s` 前进/后退 ±0.1 m/s
- `a/d` 左转/右转 ±0.1 rad（转向角）
- `1`~`5` 预设速度 0.1~1.0 m/s
- `空格` 紧急停车
- `q` 退出

### 键盘遥操

```bash
ros2 run teleop_twist_keyboard teleop_twist_keyboard
```

```
   u    i    o
   j    k    l
   m    ,    .
```
i=前进, k=后退, j/l=左右转。默认速度 0.5 m/s，加参数调：`--ros-args -p speed:=1.0`。

## 自启动

服务文件：`/etc/systemd/system/origincar-base.service`

```bash
systemctl status origincar-base     # 查看状态
systemctl stop origincar-base       # 停止
systemctl start origincar-base      # 启动
journalctl -u origincar-base -f     # 实时日志
```
