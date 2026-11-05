import os
import numpy as np
from PIL import Image, ImageDraw

def create_all_branding_assets(target_dir):
    os.makedirs(target_dir, exist_ok=True)
    
    # 4x supersampling: 720 * 4 = 2880
    scale = 4
    W = 720 * scale
    H = 720 * scale
    cx = 360 * scale
    cy = 330 * scale
    r = 240 * scale
    
    bands = [
        (90 * scale, 250 * scale),
        (264 * scale, 304 * scale),
        (322 * scale, 356 * scale),
        (378 * scale, 406 * scale),
        (432 * scale, 454 * scale),
        (484 * scale, 500 * scale)
    ]
    
    lines = [
        (524 * scale, 16 * scale, 440 * scale),
        (556 * scale, 14 * scale, 360 * scale),
        (586 * scale, 12 * scale, 280 * scale),
        (614 * scale, 10 * scale, 200 * scale),
        (640 * scale, 8 * scale, 120 * scale)
    ]
    
    # Create mask image (grayscale 8-bit, 2880x2880)
    mask_im = Image.new('L', (W, H), 0)
    draw = ImageDraw.Draw(mask_im)
    
    # Draw circle mask
    circle_mask = Image.new('L', (W, H), 0)
    cdraw = ImageDraw.Draw(circle_mask)
    cdraw.ellipse([cx - r, cy - r, cx + r, cy + r], fill=255)
    
    # Draw rect bands
    rect_im = Image.new('L', (W, H), 0)
    rdraw = ImageDraw.Draw(rect_im)
    for y1, y2 in bands:
        rdraw.rectangle([cx - r - 10, y1, cx + r + 10, y2], fill=255)
        
    # Intersect rects with circle
    rect_arr = np.array(rect_im)
    circle_arr = np.array(circle_mask)
    sun_bands = np.bitwise_and(rect_arr, circle_arr)
    
    # Add reflection lines with rounded corners
    lines_im = Image.new('L', (W, H), 0)
    ldraw = ImageDraw.Draw(lines_im)
    for y, h, w in lines:
        x0 = cx - w // 2
        x1 = cx + w // 2
        y0 = y
        y1 = y + h
        radius = h // 2
        ldraw.rounded_rectangle([x0, y0, x1, y1], radius=radius, fill=255)
        
    lines_arr = np.array(lines_im)
    combined_mask = np.bitwise_or(sun_bands, lines_arr)
    
    # Build color gradient image
    # y_min = 90 * scale, y_max = 648 * scale
    y_min = 90.0 * scale
    y_max = 648.0 * scale
    
    stops = [
        (0.00, np.array([255, 224, 102], dtype=np.float32)), # #FFE066
        (0.22, np.array([255, 158, 59], dtype=np.float32)),  # #FF9E3B
        (0.48, np.array([255, 82, 82], dtype=np.float32)),   # #FF5252
        (0.72, np.array([224, 36, 117], dtype=np.float32)),  # #E02475
        (0.90, np.array([147, 51, 234], dtype=np.float32)),  # #9333EA
        (1.00, np.array([99, 102, 241], dtype=np.float32))   # #6366F1
    ]
    
    # Create gradient table for y from 0 to H-1
    y_indices = np.arange(H, dtype=np.float32)
    t = np.clip((y_indices - y_min) / (y_max - y_min), 0.0, 1.0)
    
    grad_lut = np.zeros((H, 3), dtype=np.uint8)
    for i in range(len(stops) - 1):
        t0, c0 = stops[i]
        t1, c1 = stops[i+1]
        idx = (t >= t0) & (t <= t1)
        if np.any(idx):
            sub_t = ((t[idx] - t0) / (t1 - t0))[:, None]
            c = c0[None, :] * (1.0 - sub_t) + c1[None, :] * sub_t
            grad_lut[idx] = np.clip(c, 0, 255).astype(np.uint8)
            
    # Full gradient image: repeat columns
    rgb_arr = np.tile(grad_lut[:, None, :], (1, W, 1))
    
    # Combine RGB and Alpha
    rgba_arr = np.dstack([rgb_arr, combined_mask])
    full_hires = Image.fromarray(rgba_arr, 'RGBA')
    
    # Downsample to 720x720 (crisp antialiasing)
    logo_720 = full_hires.resize((720, 720), Image.Resampling.LANCZOS)
    logo_720.save(os.path.join(target_dir, 'aurora-logo.png'), 'PNG')
    print("Saved aurora-logo.png")
    
    # Downsample / generate 1024x1024 on dark navy background (#070B1F)
    bg_1024 = Image.new('RGBA', (1024, 1024), (7, 11, 31, 255))
    logo_fit = full_hires.resize((820, 820), Image.Resampling.LANCZOS)
    bg_1024.paste(logo_fit, ((1024 - 820)//2, (1024 - 820)//2), logo_fit)
    bg_1024.save(os.path.join(target_dir, 'aurora-logo-bg.png'), 'PNG')
    print("Saved aurora-logo-bg.png")
    
    # Windows Multi-size ICO: 16, 24, 32, 48, 64, 128, 256
    ico_im = full_hires.resize((256, 256), Image.Resampling.LANCZOS)
    ico_im.save(
        os.path.join(target_dir, 'aurora.ico'),
        format='ICO',
        sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    )
    print("Saved aurora.ico")
    
    # macOS ICNS
    bg_1024.save(os.path.join(target_dir, 'aurora.icns'), format='ICNS')
    print("Saved aurora.icns")
    
    # Monochrome variants
    # White silhouette
    mask_hires = Image.fromarray(combined_mask, 'L')
    mask_720 = mask_hires.resize((720, 720), Image.Resampling.LANCZOS)
    
    white_720 = Image.new('RGBA', (720, 720), (255, 255, 255, 255))
    white_720.putalpha(mask_720)
    white_720.save(os.path.join(target_dir, 'aurora-logo-white.png'), 'PNG')
    
    black_720 = Image.new('RGBA', (720, 720), (0, 0, 0, 255))
    black_720.putalpha(mask_720)
    black_720.save(os.path.join(target_dir, 'aurora-logo-black.png'), 'PNG')
    
    # Purple #8B5CF6
    purple_720 = Image.new('RGBA', (720, 720), (139, 92, 246, 255))
    purple_720.putalpha(mask_720)
    purple_720.save(os.path.join(target_dir, 'aurora-logo-purple.png'), 'PNG')
    
    # Teal #5EEAD4
    teal_720 = Image.new('RGBA', (720, 720), (94, 234, 212, 255))
    teal_720.putalpha(mask_720)
    teal_720.save(os.path.join(target_dir, 'aurora-logo-teal.png'), 'PNG')
    print("Saved monochrome and tint PNGs")

    # Generate SVGs
    # Color SVG
    raw_bands = [(90, 250), (264, 304), (322, 356), (378, 406), (432, 454), (484, 500)]
    raw_lines = [(524, 16, 440), (556, 14, 360), (586, 12, 280), (614, 10, 200), (640, 8, 120)]
    rcx, rcy, rr = 360, 330, 240
    
    color_svg = f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 720" width="720" height="720">
  <defs>
    <linearGradient id="sunsetGrad" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#FFE066" />
      <stop offset="22%" stop-color="#FF9E3B" />
      <stop offset="48%" stop-color="#FF5252" />
      <stop offset="72%" stop-color="#E02475" />
      <stop offset="90%" stop-color="#9333EA" />
      <stop offset="100%" stop-color="#6366F1" />
    </linearGradient>
    <clipPath id="sunDisc">
      <circle cx="{rcx}" cy="{rcy}" r="{rr}" />
    </clipPath>
  </defs>
  <g fill="url(#sunsetGrad)">
    <g clip-path="url(#sunDisc)">
'''
    for y1, y2 in raw_bands:
        h = y2 - y1
        color_svg += f'      <rect x="{rcx - rr}" y="{y1}" width="{2*rr}" height="{h}" />\n'
    color_svg += '    </g>\n'
    for y, h, w in raw_lines:
        x = rcx - w // 2
        rx = h / 2.0
        color_svg += f'    <rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{rx}" />\n'
    color_svg += '''  </g>
</svg>
'''
    with open(os.path.join(target_dir, 'aurora-logo.svg'), 'w', encoding='utf-8') as f:
        f.write(color_svg)

    def make_mono_svg(fill_color):
        s = f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 720" width="720" height="720">
  <defs>
    <clipPath id="sunDisc">
      <circle cx="{rcx}" cy="{rcy}" r="{rr}" />
    </clipPath>
  </defs>
  <g fill="{fill_color}">
    <g clip-path="url(#sunDisc)">
'''
        for y1, y2 in raw_bands:
            h = y2 - y1
            s += f'      <rect x="{rcx - rr}" y="{y1}" width="{2*rr}" height="{h}" />\n'
        s += '    </g>\n'
        for y, h, w in raw_lines:
            x = rcx - w // 2
            rx = h / 2.0
            s += f'    <rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{rx}" />\n'
        s += '''  </g>
</svg>
'''
        return s

    with open(os.path.join(target_dir, 'aurora-logo-white.svg'), 'w', encoding='utf-8') as f:
        f.write(make_mono_svg('#ffffff'))
    with open(os.path.join(target_dir, 'aurora-logo-black.svg'), 'w', encoding='utf-8') as f:
        f.write(make_mono_svg('#000000'))
    print("Saved all SVGs successfully")

if __name__ == '__main__':
    target = os.path.join(os.path.dirname(__file__), '..', 'assets', 'branding')
    create_all_branding_assets(target)

