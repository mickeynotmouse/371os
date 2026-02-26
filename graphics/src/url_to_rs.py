import requests
from io import BytesIO
from PIL import Image
import numpy as np
import sys
import os

WIDTH, HEIGHT = 80, 25
OUTPUT = "colors/img.rs"

DEFAULT_URL = "https://cd-rs.github.io/os/img/rainbow.jpg"
IMG_URL = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_URL

VGA = np.array([
    [0,0,0], [0,0,170], [0,170,0], [0,170,170],
    [170,0,0], [170,0,170], [170,85,0], [170,170,170],
    [85,85,85], [85,85,255], [85,255,85], [85,255,255],
    [255,85,85], [255,85,255], [255,255,85], [255,255,255],
], dtype=np.int16)

img = Image.open(
    BytesIO(requests.get(IMG_URL).content)
).convert("RGB")

img = img.resize((WIDTH, HEIGHT), Image.NEAREST)
pixels = np.array(img, dtype=np.int16)

flat = pixels.reshape(-1, 3)

diff = flat[:, None, :] - VGA[None, :, :]
dist = np.sum(diff * diff, axis=2)

mapped = np.argmin(dist, axis=1).reshape(HEIGHT, WIDTH)

os.makedirs(os.path.dirname(OUTPUT), exist_ok=True)

with open(OUTPUT, "w") as f:
    f.write(f"pub const WIDTH: usize = {WIDTH};\n")
    f.write(f"pub const HEIGHT: usize = {HEIGHT};\n\n")
    f.write("pub const DATA: &[[u8; WIDTH]; HEIGHT] = &[\n")

    for row in mapped:
        f.write("    [" + ",".join(map(str, row)) + "],\n")

    f.write("];\n")

print("Generated:", OUTPUT)
