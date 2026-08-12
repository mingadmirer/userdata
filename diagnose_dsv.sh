#!/bin/bash
# 诊断脚本：找出哪个 .dsv 文件包含非 UTF-8 编码字节
# 在 10.159.132.24 上运行此脚本

echo "========================================="
echo "扫描 .dsv 文件中的编码问题..."
echo "========================================="

BROKEN_FILES=()

# 扫描 /opt/ros/humble 和常见工作空间路径
for dir in /opt/ros/humble /home /root /workspace /userdata; do
  if [ ! -d "$dir" ]; then continue; fi
  while IFS= read -r -d '' f; do
    # 检查文件是否包含非 ASCII 字节
    if ! python3 -c "
try:
    with open('$f', 'r', encoding='utf-8') as fh:
        fh.read()
except UnicodeDecodeError as e:
    exit(1)
" 2>/dev/null; then
      echo ""
      echo "❌ 损坏文件: $f"
      BROKEN_FILES+=("$f")
      # 显示原始字节（前 40 字节）
      python3 -c "
with open('$f', 'rb') as fh:
    data = fh.read(100)
print('  原始字节(hex):', data[:40].hex(' '))
print('  原始字节(raw):', data[:40])
# 尝试 GBK 解码
try:
    text = data.decode('gbk')
    print('  GBK解码成功:', repr(text[:80]))
except:
    pass
# 尝试 latin-1 解码
try:
    text = data.decode('latin-1')
    print('  latin-1解码:', repr(text[:80]))
except:
    pass
"
    fi
  done < <(find "$dir" -name "*.dsv" -type f -print0 2>/dev/null)
done

echo ""
echo "========================================="
echo "结果汇总:"
if [ ${#BROKEN_FILES[@]} -eq 0 ]; then
  echo "  ✅ 未找到损坏的 .dsv 文件"
  echo "  （错误可能来自 Python 包中的 pyc 文件或其他源）"
else
  echo "  ❌ 找到 ${#BROKEN_FILES[@]} 个损坏的 .dsv 文件:"
  for f in "${BROKEN_FILES[@]}"; do
    echo "     $f"
  done
fi
echo "========================================="
