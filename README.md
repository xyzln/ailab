# ailab

**把神经网络拆成一张张可以手算的表格。**

看公式和真正算一遍，是两件事。`ailab` 提供 20 份练习手册，每一份都把一种网络结构摊开在
网格纸上：权重摆在左边，中间量留空，你一格一格把数字填进去。填完后表格会自己告诉你做得
对不对。

没有框架，没有自动求导，只有一个个你能亲眼看着算出来的数。

![手册涵盖的网络结构](assets/gallery.svg)

---

## 为什么要手算

反向传播真正难住人的地方，从来不是「梯度要链式相乘」这句话，而是当矩阵从 3 层堆到 8 层、
维度从 2 涨到 24 的时候，你还能不能盯住每一个下标。

把这些步骤摊在网格里，好处很直接：

- **每一步都看得见。** 中间结果就躺在相邻的格子里，不用在脑子里维护调用栈。
- **错了立刻知道。** 手册自带判题公式，填错会当场标出来，不用等到 loss 炸了才回头找。
- **规模可以自己挑。** 同一结构备了 small / medium / large 三档，先在小尺寸上把流程走通，
  再去大尺寸上验证自己是真的理解了，而不只是记住了形状。

## 快速开始

```bash
git clone https://github.com/xyzln/ailab.git
cd ailab

cargo run --release          # 生成全部手册到 ./out
cargo run --release -- -g basic     # 只生成基础篇
cargo run --release -- -l           # 列出全部手册
```

生成结果按类别落在 `out/` 下：

```
out/
├── basic/      基础算子
├── workbook/   分题练习册
├── advanced/   完整网络结构
└── lectures/   专题
```

仓库根目录已经放好了一份生成好的成品，不想装工具链的话，直接下载 `.xlsx` 打开即可。

## 手册清单

### 基础篇 · basic

| 手册 | 内容 |
| --- | --- |
| `Softmax` | 指数归一化，含用 3 代替 e 的简化版 |
| `LeakyReLU` | ReLU 与 LeakyReLU 的逐元素对照 |
| `Temperature` | 温度系数如何改变采样分布 |

### 练习册 · workbook

每册 25 道题，做完一页对照一页。

| 手册 | 内容 |
| --- | --- |
| `W1_Dot-Product` | 向量点积 |
| `W2_Matrix-Multiplication` | 矩阵乘法 |
| `W3_Linear-Layer` | 线性层 |

### 进阶篇 · advanced

| 手册 | 内容 |
| --- | --- |
| `Multi-Layer-Perceptron` | 多层感知机，三档尺寸 |
| `Backpropagation` | 三层网络完整反向传播 |
| `RNN` | 循环网络的时间展开 |
| `LSTM` | 长短期记忆，含 seq2seq |
| `xLSTM` | 扩展 LSTM（mLSTM） |
| `ResNet` | 残差连接及其配套 MLP |
| `Self-Attention` | 自注意力，三档尺寸 |
| `Multihead-Attention` | 多头注意力 |
| `Transformer` | Transformer 编解码结构 |
| `Transformer-Full-Stack` | 完整堆叠版 Transformer |
| `Autoencoder` | 自编码器 |
| `Mamba` | 状态空间模型与离散化 |
| `AlphaFold` | MSA 行门控注意力与配对偏置 |

### 专题 · lectures

| 手册 | 内容 |
| --- | --- |
| `DeepSeek` | 多头潜在注意力 + 混合专家 |

![DeepSeek 结构：多头潜在注意力与混合专家](assets/deepseek.svg)

## 怎么用

1. **挑一份**。第一次上手建议从 `basic/Softmax.xlsx` 开始；想系统练计算，走
   `workbook/W1_Dot-Product.xlsx`。
2. **只填空的格子**。带底色、没有公式的格子是留给你写的。其余格子是题目给定的条件。
3. **照着箭头走**。格子里的小箭头标示数据流向，顺着走就不会跳步。
4. **看判题格**。练习册每道题上方有一格反馈，会显示 `CORRECT` / `INCORRECT` / `NOT DONE`。

> 需要 Microsoft 365 或同版本以上的表格软件：`Temperature`、`xLSTM`、`Multihead-Attention`
> 几份用到了 `LET`、`SORT`、`SEQUENCE` 这类动态数组函数，旧版本打不开。

## 它是怎么搭起来的

手册不是手敲出来的，而是由 Rust 程序生成的。这样做的好处是：改一处布局，20 份手册同时生效；
换一套配色，也只动一个地方。

```
src/
├── main.rs          命令行入口
├── model.rs         工作簿的中间表示（方格纸 + 单元格）
├── style.rs         样式描述与格式转换
├── render.rs        渲染成 .xlsx
├── license.rs       许可页文本
└── exercises/
    ├── mod.rs       手册注册表
    └── data/        各手册的版面数据
```

一张工作表被描述为：先用基础样式铺一张方格纸，再往上叠加标签、常量和公式。单元格用
`(行, 列)` 定位，行列均从 1 起算，和你在表格软件里看到的坐标一致，方便对照校对。

想加一份新手册，照着 `src/exercises/data/` 下任一模块的格式写一份版面数据，再到
`exercises/mod.rs` 里登记一行就行。

`assets/` 下的示意图由 `tools/` 里的脚本生成（`python3 tools/draw_gallery.py`），
改配色或增删结构条目后重跑即可，不依赖任何外部服务。

## 许可

MIT。每份生成的手册末尾都附有一张许可页，单独分发时也带着许可信息。
