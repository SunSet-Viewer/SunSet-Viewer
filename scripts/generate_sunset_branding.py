from pathlib import Path

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[1]
BRANDING = ROOT / "assets" / "branding"
SIZE = 720
SCALE = 4
PALETTE = [
    (255, 224, 102),
    (255, 158, 59),
    (255, 82, 82),
    (224, 36, 117),
    (99, 102, 241),
]


def interpolate(start: tuple[int, int, int], end: tuple[int, int, int], amount: float) -> tuple[int, int, int]:
    return tuple(round(a + (b - a) * amount) for a, b in zip(start, end))


def color_at(amount: float) -> tuple[int, int, int, int]:
    scaled = amount * (len(PALETTE) - 1)
    index = min(int(scaled), len(PALETTE) - 2)
    return (*interpolate(PALETTE[index], PALETTE[index + 1], scaled - index), 255)


def render_logo() -> Image.Image:
    size = SIZE * SCALE
    mask = Image.new("L", (size, size))
    ImageDraw.Draw(mask).ellipse((100 * SCALE, 100 * SCALE, 620 * SCALE, 620 * SCALE), fill=255)
    gradient = Image.new("RGBA", (size, size))
    pixels = gradient.load()
    for y in range(size):
        color = color_at(y / (size - 1))
        for x in range(size):
            pixels[x, y] = color
    gradient.putalpha(mask)

    draw = ImageDraw.Draw(gradient)
    draw.polygon(
        [
            (100 * SCALE, 430 * SCALE),
            (225 * SCALE, 345 * SCALE),
            (315 * SCALE, 405 * SCALE),
            (435 * SCALE, 310 * SCALE),
            (620 * SCALE, 445 * SCALE),
            (620 * SCALE, 620 * SCALE),
            (100 * SCALE, 620 * SCALE),
        ],
        fill=(24, 27, 45, 255),
    )
    draw.polygon(
        [
            (100 * SCALE, 515 * SCALE),
            (255 * SCALE, 425 * SCALE),
            (360 * SCALE, 500 * SCALE),
            (490 * SCALE, 405 * SCALE),
            (620 * SCALE, 495 * SCALE),
            (620 * SCALE, 620 * SCALE),
            (100 * SCALE, 620 * SCALE),
        ],
        fill=(11, 15, 32, 255),
    )
    return gradient.resize((SIZE, SIZE), Image.Resampling.LANCZOS)


def main() -> None:
    BRANDING.mkdir(parents=True, exist_ok=True)
    logo = render_logo()
    logo.save(BRANDING / "sunset-logo.png")
    logo.save(BRANDING / "sunset.ico", format="ICO", sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])

    background = Image.new("RGBA", (1024, 1024), (7, 11, 31, 255))
    icon = logo.resize((820, 820), Image.Resampling.LANCZOS)
    background.alpha_composite(icon, ((1024 - 820) // 2, (1024 - 820) // 2))
    background.save(BRANDING / "sunset.icns", format="ICNS")

    (BRANDING / "sunset-logo.svg").write_text(
        """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 720">
  <defs>
    <linearGradient id="sunset" x1="0" x2="0" y1="0" y2="1">
      <stop offset="0" stop-color="#ffe066"/>
      <stop offset=".22" stop-color="#ff9e3b"/>
      <stop offset=".48" stop-color="#ff5252"/>
      <stop offset=".72" stop-color="#e02475"/>
      <stop offset="1" stop-color="#6366f1"/>
    </linearGradient>
    <clipPath id="disc"><circle cx="360" cy="360" r="260"/></clipPath>
  </defs>
  <g clip-path="url(#disc)">
    <circle cx="360" cy="360" r="260" fill="url(#sunset)"/>
    <path d="M100 430 225 345l90 60 120-95 185 135v175H100z" fill="#181b2d"/>
    <path d="m100 515 155-90 105 75 130-95 130 90v125H100z" fill="#0b0f20"/>
  </g>
</svg>
""",
        encoding="utf-8",
    )
    (BRANDING / "sunset-logo-white.svg").write_text(
        """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 720">
  <defs><clipPath id="disc"><circle cx="360" cy="360" r="260"/></clipPath></defs>
  <g clip-path="url(#disc)" fill="#fff">
    <circle cx="360" cy="360" r="260"/>
    <path d="M100 430 225 345l90 60 120-95 185 135v175H100z" fill="#000"/>
    <path d="m100 515 155-90 105 75 130-95 130 90v125H100z" fill="#000"/>
  </g>
</svg>
""",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
