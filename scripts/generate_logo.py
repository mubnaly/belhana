"""Regenerate the Belhana logo artwork used by web, desktop and Android.

Requires Pillow (`pip install pillow`). Run from the repository root:
    python scripts/generate_logo.py

The SVG is the source of truth for the web; PNG/ICO assets below are rasterised
from the same geometry so the app icon and splash screen always match.
"""

from pathlib import Path
import math

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
WEB = ROOT / "apps" / "web" / "static"
TAURI_ICONS = ROOT / "apps" / "desktop" / "src-tauri" / "icons"
ANDROID_RES = ROOT / "apps" / "desktop" / "src-tauri" / "gen" / "android" / "app" / "src" / "main" / "res"

GREEN = "#14543F"
GREEN_DARK = "#0E3B2C"
LEAF = "#6FB98F"
LEAF_LIGHT = "#8CCB9F"
GOLD = "#E9B44C"
CREAM = "#F7EEDC"
RICE = "#FFF7E6"
SHADE = "#EAD9B6"
GRAIN = "#E8DABF"

SVG = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="512" height="512" role="img" aria-label="Belhana">
  <circle cx="256" cy="256" r="248" fill="#14543F"/>
  <circle cx="356" cy="158" r="46" fill="#E9B44C"/>
  <circle cx="356" cy="158" r="36" fill="none" stroke="#FFF3D6" stroke-width="6"/>
  <path d="M356 136v44M344 148h24" fill="none" stroke="#FFF3D6" stroke-width="7" stroke-linecap="round"/>
  <path d="M252 246C218 190 226 138 268 104c18 46 10 104-16 142Z" fill="#6FB98F"/>
  <path d="M262 250c34-58 78-76 116-72-16 50-58 86-116 72Z" fill="#8CCB9F"/>
  <path d="M256 274c-2-56 4-104 18-142" fill="none" stroke="#0E3B2C" stroke-width="9" stroke-linecap="round"/>
  <path d="M144 286c8-58 56-96 112-96s104 38 112 96Z" fill="#FFF7E6"/>
  <g fill="#E8DABF">
    <ellipse cx="196" cy="245" rx="9" ry="5" transform="rotate(-28 196 245)"/>
    <ellipse cx="232" cy="220" rx="9" ry="5" transform="rotate(18 232 220)"/>
    <ellipse cx="274" cy="214" rx="9" ry="5" transform="rotate(-18 274 214)"/>
    <ellipse cx="312" cy="240" rx="9" ry="5" transform="rotate(28 312 240)"/>
  </g>
  <path d="M120 284h272c-8 76-62 130-136 130S128 360 120 284Z" fill="#F7EEDC"/>
  <path d="M136 302h240c-8 56-52 98-120 98s-112-42-120-98Z" fill="#EAD9B6"/>
  <ellipse cx="256" cy="284" rx="136" ry="22" fill="#FFF7E6"/>
  <path d="M170 392c22 17 52 25 86 25s64-8 86-25" fill="none" stroke="#E9B44C" stroke-width="10" stroke-linecap="round"/>
