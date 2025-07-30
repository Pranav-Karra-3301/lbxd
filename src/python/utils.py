"""
Utility functions for ASCII art generation
"""
import os
from PIL import ImageFont


def get_data(language="english", mode="standard"):
    """
    Get character list, font, sample character, and scale for ASCII conversion
    """
    # Character sets for different modes
    char_sets = {
        "standard": " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
        "simple": " .:-=+*#%@",
        "complex": " ░▒▓█▉▊▋▌▍▎▏▕▐",
        "blocks": " ▁▂▃▄▅▆▇█"
    }
    
    # Get character list
    char_list = list(char_sets.get(mode, char_sets["standard"]))
    
    # Try to load a monospace font, fallback to default
    font = None
    font_paths = [
        "/System/Library/Fonts/Monaco.ttf",  # macOS
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",  # Linux
        "C:/Windows/Fonts/consola.ttf",  # Windows
    ]
    
    font_size = 12
    for font_path in font_paths:
        if os.path.exists(font_path):
            try:
                font = ImageFont.truetype(font_path, font_size)
                break
            except Exception:
                continue
    
    # Fallback to default font
    if font is None:
        try:
            font = ImageFont.load_default()
        except Exception:
            # PIL default font fallback
            font = ImageFont.load_default()
    
    # Sample character for measurements
    sample_character = "M"
    
    # Scale factor
    scale = 0.43  # Aspect ratio compensation for monospace fonts
    
    return char_list, font, sample_character, scale


def detect_terminal_colors():
    """
    Detect if terminal supports colors
    """
    import subprocess
    import os
    
    # Check TERM environment variable
    term = os.environ.get('TERM', '')
    if '256color' in term or 'color' in term:
        return True
    
    # Check tput colors
    try:
        result = subprocess.run(['tput', 'colors'], capture_output=True, text=True, timeout=5)
        if result.returncode == 0:
            colors = int(result.stdout.strip())
            return colors >= 8
    except Exception:
        pass
    
    # Check for common color-supporting terminals
    color_terms = ['xterm', 'screen', 'tmux', 'rxvt', 'gnome', 'konsole', 'iterm']
    return any(color_term in term.lower() for color_term in color_terms)


if __name__ == "__main__":
    # Test the functions
    char_list, font, sample_char, scale = get_data()
    print(f"Character list length: {len(char_list)}")
    print(f"Sample characters: {''.join(char_list[:10])}...")
    print(f"Font: {font}")
    print(f"Terminal supports colors: {detect_terminal_colors()}")