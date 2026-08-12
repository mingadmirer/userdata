#!/usr/bin/env python3
"""测试 pyzbar 对裁切图的解码能力"""

import cv2
import numpy as np
from pyzbar import pyzbar

img = cv2.imread('/userdata/qr_crop.png')
if img is None:
    print('未找到 /userdata/qr_crop.png, 请先运行 save_qr_crop.py 采集')
    exit(1)

h, w = img.shape[:2]
print(f'裁切图尺寸: {w}x{h}')

# 试多种预处理
tests = [
    ('原始BGR', img),
    ('灰度', cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)),
    ('放大2倍', cv2.resize(img, (w*2, h*2))),
    ('放大4倍', cv2.resize(img, (w*4, h*4))),
    ('CLAHE', cv2.createCLAHE(2.0, (8,8)).apply(cv2.cvtColor(img, cv2.COLOR_BGR2GRAY))),
    ('CLAHE+放大2倍', cv2.resize(
        cv2.createCLAHE(2.0, (8,8)).apply(cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)),
        (w*2, h*2))),
    ('二值化', cv2.adaptiveThreshold(
        cv2.cvtColor(img, cv2.COLOR_BGR2GRAY), 255,
        cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 15, 2)),
]

for name, test_img in tests:
    results = pyzbar.decode(test_img)
    if results:
        print(f'✅ {name}: {results[0].data.decode()}')
    else:
        print(f'❌ {name}')

# 也试 OpenCV 对比
print('\n--- OpenCV 对比 ---')
detector = cv2.QRCodeDetector()
data, _, _ = detector.detectAndDecode(img)
print(f'原始: {data or "无"}')
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
clahe = cv2.createCLAHE(2.0, (8,8)).apply(gray)
data, _, _ = detector.detectAndDecode(clahe)
print(f'CLAHE: {data or "无"}')
