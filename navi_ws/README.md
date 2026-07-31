# navi_ws — 导航融合工作空间

IMU + 轮式里程计 EKF 融合，输出更准的定位。

## 数据流

```
/imu/data_raw (20Hz) ──┐
                        ├──→ EKF → /odom_combined (30Hz) + tf
/odom (20Hz) ──────────┘
```

## 工作空间结构

```
navi_ws/
├── src/
│   └── navi_bringup/
│       ├── launch/
│       │   └── ekf_fusion.launch.py
│       └── config/
│           ├── ekf.yaml         ← EKF 参数 (融合矩阵)
│           └── imu_filter.yaml  ← IMU Madgwick 滤波参数
├── build/
├── install/
└── log/
```

## 编译

```bash
cd /userdata/navi_ws
source /opt/tros/humble/setup.bash
source /userdata/dev_ws/install/setup.bash
colcon build --symlink-install
```

## 启动

```bash
# 底盘先跑
ros2 launch origincar_bringup stm32_ackermann.launch.py

# 再启 EKF
source /userdata/navi_ws/install/setup.bash
ros2 launch navi_bringup ekf_fusion.launch.py
```

## 话题

| 话题 | 方向 | 说明 |
|------|------|------|
| `/imu/data_raw` | 输入 | IMU (底盘自动发布) |
| `/odom` | 输入 | 轮式里程计 (底盘自动发布) |
| `/odom_combined` | 输出 | EKF 融合里程计 |
| `/tf` | 输出 | `odom_combined → base_footprint` |

## 验证

```bash
ros2 topic echo /odom_combined
ros2 topic hz /odom_combined
```
