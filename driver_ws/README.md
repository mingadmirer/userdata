# driver_ws — 深度相机 & 激光雷达驱动工作空间

RDK X5 智能小车的传感器驱动：Deptrum Aurora930 深度相机 + LSLiDAR N10 激光雷达。

## 硬件

| 传感器 | 型号 | 接口 | 设备路径 |
|--------|------|------|---------|
| 深度相机 | Deptrum Aurora930 | USB | (自动识别) |
| 激光雷达 | LSLiDAR N10 串口版 | UART | `/dev/wheeltec_lidar` |

## 工作空间结构

```
driver_ws/
├── src/
│   ├── deptrum-ros-driver/          ← Aurora930 深度相机驱动
│   │   └── launch_aurora930/launch/
│   │       ├── aurora930_launch.py      ← 主启动文件
│   │       ├── viewer930_launch.py      ← RViz 可视化
│   │       └── sub_node_ci_aurora930_launch.py  ← 图像保存
│   ├── lslidar_driver/              ← LSLiDAR 雷达驱动
│   │   ├── launch/
│   │   │   └── lsn10_launch.py          ← N10 串口版启动
│   │   └── config/
│   │       └── lslidar_n10_uart.yaml    ← N10 参数配置
│   └── lslidar_msgs/                ← 雷达消息接口
├── wheeltec_udev.sh                 ← 雷达串口别名脚本
├── build/
├── install/
└── log/
```

## 编译

```bash
cd /userdata/driver_ws
source /opt/tros/humble/setup.bash
colcon build --symlink-install --cmake-args -DSTREAM_SDK_TYPE=AURORA930
```

> 如果重新解压 deptrum 源码，需先创建 stream SDK 的 `.so` symlink：
> ```bash
> ln -s libdeptrum_stream_aurora900.so.1.1.22 \
>   driver_ws/src/deptrum-ros-driver/ext/deptrum-stream-aurora900-linux-aarch64-v1.1.22-18.04/lib/libdeptrum_stream_aurora900.so
> ```

---

# 一、深度相机 — Deptrum Aurora930

## 启动

```bash
source /opt/tros/humble/setup.bash
source /userdata/driver_ws/install/setup.bash
ros2 launch deptrum-ros-driver-aurora930 aurora930_launch.py
```

> 驱动默认 namespace 为 `aurora`，所有话题带 `/aurora` 前缀。

## 查看话题

```bash
source /userdata/driver_ws/install/setup.bash
ros2 topic list | grep aurora
```

常见输出：

| 话题 | 类型 | 说明 |
|------|------|------|
| `/aurora/rgb/image_raw` | `sensor_msgs/Image` | RGB 彩色图像 (bgr8) |
| `/aurora/rgb/camera_info` | `sensor_msgs/CameraInfo` | RGB 相机内参 |
| `/aurora/ir/image_raw` | `sensor_msgs/Image` | IR 红外图像 |
| `/aurora/ir/camera_info` | `sensor_msgs/CameraInfo` | IR/Depth 相机内参 |
| `/aurora/depth/image_raw` | `sensor_msgs/Image` | 深度图像 (16bit) |
| `/aurora/points2` | `sensor_msgs/PointCloud2` | 深度点云 |

查看帧率：

```bash
ros2 topic hz /aurora/depth/image_raw
```

## RViz 可视化

```bash
source /userdata/driver_ws/install/setup.bash
ros2 launch deptrum-ros-driver-aurora930 viewer930_launch.py
```

手动配置：Fixed Frame 设为 `depth_camera_link`，通过 By topic 添加图像和点云。

## 常用启动参数

```bash
# 仅 RGB + 深度，不发布点云
ros2 launch deptrum-ros-driver-aurora930 aurora930_launch.py \
    rgb_enable:=true ir_enable:=false depth_enable:=true point_cloud_enable:=false

# 限制最大深度 3 米
ros2 launch deptrum-ros-driver-aurora930 aurora930_launch.py \
    maximum_filter_depth_value:=3000
```

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `rgb_enable` | true | 是否发布 RGB 图像 |
| `ir_enable` | true | 是否发布 IR 图像 |
| `depth_enable` | true | 是否发布深度图像 |
| `point_cloud_enable` | true | 是否发布点云 |
| `ir_fps` | 15 | IR 帧率 |
| `rgb_fps` | 15 | RGB 帧率 |
| `depth_correction` | true | 深度修正 |
| `align_mode` | true | RGB/IR 对齐 |
| `minimum_filter_depth_value` | 150 | 最小深度 (mm) |
| `maximum_filter_depth_value` | 4000 | 最大深度 (mm) |

