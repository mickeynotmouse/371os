from PIL import Image
import numpy as np

# Load the screendump
IMG_NAME = "dump.ppm"  # adjust if your file is elsewhere
img = Image.open(IMG_NAME)

# Convert to a NumPy array (height x width x 3)
arr = np.array(img)

print("Image shape:", arr.shape)

# Example: extract the top row pixel colors as hex
top_row = arr[0]  # first row
hex_colors = ['#{:02X}{:02X}{:02X}'.format(r, g, b) for r, g, b in top_row]
print("Top row hex colors:", hex_colors)

# Optional: save all colors into a file for later use
with open("colors_hex.txt", "w") as f:
    for row in arr:
        hex_row = ['#{:02X}{:02X}{:02X}'.format(r, g, b) for r, g, b in row]
        f.write(" ".join(hex_row) + "\n")

print("Hex color extraction complete, saved to colors_hex.txt")
