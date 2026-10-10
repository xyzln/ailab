//! 许可声明。
//!
//! 每份生成的工作簿末尾都会附带一张许可页，说明这些练习材料可以如何使用。

/// 许可页的标题。
pub const TITLE: &str = "MIT License";

/// 版权持有人。若你要以自己的名义发布，改这里即可。
pub const HOLDER: &str = "xyzln";

/// 许可页正文，按行给出。
pub fn lines() -> Vec<String> {
    let text = format!(
        "MIT License\n\
         \n\
         Copyright (c) 2026 {holder}\n\
         \n\
         This work is based on ai-by-hand-excel\n\
         <https://github.com/ImagineAILab/ai-by-hand-excel>,\n\
         Copyright (c) 2024 Tom Yeh, licensed under the MIT License.\n\
         \n\
         Permission is hereby granted, free of charge, to any person obtaining a copy\n\
         of this software and associated documentation files (the \"Software\"), to deal\n\
         in the Software without restriction, including without limitation the rights\n\
         to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\
         copies of the Software, and to permit persons to whom the Software is\n\
         furnished to do so, subject to the following conditions:\n\
         \n\
         The above copyright notice and this permission notice shall be included in all\n\
         copies or substantial portions of the Software.\n\
         \n\
         THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\
         IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\
         FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n\
         AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n\
         LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n\
         OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n\
         SOFTWARE.",
        holder = HOLDER
    );
    text.split('\n').map(|s| s.to_string()).collect()
}
