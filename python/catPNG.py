import re
from PIL import Image
import os

def natural_sort_key(s):
    # 使用正则表达式提取字符串中的数字部分，并将其转换为整数
    return [int(text) if text.isdigit() else text.lower() for text in re.split('(\d+)', s)]

def concat_images_horizontally(folder_path, output_path='output.png'):
    # 获取所有png文件并按自然顺序排序
    image_files = sorted([
        f for f in os.listdir(folder_path)
        if f.lower().endswith('.png')
    ], key=natural_sort_key)

    # 加载所有图像
    images = [Image.open(os.path.join(folder_path, f)) for f in image_files]

    # 检查图像是否为空
    if not images:
        print("没有找到 PNG 图片")
        return

    # 统一高度，不做缩放，直接拼接
    heights = [img.height for img in images]
    widths = [img.width for img in images]
    total_width = sum(widths)
    max_height = max(heights)

    # 创建新的空白图像
    new_image = Image.new('RGBA', (total_width, max_height))

    # 拼接图像
    x_offset = 0
    for img in images:
        print(f"拼接图像: {img.filename}")
        new_image.paste(img, (x_offset, 0))
        x_offset += img.width

    # 保存输出图像
    new_image.save(output_path)
    print(f"拼接完成，保存为 {output_path}")

# 🧪 示例使用
# 请替换为你自己的路径
if __name__ == '__main__':
    folder = 'C:/Users/Jade/Downloads/ConeheadZombieAttack'
    concat_images_horizontally(folder, 'ConeheadZombieAttack.png')
