//! 单元格样式描述与格式转换。
//!
//! 版面数据里只记录样式的「语义」（颜色、填充、字号……），渲染时再翻译成
//! 具体表格文件的格式对象。这样同一份数据可以为不同后端复用。

use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

/// 水平对齐方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// 一个单元格样式的完整描述。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 字体颜色，形如 `FFC00000`；`None` 表示使用默认色。
    pub color: Option<&'static str>,
    /// 背景填充色，形如 `FFFFFF00`；`None` 表示不填充。
    pub fill: Option<&'static str>,
    /// 字号（磅）。
    pub size: Option<f64>,
    pub bold: bool,
    /// 数字格式代码，如 `0.00`。
    pub numfmt: Option<&'static str>,
    pub align: Option<Align>,
    pub wrap: bool,
}

impl Style {
    /// 把 `FFRRGGBB` / `RRGGBB` 形式的颜色串解析为 RGB 值。
    fn parse_color(hex: &str) -> Option<Color> {
        let hex = hex.trim_start_matches('#');
        let hex = if hex.len() == 8 { &hex[2..] } else { hex };
        if hex.len() != 6 {
            return None;
        }
        u32::from_str_radix(hex, 16).ok().map(Color::RGB)
    }

    /// 渲染为表格文件格式。
    ///
    /// `grid` 为真时额外绘制四周边框 —— 方格纸效果正是由每一格的细边框拼出来的。
    pub fn to_format(&self, grid: bool) -> Format {
        let mut f = Format::new();
        f = f.set_font_name("Calibri");

        if let Some(size) = self.size {
            f = f.set_font_size(size);
        } else {
            f = f.set_font_size(10.0);
        }
        if self.bold {
            f = f.set_bold();
        }
        if let Some(c) = self.color.and_then(Self::parse_color) {
            f = f.set_font_color(c);
        }
        if let Some(c) = self.fill.and_then(Self::parse_color) {
            f = f.set_background_color(c);
        }
        if let Some(code) = self.numfmt {
            f = f.set_num_format(code);
        }
        if let Some(a) = self.align {
            f = f.set_align(match a {
                Align::Left => FormatAlign::Left,
                Align::Center => FormatAlign::Center,
                Align::Right => FormatAlign::Right,
            });
        }
        if self.wrap {
            f = f.set_text_wrap();
        }
        if grid {
            f = f
                .set_border(FormatBorder::Thin)
                .set_border_color(Color::RGB(0xD0D0D0));
        }
        f
    }
}
