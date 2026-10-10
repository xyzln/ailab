//! 单元格的紧凑构造函数。
//!
//! 版面数据文件里有几万个单元格，用结构体字面量写会非常冗长，
//! 因此提供单字母别名：`t` 文本、`n` 数值、`f` 公式、`d` 动态数组公式、`b` 空格。

use crate::model::{Cell, Kind};

/// 文本单元格。
pub const fn t(row: u16, col: u16, text: &'static str, style: u16) -> Cell {
    Cell { row, col, kind: Kind::Text, text, num: 0.0, style }
}

/// 数值单元格。
pub const fn n(row: u16, col: u16, num: f64, style: u16) -> Cell {
    Cell { row, col, kind: Kind::Number, text: "", num, style }
}

/// 普通公式单元格（公式文本不含前导 `=`）。
pub const fn f(row: u16, col: u16, formula: &'static str, style: u16) -> Cell {
    Cell { row, col, kind: Kind::Formula, text: formula, num: 0.0, style }
}

/// 动态数组公式单元格，结果会溢出到相邻格。
pub const fn d(row: u16, col: u16, formula: &'static str, style: u16) -> Cell {
    Cell { row, col, kind: Kind::DynFormula, text: formula, num: 0.0, style }
}

/// 仅绘制格式的空格。
pub const fn b(row: u16, col: u16, style: u16) -> Cell {
    Cell { row, col, kind: Kind::Blank, text: "", num: 0.0, style }
}
