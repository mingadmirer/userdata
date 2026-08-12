#!/bin/bash
# ============================================================
# ROS2 .dsv 文件编码问题一键修复脚本
# 在 10.159.132.24 机器上运行此脚本
# 用法: sudo bash fix_dsv_encoding.sh
# ============================================================
set -e

echo "╔══════════════════════════════════════════════╗"
echo "║   ROS2 .dsv UnicodeDecodeError 修复工具      ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

# ==========================================
# Part 1: 修补 _local_setup_util.py (治标)
# ==========================================
TARGET="/opt/ros/humble/_local_setup_util.py"
OLD_LINE="with open(dsv_path, 'r') as h:"
NEW_LINE="with open(dsv_path, 'r', encoding='utf-8', errors='replace') as h:"

echo "[1/2] 修补 ROS2 系统文件（增加编码容错）..."
if [ -f "$TARGET" ]; then
    cp "$TARGET" "${TARGET}.bak.$(date +%Y%m%d_%H%M%S)"
    if grep -qF "$OLD_LINE" "$TARGET"; then
        sed -i "s|$OLD_LINE|$NEW_LINE|" "$TARGET"
        echo "  ✅ 补丁已应用: $(grep -n 'open(dsv_path' "$TARGET")"
    else
        echo "  ⚠️  未找到目标行，可能已修改过:"
        grep -n 'open(dsv_path' "$TARGET" || echo "  (未找到 open(dsv_path 调用)"
    fi
else
    echo "  ❌ 找不到 $TARGET"
fi

# ==========================================
# Part 2: 查找并修复损坏的 .dsv 文件 (治本)
# ==========================================
echo ""
echo "[2/2] 扫描损坏的 .dsv 源文件..."

FOUND=0

scan_dir() {
    local dir="$1"
    [ ! -d "$dir" ] && return
    while IFS= read -r -d '' f; do
        if ! python3 -c "open('$f', encoding='utf-8').read()" 2>/dev/null; then
            echo ""
            echo "  ❌ 发现损坏文件: $f"

            # 分析内容
            python3 -c "
import sys
with open('$f', 'rb') as fh:
    data = fh.read()
print(f'    文件大小: {len(data)} bytes')
print(f'    前20字节(hex): {data[:20].hex(\" \")}')

# 尝试常见编码
for enc in ['gbk', 'gb2312', 'gb18030', 'latin-1', 'cp1252']:
    try:
        text = data.decode(enc)
        print(f'    {enc} 解码成功: {text[:100]}')
        # 自动修复：写回 UTF-8
        with open('$f', 'w', encoding='utf-8') as fh:
            fh.write(text)
        print(f'    ✅ 已修复: $f ({enc} -> UTF-8)')
        sys.exit(0)
    except Exception as e:
        pass

# 都失败了，用 replace 策略
text = data.decode('utf-8', errors='replace')
with open('$f', 'w', encoding='utf-8') as fh:
    fh.write(text)
print(f'    ✅ 已修复(replace策略): $f')
"
            FOUND=$((FOUND + 1))
        fi
    done < <(find "$dir" -name "*.dsv" -type f -print0 2>/dev/null)
}

scan_dir "/opt/ros/humble"
scan_dir "/home"
scan_dir "/userdata"
scan_dir "/workspace"

if [ "$FOUND" -eq 0 ]; then
    echo "  ✅ 未发现损坏的 .dsv 文件（Part 1 的系统文件补丁已足够）"
else
    echo "  ✅ 共修复 $FOUND 个文件"
fi

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║  修复完成!                                   ║"
echo "║  请运行以下命令验证:                          ║"
echo "║    source /opt/ros/humble/setup.bash          ║"
echo "╚══════════════════════════════════════════════╝"
