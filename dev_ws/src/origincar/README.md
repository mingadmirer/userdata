# Origincar 使用说明

## 系统架构

```
┌──────────────────────────────────────────────────┐
│  上位机 (Horizon RDK) — ROS2 Humble               │
│                                                   │
│  /cmd_vel (Twist)                                 │
│      │                                            │
│      ▼                                            │
│  cmd_vel_to_ackermann_drive.py  ← 仅 akmcar:=true │
│      │                                            │
│      ▼ /ackermann_cmd (AckermannDriveStamped)     │
│  origincar_base_node                               │
│      │ 串口 /dev/ttyACM0, 115200bps               │
└──────┼────────────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────────┐
│  下位机 — STM32F407 + FreeRTOS                    │
│  阿克曼运动学逆解 + PI 速度闭环 + 舵机控制          │
└──────────────────────────────────────────────────┘
```

## 通信协议

| 项目 | 值 |
|------|-----|
| 物理层 | 串口 `/dev/ttyACM0`, 115200bps, 8N1 |
| 帧头 | `0x7B` |
| 帧尾 | `0x7D` |
| 上行帧 (ROS2→STM32) | 11 字节: `[帧头] [预留2B] [Vx int16, mm/s] [Vy int16] [Vz int16] [XOR校验] [帧尾]` |
| 下行帧 (STM32→ROS2) | 24 字节: `[帧头] [故障标志] [三轴速度] [IMU 6轴] [电池电压] [XOR校验] [帧尾]` |
| 阿克曼模式 Vz 含义 | steering_angle × 500 (ROS2 侧 `*1000/2`, STM32 侧直接用作前轮转角) |

## 话题接口

| 话题 | 类型 | 方向 | 说明 |
|------|------|------|------|
| `/ackermann_cmd` | `AckermannDriveStamped` | 订阅 | 阿克曼控制指令 (speed m/s + steering_angle rad) |
| `/cmd_vel` | `Twist` | 订阅 | 通用速度指令, 经转换节点转成 Ackermann |
| `/odom` | `Odometry` | 发布 | 里程计 |
| `/imu/data_raw` | `Imu` | 发布 | MPU6050 原始数据 |
| `/PowerVoltage` | `Float32` | 发布 | 电池电压 |

## 底盘参数 (阿克曼模式)

| 参数 | 值 |
|------|-----|
| 轮距 (track width) | 0.162m |
| 轴距 (wheelbase) | 0.144m |
| 最小转弯半径 | 0.350m |
| 车轮直径 | 65mm |
| 减速比 | 30:1 |
| 编码器 | 13线霍尔 × 4倍频 = 1560脉冲/圈 |
| 软件极速 | ±3.5 m/s |
| 舵机中位 PWM | 1500 |
| 舵机范围 | 800~2200 |

## 控制行为

- **状态机模式**: 发送一次指令后保持状态不变，收到新指令覆盖。下位机无超时看门狗，断联不会自动停车。
- 开机后 10 秒内不响应控制指令（安全延时）。
- 电池电压 < 10V 或使能开关 OFF 时强制停车。

## 编译

```bash
cd /userdata/dev_ws
colcon build --symlink-install --packages-select origincar_base origincar_msg
```

## 启动底盘

### 阿克曼模式 (推荐)

```bash
source /userdata/dev_ws/install/setup.bash
ros2 launch origincar_base base_serial.launch.py akmcar:=true
```

启动后两个节点:
- `origincar_base_node` — 串口桥接, 订阅 `/ackermann_cmd`, 发布 `/odom` `/imu/data_raw` `/PowerVoltage`
- `cmd_vel_to_ackermann_drive.py` — Twist→Ackermann 转换, 可选

### 普通模式 (差速/全向轮)

```bash
ros2 launch origincar_base base_serial.launch.py akmcar:=false
```

仅启动 `origincar_base_node`, 订阅 `/cmd_vel` (Twist), Z 轴传角速度给下位机自己解算。

## 测试脚本

### 交互式测试

```bash
ros2 run origincar_base akm_test.py
```

| 按键 | 功能 |
|------|------|
| `w` / `s` | 前进 / 后退 (±0.1 m/s 步进) |
| `a` / `d` | 左转 / 右转 (±0.1 rad 步进) |
| `空格` | 紧急停止 |
| `r` | 回正舵机 |
| `1`~`5` | 预设速度 0.1/0.2/0.3/0.5/1.0 m/s |
| `q` | 退出 (自动停车) |

### 极速测试

```bash
ros2 run origincar_base akm_maxspeed.py [速度m/s]
# 例: ros2 run origincar_base akm_maxspeed.py 3.0
# Ctrl+C 停车退出
```

### 直接发包 (程序调用)

```python
from ackermann_msgs.msg import AckermannDriveStamped

msg = AckermannDriveStamped()
msg.drive.speed = 1.0           # m/s
msg.drive.steering_angle = 0.3  # rad, 正=左转
pub.publish(msg)  # → /ackermann_cmd → 串口 → STM32
```

## 自启动 (开机自动运行)

已配置 systemd 服务, 开机后自动启动阿克曼模式:

```bash
# 查看状态
systemctl status origincar-base

# 手动启停
systemctl stop origincar-base
systemctl start origincar-base

# 查看日志
journalctl -u origincar-base -f

# 禁用自启动
systemctl disable origincar-base
```

服务文件: `/etc/systemd/system/origincar-base.service`

启动流程: 等待 `/dev/ttyACM0` 就绪 → source ROS2 环境 → `ros2 launch origincar_base base_serial.launch.py akmcar:=true`。退出时自动向底盘发送零速停车帧。

## 键盘控制 (手柄替代)

```bash
ros2 run teleop_twist_keyboard teleop_twist_keyboard
```

```
   u    i    o
   j    k    l
   m    ,    .
```

发布 `/cmd_vel`，由 `cmd_vel_to_ackermann_drive.py` 转换为阿克曼指令。

## USB 相机驱动与图像可视化

```bash
ros2 launch origincar_bringup usb_websocket_display.launch.py
```

同一网络 PC 浏览器访问 `http://<IP>:8000` → "web展示端"。

## 深度相机 (Aurora930)

```bash
ros2 launch deptrum-ros-driver-aurora930 aurora930_launch.py
ros2 run rqt_image_view rqt_image_view
```