## 保存图像

```bash
# 方式一：直接运行
cd /userdata/driver_ws/install/deptrum-ros-driver-aurora930/lib/deptrum-ros-driver-aurora930
./sub_node

# 方式二：launch 启动
ros2 launch deptrum-ros-driver-aurora930 sub_node_ci_aurora930_launch.py
```

图像保存到 `/tmp/image`。

## Remap 话题适配

如果其他节点订阅默认相机话题名：

```bash
ros2 run your_package your_node --ros-args \
    -r /camera/color/image_raw:=/aurora/rgb/image_raw \
    -r /camera/depth/image_raw:=/aurora/depth/image_raw \
    -r /camera/color/camera_info:=/aurora/rgb/camera_info \
    -r /camera/depth/camera_info:=/aurora/ir/camera_info
```

---

# 二、激光雷达 — LSLiDAR N10 串口版

## 串口别名（已配置）

`wheeltec_udev.sh` 已执行，udev 规则写入 `/etc/udev/rules.d/`。雷达插入后自动创建 `/dev/wheeltec_lidar` 别名。

配置文件中使用的串口：

```yaml
# lslidar_driver/config/lslidar_n10_uart.yaml
serial_port: "/dev/wheeltec_lidar"
```

如果别名未生效，可改为实际串口如 `/dev/ttyUSB0`。

## 启动

```bash
source /userdata/driver_ws/install/setup.bash
ros2 launch lslidar_driver lsn10_launch.py
```

## 查看话题

```bash
source /userdata/driver_ws/install/setup.bash
ros2 topic list | grep -E "scan|lslidar"
```

| 话题 | 类型 | 说明 |
|------|------|------|
| `/scan` | `sensor_msgs/LaserScan` | 二维激光扫描，导航避障用 |
| `lslidar_point_cloud` | `sensor_msgs/PointCloud2` | 3D 点云 |

## RViz 可视化

```bash
# 方式一：使用自带配置
rviz2 -d /userdata/driver_ws/install/lslidar_driver/share/lslidar_driver/rviz/lslidar_x10.rviz

# 方式二：手动
rviz2
```

手动配置：Fixed Frame 设为 `laser`，By topic 添加 `/scan` → LaserScan。

## N10 核心参数

```yaml
# lslidar_n10_uart.yaml
serial_port: "/dev/wheeltec_lidar"
frame_id: "laser"
pointcloud_topic: "lslidar_point_cloud"
laserscan_topic: "/scan"
publish_scan: true
min_range: 0.15       # 最小有效距离 (m)
max_range: 50.0       # 最大有效距离 (m)
```

### 角度屏蔽

```yaml
# 屏蔽 0° ~ 90°
angle_disable_min: [0]
angle_disable_max: [9000]   # 单位 0.01°, 9000 = 90°

# 屏蔽两段
angle_disable_min: [0, 18000]
angle_disable_max: [9000, 27000]
```

## 发布静态 TF（雷达坐标系）

```bash
ros2 run tf2_ros static_transform_publisher 0 0 0.15 0 0 0 base_link laser
```

`0.15` 为雷达安装高度，按实际修改。

---

# 三、推荐启动流程

**终端 1 — 深度相机**

```bash
source /opt/tros/humble/setup.bash
source /userdata/driver_ws/install/setup.bash
ros2 launch deptrum-ros-driver-aurora930 aurora930_launch.py
```

**终端 2 — 激光雷达**

```bash
source /userdata/driver_ws/install/setup.bash
ros2 launch lslidar_driver lsn10_launch.py
```

**终端 3 — 查看话题**

```bash
source /userdata/driver_ws/install/setup.bash
ros2 topic list | grep -E "aurora|scan|lslidar"
ros2 topic hz /aurora/depth/image_raw
ros2 topic hz /scan
```
