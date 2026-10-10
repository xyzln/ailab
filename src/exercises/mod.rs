//! 练习手册注册表。
//!
//! 每一份手册对应 `data` 下的一个模块，模块里是纯静态的版面数据；
//! 这里只负责把它们归类、串成可遍历的清单。

pub mod data {
    #![allow(unused_imports)]

    pub mod softmax;
    pub mod leaky_relu;
    pub mod temperature;
    pub mod dot_product;
    pub mod matrix_multiplication;
    pub mod linear_layer;
    pub mod alphafold;
    pub mod autoencoder;
    pub mod backpropagation;
    pub mod lstm;
    pub mod mamba;
    pub mod multihead_attention;
    pub mod mlp;
    pub mod resnet;
    pub mod rnn;
    pub mod self_attention;
    pub mod transformer_full_stack;
    pub mod transformer;
    pub mod xlstm;
    pub mod deepseek;
    pub mod style_table;
}

use crate::model::BookSpec;

/// 基础类练习。
pub static BASIC: &[BookSpec] = &[
    BookSpec { path: data::softmax::OUT, sheets: data::softmax::SHEETS },
    BookSpec { path: data::leaky_relu::OUT, sheets: data::leaky_relu::SHEETS },
    BookSpec { path: data::temperature::OUT, sheets: data::temperature::SHEETS },
];

/// 练习册类练习。
pub static WORKBOOK: &[BookSpec] = &[
    BookSpec { path: data::dot_product::OUT, sheets: data::dot_product::SHEETS },
    BookSpec { path: data::matrix_multiplication::OUT, sheets: data::matrix_multiplication::SHEETS },
    BookSpec { path: data::linear_layer::OUT, sheets: data::linear_layer::SHEETS },
];

/// 进阶类练习。
pub static ADVANCED: &[BookSpec] = &[
    BookSpec { path: data::alphafold::OUT, sheets: data::alphafold::SHEETS },
    BookSpec { path: data::autoencoder::OUT, sheets: data::autoencoder::SHEETS },
    BookSpec { path: data::backpropagation::OUT, sheets: data::backpropagation::SHEETS },
    BookSpec { path: data::lstm::OUT, sheets: data::lstm::SHEETS },
    BookSpec { path: data::mamba::OUT, sheets: data::mamba::SHEETS },
    BookSpec { path: data::multihead_attention::OUT, sheets: data::multihead_attention::SHEETS },
    BookSpec { path: data::mlp::OUT, sheets: data::mlp::SHEETS },
    BookSpec { path: data::resnet::OUT, sheets: data::resnet::SHEETS },
    BookSpec { path: data::rnn::OUT, sheets: data::rnn::SHEETS },
    BookSpec { path: data::self_attention::OUT, sheets: data::self_attention::SHEETS },
    BookSpec { path: data::transformer_full_stack::OUT, sheets: data::transformer_full_stack::SHEETS },
    BookSpec { path: data::transformer::OUT, sheets: data::transformer::SHEETS },
    BookSpec { path: data::xlstm::OUT, sheets: data::xlstm::SHEETS },
];

/// 专题类练习。
pub static LECTURES: &[BookSpec] = &[
    BookSpec { path: data::deepseek::OUT, sheets: data::deepseek::SHEETS },
];

/// 全部练习手册。
pub fn all() -> impl Iterator<Item = &'static BookSpec> {
    BASIC.iter().chain(WORKBOOK).chain(ADVANCED).chain(LECTURES)
}

/// 样式表：数据模块中的 `style` 字段都是它的下标。
pub fn styles() -> &'static [crate::style::Style] {
    data::style_table::STYLES
}
