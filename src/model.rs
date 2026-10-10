//! 工作簿的中间表示。
//!
//! 一个练习手册被描述为若干张「方格纸」（[`SheetSpec`]），每张纸由
//! 一个基础样式铺底，再在其上叠加具体的单元格。

use crate::style::Style;

/// 单元格内容的种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// 文本标签。
    Text,
    /// 数值常量。
    Number,
    /// 普通公式。
    Formula,
    /// 动态数组公式（会向相邻单元格溢出结果）。
    DynFormula,
    /// 只画格式、不写内容的空格（例如留给学生作答的输入格）。
    Blank,
}

/// 一个单元格。
///
/// 行列编号均为 **1 起算**，与表格软件中看到的坐标一致，便于对照版面校对。
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub row: u16,
    pub col: u16,
    pub kind: Kind,
    /// `Text` / `Formula` / `DynFormula` 使用。
    pub text: &'static str,
    /// `Number` 使用。
    pub num: f64,
    /// 样式表索引。
    pub style: u16,
}

/// 列宽定义，含起止列（1 起算）。
#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub min: u16,
    pub max: u16,
    pub width: f64,
}

/// 一张工作表。
#[derive(Debug, Clone, Copy)]
pub struct SheetSpec {
    pub name: &'static str,
    /// 网格行数（1 起算，即铺到第 `rows` 行）。
    pub rows: u16,
    /// 网格列数。
    pub cols: u16,
    /// 铺底样式在样式表中的索引。
    pub base: u16,
    /// 列宽定义。
    pub cols_widths: &'static [Col],
    /// 合并区域，形如 `B4:E4`。
    pub merges: &'static [&'static str],
    pub cells: &'static [Cell],
}

/// 一份工作簿（对应磁盘上的一个文件）。
#[derive(Debug, Clone, Copy)]
pub struct BookSpec {
    /// 相对输出根目录的路径，如 `advanced/LSTM.xlsx`。
    pub path: &'static str,
    pub sheets: &'static [SheetSpec],
}
