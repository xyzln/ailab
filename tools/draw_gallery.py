# -*- coding: utf-8 -*-
"""绘制练习手册结构画廊图（教学插图风）。"""
W, H = 1200, 660
CW, CH, GAP, MX, TOP = 270, 250, 20, 30, 96

BG, CARD, INK, SUB = "#FDFBF7", "#FFFFFF", "#2C3E50", "#8A94A6"
BLUE, ORANGE, GREEN, RED = "#4A90D9", "#F5A623", "#5CB85C", "#E94B3C"
PURPLE, TEAL, GREY, PINK = "#9B59B6", "#1ABC9C", "#95A5A6", "#ED6498"

parts = []
FONT = "Noto Sans CJK SC, Helvetica, Arial, sans-serif"

def rect(x, y, w, h, fill, rx=4, stroke=None, sw=1.5, extra=""):
    s = f' stroke="{stroke}" stroke-width="{sw}"' if stroke else ""
    parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{rx}" fill="{fill}"{s} {extra}/>')
def line(x1, y1, x2, y2, c, w=2, dash=None, marker=None):
    d = f' stroke-dasharray="{dash}"' if dash else ""
    m = f' marker-end="url(#{marker})"' if marker else ""
    parts.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{c}" stroke-width="{w}"{d}{m}/>')
