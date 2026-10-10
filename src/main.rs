//! 命令行入口：批量生成练习工作簿。
//!
//! ```text
//! ailab                      # 生成全部手册到 ./out
//! ailab --out dist           # 指定输出目录
//! ailab --group advanced     # 只生成某一类
//! ailab --list               # 列出全部手册
//! ```

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use ailab::exercises;
use ailab::render::render_book;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut out = PathBuf::from("out");
    let mut group = String::new();
    let mut list_only = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "-o" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    out = PathBuf::from(v);
                }
            }
            "--group" | "-g" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    group = v.clone();
                }
            }
            "--list" | "-l" => list_only = true,
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            other => {
                eprintln!("无法识别的参数：{other}\n");
                print_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if list_only {
        for book in exercises::all() {
            println!("{}", book.path);
        }
        return Ok(());
    }

    let books: Vec<_> = exercises::all()
        .filter(|b| group.is_empty() || b.path.starts_with(&format!("{group}/")))
        .collect();

    if books.is_empty() {
        eprintln!("没有匹配的手册（group = {group:?}）");
        std::process::exit(1);
    }

    let styles = exercises::styles();
    let total = books.len();
    for book in books {
        let dest = out.join(Path::new(book.path));
        if let Some(dir) = dest.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut wb = render_book(book, styles)?;
        wb.save(&dest)?;
        println!("已生成 {}", dest.display());
    }
    println!("\n共 {total} 份手册，输出目录：{}", out.display());
    Ok(())
}

fn print_help() {
    println!(
        "ailab —— 生成可在表格软件中手算的神经网络练习手册\n\
         \n\
         用法：\n\
         \x20 ailab [选项]\n\
         \n\
         选项：\n\
         \x20 -o, --out <目录>     输出目录，默认 out\n\
         \x20 -g, --group <类别>   只生成某一类：basic | workbook | advanced | lectures\n\
         \x20 -l, --list           列出全部手册路径\n\
         \x20 -h, --help           显示本帮助\n"
    );
}
