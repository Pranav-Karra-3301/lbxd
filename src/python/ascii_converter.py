"""
@author: Viet Nguyen <nhviet1009@gmail.com>
Modified for LBXD integration
"""
import argparse
import os
import sys

import cv2
import numpy as np
from PIL import Image, ImageDraw, ImageOps, ImageFont
from utils import get_data


def get_args():
    parser = argparse.ArgumentParser("Image to ASCII")
    parser.add_argument("--input", type=str, required=True, help="Path to input image")
    parser.add_argument("--output", type=str, required=True, help="Path to output text file")
    parser.add_argument("--language", type=str, default="english")
    parser.add_argument("--mode", type=str, default="standard")
    parser.add_argument("--background", type=str, default="black", choices=["black", "white"],
                        help="background's color")
    parser.add_argument("--num_cols", type=int, default=100, help="number of character for output's width")
    parser.add_argument("--scale", type=int, default=2, help="upsize output")
    parser.add_argument("--color_output", action="store_true", help="Generate colored ASCII output")
    args = parser.parse_args()
    return args


def main(opt):
    try:
        if opt.background == "white":
            bg_code = (255, 255, 255)
        else:
            bg_code = (0, 0, 0)
        
        char_list, font, sample_character, scale = get_data(opt.language, opt.mode)
        num_chars = len(char_list)
        num_cols = opt.num_cols
        
        # Load and process image
        image = cv2.imread(opt.input, cv2.IMREAD_COLOR)
        if image is None:
            print(f"Error: Could not load image from {opt.input}", file=sys.stderr)
            sys.exit(1)
            
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        height, width, _ = image.shape
        
        cell_width = width / opt.num_cols
        cell_height = scale * cell_width
        num_rows = int(height / cell_height)
        
        if num_cols > width or num_rows > height:
            print("Too many columns or rows. Use default setting")
            cell_width = 6
            cell_height = 12
            num_cols = int(width / cell_width)
            num_rows = int(height / cell_height)
        
        # Generate ASCII art
        ascii_lines = []
        for i in range(num_rows):
            line = ""
            for j in range(num_cols):
                partial_image = image[int(i * cell_height):min(int((i + 1) * cell_height), height),
                                int(j * cell_width):min(int((j + 1) * cell_width), width), :]
                
                if partial_image.size == 0:
                    continue
                    
                partial_avg_color = np.sum(np.sum(partial_image, axis=0), axis=0) / (partial_image.shape[0] * partial_image.shape[1])
                char_index = min(int(np.mean(partial_image) * num_chars / 255), num_chars - 1)
                char = char_list[char_index]
                
                if opt.color_output:
                    # Generate ANSI color codes
                    r, g, b = partial_avg_color.astype(np.int32)
                    r, g, b = max(0, min(255, r)), max(0, min(255, g)), max(0, min(255, b))
                    colored_char = f"\033[38;2;{r};{g};{b}m{char}\033[0m"
                    line += colored_char
                else:
                    line += char
                    
            ascii_lines.append(line)
        
        # Write output
        with open(opt.output, 'w', encoding='utf-8') as f:
            f.write('\n'.join(ascii_lines))
            
        print(f"ASCII art generated successfully: {opt.output}")
        
    except Exception as e:
        print(f"Error generating ASCII art: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    opt = get_args()
    main(opt)