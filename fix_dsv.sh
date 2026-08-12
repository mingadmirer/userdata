#!/bin/bash
# 修复脚本：定位并修复编码损坏的 .dsv 文件
# 在 10.159.132.24 上运行

set -e

echo "=== 步骤1：定位损坏的 .dsv 文件 ==="

BROKEN_FILE=""
for dir in /opt/ros/humble /home /root /workspace /userdata; do
  [ ! -d "$dir" ] && continue
  while IFS= read -r -d '' f; do
    if ! python3 -c "open('$f', 'r', encoding='utf-8').read()" 2>/dev/null; then
      BROKEN_FILE="$f"
      echo "找到损坏文件: $BROKEN_FILE"
      break 2
    fi
  done < <(find "$dir" -name "*.dsv" -type f -print0 2>/dev/null)
done

if [ -z "$BROKEN_FILE" ]; then
  echo "未找到损坏的 .dsv 文件，尝试其他诊断方式..."

  # 也可能是 setup*.sh 或其他配置脚本本身有问题
  echo "检查 /opt/ros/humble/setup.sh 的 main 函数入口..."

  # 运行 Python 调试找出问题文件
  python3 -c "
import sys
sys.path.insert(0, '/opt/ros/humble')
from _local_setup_util import get_commands
import os

# 遍历所有包
share_dir = '/opt/ros/humble/share'
if os.path.isdir(share_dir):
    for pkg in sorted(os.listdir(share_dir)):
        pkg_path = os.path.join(share_dir, pkg)
        if os.path.isdir(pkg_path):
            try:
                get_commands(pkg, '/opt/ros/humble', 'sh', 'bash')
            except UnicodeDecodeError as e:
                print(f'ERROR in package: {pkg}')
                print(f'  {e}')
                # 找出具体的 dsv 文件
                for root, dirs, files in os.walk(pkg_path):
                    for fn in files:
                        if fn.endswith('.dsv'):
                            fp = os.path.join(root, fn)
                            try:
                                with open(fp, 'r', encoding='utf-8') as fh:
                                    fh.read()
                            except UnicodeDecodeError as e2:
                                print(f'  CORRUPT FILE: {fp}')
                                print(f'    {e2}')
                                # 显示原始内容
                                with open(fp, 'rb') as fh:
                                    raw = fh.read()
                                print(f'    Raw hex: {raw[:50].hex(\" \")}')
                                try:
                                    print(f'    GBK decode: {raw.decode(\"gbk\")[:100]}')
                                except:
                                    pass
                print()
"
fi

echo ""
echo "=== 步骤2：修复方案 ==="

if [ -n "$BROKEN_FILE" ]; then
  echo "损坏文件: $BROKEN_FILE"

  # 方案1：删除并重新构建该包
  DIR=$(dirname "$BROKEN_FILE")
  PKG_NAME=$(basename "$DIR" 2>/dev/null)
  echo ""
  echo "建议操作："
  echo "  1. 方案A（推荐）：重新构建该包"
  echo "     cd <workspace> && rm -rf build/$PKG_NAME install/$PKG_NAME && colcon build --packages-select $PKG_NAME"
  echo ""
  echo "  2. 方案B：手动修复编码"
  echo "     python3 -c \""
  echo "with open('$BROKEN_FILE', 'rb') as f: data = f.read()"
  echo "# 尝试检测编码并转换"
  echo "for enc in ['gbk', 'gb2312', 'gb18030', 'latin-1']:"
  echo "    try:"
  echo "        text = data.decode(enc)"
  echo "        with open('$BROKEN_FILE', 'w', encoding='utf-8') as f: f.write(text)"
  echo "        print(f'已用 {enc} 解码并保存为 UTF-8')"
  echo "        break"
  echo "    except: pass"
  echo "\""
else
  echo "未找到损坏文件，应用通用补丁..."

  echo ""
  echo "=== 应用通用补丁：修改 _local_setup_util.py 增加编码容错 ==="
  PATCH_TARGET="/opt/ros/humble/_local_setup_util.py"
  if [ -f "$PATCH_TARGET" ]; then
    # 备份
    cp "$PATCH_TARGET" "${PATCH_TARGET}.bak"
    echo "已备份到 ${PATCH_TARGET}.bak"

    # 应用补丁 - 将 open(dsv_path, 'r') 改为 open(dsv_path, 'r', encoding='utf-8', errors='replace')
    python3 << 'PYFIX'
import re

target = '/opt/ros/humble/_local_setup_util.py'
with open(target, 'r') as f:
    content = f.read()

# 替换 open 调用
old = "with open(dsv_path, 'r') as h:"
new = "with open(dsv_path, 'r', encoding='utf-8', errors='replace') as h:"

if old in content:
    content = content.replace(old, new)
    with open(target, 'w') as f:
        f.write(content)
    print("补丁已应用: _local_setup_util.py")
    print(f"  {old}")
    print(f"  -> {new}")
else:
    print(f"未找到匹配行，当前内容可能已修改")
    # 显示相关行
    for i, line in enumerate(content.split('\n')):
        if 'open(dsv_path' in line:
            print(f"  Line {i+1}: {line.strip()}")
PYFIX
  else
    echo "找不到 $PATCH_TARGET"
  fi
fi

echo ""
echo "=== 完成 ==="
echo "请重新 source /opt/ros/humble/setup.bash 验证修复"
