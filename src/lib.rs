//! `ailab` —— 用表格手算神经网络的练习手册生成器。
//!
//! 全部练习以「方格纸」的形式描述：一个基础样式铺底形成网格，
//! 再在其上叠加标签、常量与公式。运行生成器即可批量产出可直接在
//! 表格软件中打开的练习工作簿。

pub mod cell;
pub mod exercises;
pub mod license;
pub mod model;
pub mod render;
pub mod style;
