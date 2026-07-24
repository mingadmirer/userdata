#!/usr/bin/env python3
"""USB 1080p 全图 QR 码扫描测试"""

import time
import cv2

cap = cv2.VideoCapture(0)
cap.set(cv2.CAP_PROP_FOURCC, cv2.VideoWriter_fourcc(*'MJPG'))
cap.set(cv2.CAP_PROP_FRAME_WIDTH, 1920)
cap.set(cv2.CAP_PROP_FRAME_HEIGHT, 1080)
cap.set(cv2.CAP_PROP_FPS, 30)

detector = cv2.QRCodeDetector()
last = None
count = 0

print(f'Camera: {cap.get(cv2.CAP_PROP_FRAME_WIDTH):.0f}x'
      f'{cap.get(cv2.CAP_PROP_FRAME_HEIGHT):.0f} '
      f'@{cap.get(cv2.CAP_PROP_FPS):.0f}fps')
print('Scanning QR... (Ctrl+C to stop)\n')

while True:
    ok, frame = cap.read()
    if not ok:
        continue

    # 缩到 1/4 像素再扫，速度提升 4x+
    h, w = frame.shape[:2]
    small = cv2.resize(frame, (w // 2, h // 2))

    t0 = time.time()
    data, points, _ = detector.detectAndDecode(small)
    t1 = time.time()

    count += 1
    if data and data != last:
        last = data
        print(f'>>> {data}')
    elif not data:
        last = None

    if count % 10 == 0:
        print(f'  scan #{count}  {1/(t1-t0):.0f}fps  {"hit" if data else ""}')
