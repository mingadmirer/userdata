#!/usr/bin/env python3
"""直接串口帧测试阿克曼底盘 — 绕过 ROS"""

import struct
import serial
import time

PORT = "/dev/ttyACM0"
BAUD = 115200

FRAME_HEADER = 0x7B
FRAME_TAIL = 0x7D

ser = serial.Serial(PORT, BAUD, timeout=1)

def checksum(data):
    c = 0
    for b in data:
        c ^= b
    return c

def send_akm(speed, angular_z):
    """speed: m/s, angular_z: rad/s — STM32 的 Vz_to_Akm_Angle 会自动转转向角"""
    sp = int(speed * 1000)
    az = int(angular_z * 1000)
    frame = bytearray([
        FRAME_HEADER,
        0, 0,
        (sp >> 8) & 0xFF, sp & 0xFF,    # tx[3-4] speed
        0, 0,                            # tx[5-6] 清零
        (az >> 8) & 0xFF, az & 0xFF,    # tx[7-8] angular_z → STM32 自转转向角
    ])
    # checksum over tx[0..8] = 9 bytes
    frame.append(checksum(frame[:9]))
    frame.append(FRAME_TAIL)
    ser.write(frame)
    print(f"speed={speed:+.2f} angular_z={angular_z:+.3f}  raw={frame.hex()}")

print("Ackermann 串口直驱测试 (STM32 Akm_Car 模式)")
print("  w/s  前进/后退 ±0.1 m/s")
print("  a/d  左转/右转  ±0.2 rad/s (角速度)")
print("  r    回正")
print("  空格  停车")
print("  q    退出\n")

import termios, tty, sys, select

speed = 0.3      # 必须有速度，Vz_to_Akm_Angle 在 Vx=0 时返回 0
angular_z = 0.0

def getkey():
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setraw(fd)
        if select.select([sys.stdin], [], [], 0.05)[0]:
            return sys.stdin.read(1)
        return ""
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)

try:
    while True:
        send_akm(speed, angular_z)
        key = getkey()
        if key == 'w': speed += 0.1
        elif key == 's': speed -= 0.1
        elif key == 'a': angular_z += 0.2
        elif key == 'd': angular_z -= 0.2
        elif key == 'r': angular_z = 0.0
        elif key == ' ':
            speed = 0.3; angular_z = 0.0   # 保持低速(舵机需要非零Vx才能转)
            send_akm(0.3, 0)
            print("回正")
        elif key == 'q': break
        time.sleep(0.02)
finally:
    send_akm(0, 0)
    ser.close()
    print("done")
