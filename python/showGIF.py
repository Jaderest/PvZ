from PIL import Image, ImageSequence

img = Image.open("C:/Users/Jade/Downloads/e23bb-main/663/images/Zombies/Zombie/ZombieAttack.gif")
duration = 0

for frame in ImageSequence.Iterator(img):
    duration += frame.info.get('duration', 0)

print(f"Total duration: {duration} ms")
