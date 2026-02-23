#!/usr/bin/env python3
"""Convert ANSI-colored terminal output to a PNG image."""

import sys
from PIL import Image, ImageDraw, ImageFont

FONT_PATH = "/usr/share/fonts/truetype/hack/Hack-Regular.ttf"
FONT_SIZE = 24
BG_COLOR = (24, 24, 24)
PADDING = 20

# Standard 3-bit ANSI colors (vibrant, readable on dark bg)
ANSI_COLORS = {
    30: (170, 170, 170),  # "black" -> gray (visible on dark bg)
    31: (240, 70, 70),    # red
    32: (70, 210, 70),    # green
    33: (230, 230, 70),   # yellow
    34: (90, 140, 250),   # blue
    35: (220, 100, 240),  # magenta
    36: (70, 230, 230),   # cyan
    37: (230, 230, 230),  # white
}

DEFAULT_FG = (200, 200, 200)

# Braille dot positions within a character cell.
# U+2800 offset encodes 8 dots in a 2-column x 4-row grid:
#   bit0=dot1(r0,c0)  bit3=dot4(r0,c1)
#   bit1=dot2(r1,c0)  bit4=dot5(r1,c1)
#   bit2=dot3(r2,c0)  bit5=dot6(r2,c1)
#   bit6=dot7(r3,c0)  bit7=dot8(r3,c1)
BRAILLE_DOT_BITS = [
    (0, 0, 0x01),
    (0, 1, 0x02),
    (0, 2, 0x04),
    (0, 3, 0x40),
    (1, 0, 0x08),
    (1, 1, 0x10),
    (1, 2, 0x20),
    (1, 3, 0x80),
]


def is_braille(ch):
    return 0x2800 <= ord(ch) <= 0x28FF


def has_braille(line):
    """Check if a line contains any Braille characters."""
    return any(is_braille(ch) for ch, _ in line)


def parse_ansi(text):
    """Parse text with ANSI codes into list of (char, color) tuples per line."""
    lines = []
    current_line = []
    color = DEFAULT_FG

    i = 0
    while i < len(text):
        if text[i] == '\x1b' and i + 1 < len(text) and text[i + 1] == '[':
            # Parse CSI sequence
            j = i + 2
            while j < len(text) and text[j] not in 'ABCDEFGHJKSTfmnsulh':
                j += 1
            if j < len(text) and text[j] == 'm':
                params = text[i + 2:j]
                codes = [int(c) if c else 0 for c in params.split(';')] if params else [0]
                for code in codes:
                    if code == 0:
                        color = DEFAULT_FG
                    elif code in ANSI_COLORS:
                        color = ANSI_COLORS[code]
                i = j + 1
            else:
                i = j + 1
        elif text[i] == '\n':
            lines.append(current_line)
            current_line = []
            i += 1
        else:
            current_line.append((text[i], color))
            i += 1

    if current_line:
        lines.append(current_line)

    return lines


def draw_braille(draw, x, y, ch, color, char_w, char_h, dot_r):
    """Draw a Braille character by rendering its dots manually."""
    bits = ord(ch) - 0x2800
    if bits == 0:
        return

    # Place dots at 1/4, 3/4 of cell width and 1/8, 3/8, 5/8, 7/8 of cell height.
    # This ensures equal spacing both within and across cell boundaries.
    col_positions = [char_w * 0.25, char_w * 0.75]
    row_positions = [char_h * 0.4375, char_h * 0.6875, char_h * 0.9375, char_h * 1.1875]

    for col, row, bit in BRAILLE_DOT_BITS:
        if bits & bit:
            cx = x + col_positions[col]
            cy = y + row_positions[row]
            draw.ellipse(
                [cx - dot_r, cy - dot_r, cx + dot_r, cy + dot_r],
                fill=color,
            )


def render_png(lines, output_path):
    """Render parsed lines to a PNG file."""
    font = ImageFont.truetype(FONT_PATH, FONT_SIZE)

    ascent, descent = font.getmetrics()
    text_line_h = ascent + descent

    # Braille line height must be a multiple of 4 so dot row spacing (h/4)
    # is an exact integer — otherwise rounding causes visible seams.
    # Also keep cells compact (not too tall) for a natural aspect ratio.
    braille_line_h = (round(text_line_h * 0.70) // 4) * 4

    # Character width (monospace), snapped to multiple of 4 for same reason
    raw_char_w = font.getbbox('M')[2] - font.getbbox('M')[0]
    char_w = (round(raw_char_w) // 4) * 4 or 4

    # Dot radius scales with font size
    dot_r = max(2, int(FONT_SIZE * 0.11))

    # Compute line heights
    line_heights = []
    for line in lines:
        if has_braille(line):
            line_heights.append(braille_line_h)
        else:
            line_heights.append(text_line_h)

    # Compute image dimensions
    max_cols = max((len(line) for line in lines), default=0)
    img_w = max_cols * char_w + 2 * PADDING
    img_h = sum(line_heights) + 2 * PADDING

    img = Image.new("RGB", (img_w, img_h), BG_COLOR)
    draw = ImageDraw.Draw(img)

    y = PADDING
    for row_idx, line in enumerate(lines):
        lh = line_heights[row_idx]
        for col_idx, (ch, color) in enumerate(line):
            x = PADDING + col_idx * char_w
            if is_braille(ch):
                draw_braille(draw, x, y, ch, color, char_w, lh, dot_r)
            else:
                draw.text((x, y), ch, font=font, fill=color)
        y += lh

    img.save(output_path, optimize=True)


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <output.png>", file=sys.stderr)
        print("Reads ANSI text from stdin.", file=sys.stderr)
        sys.exit(1)

    text = sys.stdin.read()
    lines = parse_ansi(text)

    # Strip trailing empty lines
    while lines and not lines[-1]:
        lines.pop()

    render_png(lines, sys.argv[1])


if __name__ == "__main__":
    main()
