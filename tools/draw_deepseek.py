# -*- coding: utf-8 -*-
"""DeepSeek 结构示意图：多头潜在注意力 + 混合专家。"""
W, H = 1200, 560
FONT = "Noto Sans CJK SC, Helvetica, Arial, sans-serif"
BG, CARD, INK, SUB = "#FDFBF7", "#FFFFFF", "#2C3E50", "#8A94A6"
BLUE, ORANGE, GREEN, RED = "#4A90D9", "#F5A623", "#5CB85C", "#E94B3C"
PURPLE, TEAL, GREY = "#9B59B6", "#1ABC9C", "#95A5A6"

P = []
def rect(x, y, w, h, fill, rx=6, stroke=None, sw=1.5, op=None):
    s = f' stroke="{stroke}" stroke-width="{sw}"' if stroke else ""
    o = f' opacity="{op}"' if op else ""
    P.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{rx}" fill="{fill}"{s}{o}/>')
def line(x1, y1, x2, y2, c=GREY, w=2, dash=None, marker=None):
    d = f' stroke-dasharray="{dash}"' if dash else ""
    m = f' marker-end="url(#{marker})"' if marker else ""
    P.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{c}" stroke-width="{w}"{d}{m}/>')
def text(x, y, s, size=13, fill=INK, weight="600", anchor="start"):
    e = s.replace("&","&amp;").replace("<","&lt;").replace(">","&gt;")
    P.append(f'<text x="{x}" y="{y}" font-family="{FONT}" font-size="{size}" fill="{fill}" font-weight="{weight}" text-anchor="{anchor}">{e}</text>')
def text_raw(x, y, inner, size=13, fill=INK, weight="600", anchor="start"):
    P.append(f'<text x="{x}" y="{y}" font-family="{FONT}" font-size="{size}" fill="{fill}" font-weight="{weight}" text-anchor="{anchor}">{inner}</text>')

# 背景
rect(0, 0, W, H, BG)
for gx in range(0, W, 24):
    P.append(f'<line x1="{gx}" y1="0" x2="{gx}" y2="{H}" stroke="#F0ECE4" stroke-width="1"/>')
for gy in range(0, H, 24):
    P.append(f'<line x1="0" y1="{gy}" x2="{W}" y2="{gy}" stroke="#F0ECE4" stroke-width="1"/>')

text(30, 50, "DeepSeek", 30, INK, "700")
text(30 + 168, 50, "用更少的显存，装下更大的模型", 15, SUB, "400")

# ---------------- 左：MLA ----------------
rect(30, 88, 550, 420, CARD, rx=14, stroke="#E6E1D8")
text(56, 126, "多头潜在注意力", 17, INK, "700")
text(56, 148, "先把 KV 压进低维潜向量，再解压成多头", 12, SUB, "500")

# 输入向量
rect(66, 236, 46, 104, BLUE, rx=6)
for i in range(4):
    P.append(f'<circle cx="{89}" cy="{254+i*24}" r="5" fill="#FFFFFF" opacity="0.75"/>')
text(89, 222, "x", 13, SUB, "700", "middle")

line(116, 288, 152, 288, GREY, 2, marker="ar")

# 压缩
rect(156, 222, 44, 132, "#FFF4E0", rx=8, stroke=ORANGE, sw=1.8)
text(178, 288, "W", 15, ORANGE, "700", "middle")
text_raw(178, 310, '<tspan font-size="10">DKV</tspan>', 10, ORANGE, "700", "middle")
text(178, 206, "压缩", 11, SUB, "600", "middle")

line(204, 288, 240, 288, GREY, 2, marker="ar")

# 潜向量
rect(244, 250, 40, 76, ORANGE, rx=6)
text(264, 238, "c", 13, SUB, "700", "middle")
text(264, 344, "潜向量", 11, SUB, "600", "middle")

line(288, 288, 324, 288, GREY, 2, marker="ar")

# 解压
rect(328, 222, 44, 132, "#FFF4E0", rx=8, stroke=ORANGE, sw=1.8)
text(350, 288, "W", 15, ORANGE, "700", "middle")
text_raw(350, 310, '<tspan font-size="10">UK</tspan>', 10, ORANGE, "700", "middle")
text(350, 206, "解压", 11, SUB, "600", "middle")

line(376, 288, 412, 288, GREY, 2, marker="ar")

# 多头：一个容器分成 Q/K/V 三段
text(433, 206, "多头", 11, SUB, "600", "middle")
for i, (c, lb) in enumerate([(BLUE, "Q"), (TEAL, "K"), (PURPLE, "V")]):
    seg_y = 222 + i * 46
    rect(416, seg_y, 34, 42, c, rx=5)
    text(433, seg_y + 27, lb, 13, "#FFFFFF", "700", "middle")

line(454, 288, 490, 288, GREY, 2, marker="ar")

# 输出
rect(494, 236, 46, 104, GREEN, rx=6)
for i in range(4):
    P.append(f'<circle cx="{517}" cy="{254+i*24}" r="5" fill="#FFFFFF" opacity="0.75"/>')
text(517, 222, "y", 13, SUB, "700", "middle")

text(305, 466, "键值只留一份潜向量，用到时才解压", 12, SUB, "500", "middle")

# ---------------- 右：MoE ----------------
rect(620, 88, 550, 420, CARD, rx=14, stroke="#E6E1D8")
text(646, 126, "混合专家", 17, INK, "700")
text(646, 148, "每个词只叫醒少数几个专家", 12, SUB, "500")

rect(654, 262, 46, 60, BLUE, rx=6)
text(677, 248, "x", 13, SUB, "700", "middle")
line(704, 292, 738, 292, GREY, 2, marker="ar")

rect(742, 262, 62, 60, "#EAF2FB", rx=8, stroke=BLUE, sw=1.8)
text(773, 298, "路由", 14, BLUE, "700", "middle")
text(773, 246, "专家打分", 10.5, SUB, "600", "middle")

# 六个专家，两个激活
experts = [
    (840, 150, True), (840, 218, False), (840, 286, True),
    (958, 150, False), (958, 218, False), (958, 286, False),
]
for ex, ey, on in experts:
    fill = ORANGE if on else "#F1F3F6"
    stro = "#E0A04A" if on else "#E2E6EC"
    rect(ex, ey, 92, 52, fill, rx=8, stroke=stro, sw=1.6)
    text(ex + 46, ey + 32, "专家", 12.5, "#FFFFFF" if on else "#A8B2BF", "700", "middle")
    line(806 + (0 if ex < 900 else 0), 292, ex - 4, ey + 26, "#D8DFE6", 1.6)
for ex, ey, on in experts:
    if on:
        line(ex + 92, ey + 26, 1076, 292, ORANGE, 2.2, marker="ar")

line(1076, 292, 1088, 292, GREY, 2, marker="ar")
rect(1088, 262, 46, 60, GREEN, rx=6)
text(1111, 298, "y", 13, "#FFFFFF", "700", "middle")
text(1056, 384, "只激活得分最高的少数专家，其余不参与计算", 11.5, SUB, "600", "middle")

P.append('</svg>')
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
       '<defs><marker id="ar" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">',
       '<path d="M 0 0 L 10 5 L 0 10 z" fill="#95A5A6"/></marker></defs>']
svg += P
open('/workspace/ailab/assets/deepseek.svg','w').write('\n'.join(svg))
print("deepseek.svg 已生成")
