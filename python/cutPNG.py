from PIL import Image
import os

# 设置图片文件夹路径
folder_path = "C:/Users/Jade/Downloads/ZombieAttack"

# 获取文件夹中的所有PNG图片文件
for filename in os.listdir(folder_path):
    if filename.endswith(".png"):
        # 构造图片的完整路径
        img_path = os.path.join(folder_path, filename)
        
        # 打开图片
        with Image.open(img_path) as img:
            # 获取图片的宽度和高度
            width, height = img.size
            
            # 计算裁剪区域：从50到150宽度（若宽度不足150则自动调整）
            left = 65
            right = min(150, width)
            top = 15
            bottom = height
            
            # 裁剪图片
            cropped_img = img.crop((left, top, right, bottom))
            
            # 保存裁剪后的图片
            cropped_img.save(os.path.join(folder_path, f"cropped_{filename}"))
            print(f"裁剪并保存图片: {filename}")