def circle(cx, cy, r, fill, stroke=None, sw=2):
    s = f' stroke="{stroke}" stroke-width="{sw}"' if stroke else ""
    parts.append(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{fill}"{s}/>')
def text(x, y, s, size=13, fill=INK, weight="600", anchor="start", font=FONT):
    esc = s.replace("&","&amp;").replace("<","&lt;").replace(">","&gt;")
    parts.append(f'<text x="{x}" y="{y}" font-family="{font}" font-size="{size}" fill="{fill}" font-weight="{weight}" text-anchor="{anchor}">{esc}</text>')
def text_raw(x, y, inner, size=13, fill=INK, weight="600", anchor="start"):
    parts.append(f'<text x="{x}" y="{y}" font-family="{FONT}" font-size="{size}" fill="{fill}" font-weight="{weight}" text-anchor="{anchor}">{inner}</text>')
def path(d, fill="none", stroke=GREY, w=2, marker=None, dash=None):
    da = f' stroke-dasharray="{dash}"' if dash else ""
    m = f' marker-end="url(#{marker})"' if marker else ""
    parts.append(f'<path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{w}"{da}{m}/>')

# ---------- 背景 ----------
rect(0, 0, W, H, BG)
for gx in range(0, W, 24):
    parts.append(f'<line x1="{gx}" y1="0" x2="{gx}" y2="{H}" stroke="#F0ECE4" stroke-width="1"/>')
for gy in range(0, H, 24):
    parts.append(f'<line x1="0" y1="{gy}" x2="{W}" y2="{gy}" stroke="#F0ECE4" stroke-width="1"/>')

text(MX, 52, "ailab", 34, INK, "700")
text(MX + 96, 52, "把神经网络摊开在方格纸上，一格一格算出来", 16, SUB, "400")

CARDS = [
    ("Softmax", "指数归一化", "softmax"),
    ("Backpropagation", "反向传播", "backprop"),
    ("MLP", "多层感知机", "mlp"),
    ("RNN", "循环网络", "rnn"),
    ("LSTM", "门控记忆", "lstm"),
    ("Self-Attention", "自注意力", "attn"),
    ("Transformer", "堆叠编解码", "tf"),
    ("Mamba", "状态空间", "mamba"),
]

def mini(kind, x, y):
    """在 (x,y) 起的 238x176 区域里画示意图。"""
    cx, cy = x, y
    if kind == "softmax":
        hs = [34, 52, 84, 118, 150]
        cols = [BLUE, BLUE, TEAL, ORANGE, RED]
        base = cy + 160
        bw = 32
        for i, (h, c) in enumerate(zip(hs, cols)):
            bx = cx + 12 + i * (bw + 8)
            rect(bx, base - h, bw, h, c, rx=3)
        line(cx + 6, base, cx + 232, base, "#BDC3C7", 2)
        text(cx + 12, cy + 18, "logits  →", 12, SUB, "600")
        text_raw(cx + 78, cy + 18, 'e<tspan dy="-4" font-size="9">x</tspan><tspan dy="4"> → 归一化</tspan>', 12, SUB, "600")
    elif kind == "backprop":
        layers = [(cx + 40, 3, BLUE), (cx + 120, 4, TEAL), (cx + 210, 2, ORANGE)]
        ys = lambda n, i: cy + 60 + i * (100 // max(n - 1, 1))
        pts = []
        for lx, n, c in layers:
            pts.append([(lx, ys(n, i)) for i in range(n)])
        for a, b in zip(pts, pts[1:]):
            for p in a:
                for q in b:
                    line(p[0], p[1], q[0], q[1], "#DDE3EA", 1)
        for (lx, n, c), ps in zip(layers, pts):
            for p in ps:
                circle(p[0], p[1], 9, c)
        for a, b in zip(pts[1:], pts[:-1]):
            line(a[0][0] - 14, a[0][1] - 16, b[0][0] + 14, b[0][1] - 16, RED, 2, dash="6 4", marker="arr")
    elif kind == "mlp":
        layers = [(cx + 44, 3, BLUE), (cx + 124, 5, PURPLE), (cx + 204, 2, GREEN)]
        pts = []
        for lx, n, c in layers:
            step = 120 // max(n - 1, 1)
            pts.append([(lx, cy + 40 + i * step) for i in range(n)])
        for a, b in zip(pts, pts[1:]):
            for p in a:
                for q in b:
                    line(p[0], p[1], q[0], q[1], "#E3E8EE", 1.2)
        for (lx, n, c), ps in zip(layers, pts):
            for p in ps:
                circle(p[0], p[1], 10, c)
        text_raw(cx + 82, cy + 168, 'W<tspan dy="3" font-size="9">1</tspan>', 12, SUB, "700", "middle")
        text_raw(cx + 164, cy + 168, 'W<tspan dy="3" font-size="9">2</tspan>', 12, SUB, "700", "middle")
    elif kind == "rnn":
        rect(cx + 70, cy + 44, 92, 66, TEAL, rx=8)
        text(cx + 116, cy + 82, "h", 18, "white", "700", "middle")
        line(cx + 12, cy + 77, cx + 66, cy + 77, GREY, 2, marker="ar")
        line(cx + 166, cy + 77, cx + 226, cy + 77, GREY, 2, marker="ar")
        path(f"M {cx+92} {cy+110} q 24 34 48 0", stroke=ORANGE, w=2, marker="ar")
        text(cx + 12, cy + 24, "x", 13, SUB, "600")
        text(cx + 206, cy + 24, "y", 13, SUB, "600")
        text(cx + 116, cy + 156, "时间展开", 12, SUB, "600", "middle")
    elif kind == "lstm":
        rect(cx + 14, cy + 40, 210, 92, "#FFFFFF", rx=8, stroke="#DDE3EA")
        line(cx + 14, cy + 76, cx + 224, cy + 76, PURPLE, 3)
        text(cx + 232, cy + 72, "C", 13, PURPLE, "700")
        for i, (gx, c, lb) in enumerate([(cx + 40, RED, "f"), (cx + 96, ORANGE, "i"),
                                         (cx + 152, BLUE, "o"), (cx + 196, GREEN, "g")]):
            rect(gx - 18, cy + 96, 36, 26, c, rx=5)
            text(gx, cy + 114, lb, 13, "white", "700", "middle")
            line(gx, cy + 76, gx, cy + 94, GREY, 1.5)
        text(cx + 14, cy + 158, "遗忘 / 输入 / 输出 / 候选", 11, SUB, "600")
    elif kind == "attn":
        shades = ["#FDEBD0", "#FAD7A0", "#F5B041", "#E67E22"]
        grid = [[0, 1, 3, 2], [1, 3, 2, 1], [3, 2, 1, 0], [2, 1, 0, 1]]
        cell, gap = 36, 4
        for r in range(4):
            for c in range(4):
                rect(cx + 40 + c * (cell + gap), cy + 26 + r * (cell + gap),
                     cell, cell, shades[grid[r][c]], rx=3)
        text(cx + 40, cy + 18, "查询 × 键", 11, SUB, "600")
        text(cx + 40, cy + 196, "得到注意力权重", 11, SUB, "600")
    elif kind == "tf":
        for i in range(3):
            by = cy + 18 + i * 54
            rect(cx + 40, by, 158, 42, "#FFFFFF", rx=6, stroke="#DDE3EA")
            rect(cx + 48, by + 8, 66, 12, BLUE if i % 2 == 0 else TEAL, rx=3)
            rect(cx + 48, by + 24, 96, 10, "#E8EDF2", rx=3)
            line(cx + 116, by + 42, cx + 116, by + 54, GREY, 2)
        text(cx + 206, cy + 44, "× N", 13, SUB, "700")
        line(cx + 12, cy + 39, cx + 36, cy + 39, GREY, 2, marker="ar")
        line(cx + 200, cy + 168, cx + 226, cy + 168, GREY, 2, marker="ar")
    elif kind == "mamba":
        rect(cx + 20, cy + 50, 46, 66, BLUE, rx=6)
        rect(cx + 92, cy + 50, 122, 66, "#FFFFFF", rx=6, stroke="#DDE3EA")
        for i in range(4):
            parts.append(f'<rect x="{cx+100+i*28}" y="{cy+58}" width="24" height="50" rx="3" fill="{["#FAD7A0","#F5B041","#F5A623","#E67E22"][i]}"/>')
        rect(cx + 226 - 46, cy + 50, 46, 66, GREEN, rx=6)
        line(cx + 66, cy + 83, cx + 90, cy + 83, GREY, 2, marker="ar")
        line(cx + 214, cy + 83, cx + 234, cy + 83, GREY, 2, marker="ar")
        text(cx + 20, cy + 32, "x", 13, SUB, "700")
        text(cx + 116, cy + 32, "h", 13, SUB, "700", "middle")
        text(cx + 203, cy + 32, "y", 13, SUB, "700", "middle")
        text(cx + 20, cy + 148, "选择性状态空间", 11, SUB, "600")

for i, (title, sub, kind) in enumerate(CARDS):
    col, row = i % 4, i // 4
    x = MX + col * (CW + GAP)
    y = TOP + row * (CH + GAP)
    rect(x, y, CW, CH, CARD, rx=12, stroke="#E6E1D8", sw=1.5)
    text(x + 16, y + 30, title, 15, INK, "700")
    text(x + 16, y + 50, sub, 11.5, SUB, "500")
    mini(kind, x + 16, y + 58)

svg = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
       '<defs>',
       '<marker id="ar" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">',
       '<path d="M 0 0 L 10 5 L 0 10 z" fill="#95A5A6"/></marker>',
       '<marker id="arr" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">',
       '<path d="M 0 0 L 10 5 L 0 10 z" fill="#E94B3C"/></marker>',
       '</defs>']
svg += parts
svg.append('</svg>')
open('/workspace/ailab/assets/gallery.svg','w').write('\n'.join(svg))
print("gallery.svg 已生成")
