from PIL import Image
import os

# 设置图片文件夹路径
folder_path = "C:/Users/Jade/OneDrive/桌面/"

# 获取文件夹中的所有JPG图片文件
for filename in os.listdir(folder_path):
    if filename.endswith(".png"):
        # 构造图片的完整路径
        img_path = os.path.join(folder_path, filename)
        
        # 打开图片
        with Image.open(img_path) as img:
            # 获取图片的宽度和高度
            width, height = img.size
            
            # 计算裁剪区域：从50到150宽度（若宽度不足150则自动调整）
            left = 0
            right = min(left + 10000, width)
            top = 100
            bottom = min(top + 905, height)
            
            # 裁剪图片
            cropped_img = img.crop((left, top, right, bottom))
            
            # 构造新的文件名（将.jpg替换为.png）
            new_filename = os.path.splitext(filename)[0] + ".png"
            
            # 保存裁剪后的图片为PNG格式
            cropped_img.save(os.path.join(folder_path, new_filename), format="PNG")
            print(f"裁剪并保存图片: {new_filename}")