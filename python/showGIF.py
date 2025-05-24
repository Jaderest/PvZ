from PIL import Image, ImageSequence

img = Image.open("C:/Users/Jade/Downloads/19ca4-main/植物大战僵尸资源包)/images/Zombies/Zombie/Zombie2.gif")
duration = 0

for frame in ImageSequence.Iterator(img):
    duration += frame.info.get('duration', 0)

print(f"Total duration: {duration} ms")