</svg>
"""


def bezier(p0, p1, p2, p3, steps=24):
    pts = []
    for i in range(steps + 1):
        t = i / steps
        u = 1 - t
        pts.append((
            u**3 * p0[0] + 3*u*u*t * p1[0] + 3*u*t*t * p2[0] + t**3 * p3[0],
            u**3 * p0[1] + 3*u*u*t * p1[1] + 3*u*t*t * p2[1] + t**3 * p3[1],
        ))
    return pts


def draw_logo(size, scale_marks=1.0, badge=True):
    ss = 4
    canvas = size * ss
    img = Image.new("RGBA", (canvas, canvas), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    k = canvas / 512.0

    def P(*pts):
        return [(x * k, y * k) for x, y in pts]

    def ellipse(box, fill=None, outline=None, width=1):
        d.ellipse([v * k for v in box], fill=fill, outline=outline, width=max(1, int(width * k)))

    def poly(points, fill):
        d.polygon(P(*points), fill=fill)

    def line(points, fill, width):
        d.line(P(*points), fill=fill, width=max(1, int(width * k)), joint="curve")

    if badge:
        ellipse((8, 8, 504, 504), fill=GREEN)

    # Marks are grouped so adaptive-icon foregrounds can be inset safely.
    offset = (1 - scale_marks) * 256
    def S(pt):
        return (offset + pt[0] * scale_marks, offset + pt[1] * scale_marks)

    def SB(box):
        a, b = S((box[0], box[1])), S((box[2], box[3]))
        return (a[0], a[1], b[0], b[1])

    ellipse(SB((310, 112, 402, 204)), fill=GOLD)
    ellipse(SB((320, 122, 392, 194)), outline="#FFF3D6", width=6)
    line([S((356, 136)), S((356, 180))], "#FFF3D6", 7)
    line([S((344, 148)), S((368, 148))], "#FFF3D6", 7)

    poly([S(p) for p in bezier((252, 246), (218, 190), (226, 138), (268, 104))] + [S((252, 246))], LEAF)
    poly([S(p) for p in bezier((262, 250), (296, 192), (340, 174), (378, 178))] + [S((262, 250))], LEAF_LIGHT)
    line([S((256, 274)), S((254, 218)), S((264, 160)), S((274, 132))], GREEN_DARK, 9)

    poly(
        [S(p) for p in bezier((144, 286), (152, 228), (200, 190), (256, 190))]
        + [S(p) for p in bezier((256, 190), (312, 190), (360, 228), (368, 286))]
        + [S((368, 286)), S((144, 286))],
        RICE,
    )
    for cx, cy, angle in [(196, 245, -28), (232, 220, 18), (274, 214, -18), (312, 240, 28)]:
        gx, gy = S((cx, cy))
        g = Image.new("RGBA", (canvas, canvas), (0, 0, 0, 0))
        gd = ImageDraw.Draw(g)
        r = 9 * k * scale_marks
        gd.ellipse([gx - r, gy - r * 0.55, gx + r, gy + r * 0.55], fill=GRAIN)
        g = g.rotate(angle, resample=Image.Resampling.BICUBIC, center=(gx, gy))
        img.alpha_composite(g)
        d = ImageDraw.Draw(img)

    poly(
        [S((120, 284)), S((392, 284))]
        + [S(p) for p in bezier((392, 284), (384, 360), (330, 414), (256, 414))]
        + [S(p) for p in bezier((256, 414), (182, 414), (128, 360), (120, 284))],
        CREAM,
    )
    poly(
        [S((136, 302)), S((376, 302))]
        + [S(p) for p in bezier((376, 302), (368, 358), (324, 400), (256, 400))]
        + [S(p) for p in bezier((256, 400), (188, 400), (144, 358), (136, 302))],
        SHADE,
    )
    ellipse(SB((120, 262, 392, 306)), fill=RICE)
    line([S(p) for p in bezier((170, 392), (192, 409), (222, 417), (256, 417))], GOLD, 10)
    line([S(p) for p in bezier((256, 417), (290, 417), (320, 409), (342, 392))], GOLD, 10)

    return img.resize((size, size), Image.Resampling.LANCZOS)


def save_png(path, img):
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path, optimize=True)


def save_ico(path, sizes):
    path.parent.mkdir(parents=True, exist_ok=True)
    images = [draw_logo(s) for s in sizes]
    images[0].save(path, format="ICO", sizes=[(s, s) for s in sizes])


def save_round(img, path):
    size = img.width
    mask = Image.new("L", (size * 4, size * 4), 0)
    ImageDraw.Draw(mask).ellipse((0, 0, size * 4 - 1, size * 4 - 1), fill=255)
    out = img.copy()
    out.putalpha(mask.resize((size, size), Image.Resampling.LANCZOS))
    save_png(path, out)


def main():
    WEB.mkdir(parents=True, exist_ok=True)
    (WEB / "logo.svg").write_text(SVG, encoding="utf-8")
    base = draw_logo(512)
    save_png(WEB / "logo.png", base)
    save_png(WEB / "favicon.png", draw_logo(64))
    save_png(TAURI_ICONS / "icon.png", base)
    save_ico(TAURI_ICONS / "icon.ico", [16, 32, 48, 64, 128, 256])

    splash = ANDROID_RES / "drawable-nodpi"
    save_png(splash / "splash_logo.png", draw_logo(512, scale_marks=0.82, badge=False))

    densities = {"mdpi": 48, "hdpi": 72, "xhdpi": 96, "xxhdpi": 144, "xxxhdpi": 192}
    for density, size in densities.items():
        folder = ANDROID_RES / f"mipmap-{density}"
        icon = draw_logo(size)
        save_png(folder / "ic_launcher.png", icon)
        save_round(icon, folder / "ic_launcher_round.png")
        save_png(folder / "ic_launcher_foreground.png", draw_logo(size, scale_marks=0.72, badge=False))

    print("Logo assets written for web, desktop and Android")


if __name__ == "__main__":
    main()
