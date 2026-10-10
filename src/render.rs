//! 把中间表示渲染为真实的表格文件。

use rust_xlsxwriter::{Workbook, Worksheet, XlsxError};

use crate::model::{BookSpec, Kind, SheetSpec};
use crate::style::Style;

/// 渲染一份工作簿。
pub fn render_book(book: &BookSpec, styles: &[Style]) -> Result<Workbook, XlsxError> {
    let mut wb = Workbook::new();
    for sheet in book.sheets {
        let ws = wb.add_worksheet();
        render_sheet(ws, sheet, styles)?;
    }
    render_license(&mut wb)?;
    Ok(wb)
}

/// 追加许可页 —— 每份手册都自带一份，方便单独分发。
fn render_license(wb: &mut Workbook) -> Result<(), XlsxError> {
    let ws = wb.add_worksheet();
    ws.set_name("LICENSE")?;
    ws.set_column_width(0, 116.0)?;

    let body = Style {
        color: None,
        fill: None,
        size: Some(10.0),
        bold: false,
        numfmt: None,
        align: None,
        wrap: false,
    };
    let head = Style { bold: true, size: Some(12.0), ..body };
    let body_fmt = body.to_format(false);
    let head_fmt = head.to_format(false);

    for (i, line) in crate::license::lines().iter().enumerate() {
        let fmt = if i == 0 { &head_fmt } else { &body_fmt };
        ws.write_string_with_format(i as u32, 0, line, fmt)?;
    }
    Ok(())
}

/// 渲染一张工作表：先铺方格纸底，再逐个叠加单元格。
fn render_sheet(
    ws: &mut Worksheet,
    sheet: &SheetSpec,
    styles: &[Style],
) -> Result<(), XlsxError> {
    ws.set_name(sheet.name)?;

    let fallback = Style {
        color: None,
        fill: None,
        size: Some(10.0),
        bold: false,
        numfmt: None,
        align: None,
        wrap: false,
    };
    let base = styles.get(sheet.base as usize).copied().unwrap_or(fallback);
    let grid_format = base.to_format(true);

    for c in sheet.cols_widths {
        for col in c.min..=c.max {
            ws.set_column_width(col - 1, c.width)?;
        }
    }

    // 方格纸：在内容矩形内逐格打底。
    for row in 0..sheet.rows as u32 {
        for col in 0..sheet.cols {
            ws.write_blank(row, col, &grid_format)?;
        }
    }

    for cell in sheet.cells {
        write_cell(ws, cell, styles, base)?;
    }

    // 合并区域放在最后。`merge_range` 会清空整片区域，所以合并后要把
    // 左上角原本的内容重新写回去，否则标题文字会被抹掉。
    for m in sheet.merges {
        let Some((r1, c1, r2, c2)) = parse_ref(m) else { continue };
        ws.merge_range(r1, c1, r2, c2, "", &grid_format)?;
        if let Some(cell) = sheet
            .cells
            .iter()
            .find(|c| c.row.saturating_sub(1) as u32 == r1 && c.col.saturating_sub(1) == c1)
        {
            write_cell(ws, cell, styles, base)?;
        }
    }

    Ok(())
}

/// 按单元格种类写入一个格子。
fn write_cell(
    ws: &mut Worksheet,
    cell: &crate::model::Cell,
    styles: &[Style],
    base: Style,
) -> Result<(), XlsxError> {
    let row = cell.row.saturating_sub(1) as u32;
    let col = cell.col.saturating_sub(1);
    let st = styles.get(cell.style as usize).copied().unwrap_or(base);
    let fmt = st.to_format(true);

    match cell.kind {
        Kind::Text => {
            ws.write_string_with_format(row, col, cell.text, &fmt)?;
        }
        Kind::Number => {
            ws.write_number_with_format(row, col, cell.num, &fmt)?;
        }
        Kind::Formula => {
            // 空公式是留给学习者填写的位置，只保留格子。
            let formula = cell.text.trim();
            if formula.is_empty() {
                ws.write_blank(row, col, &fmt)?;
            } else {
                let f = prefix_formula(formula);
                ws.write_formula_with_format(row, col, f.as_str(), &fmt)?;
            }
        }
        Kind::DynFormula => {
            let formula = cell.text.trim();
            if formula.is_empty() {
                ws.write_blank(row, col, &fmt)?;
            } else {
                let f = prefix_formula(formula);
                ws.write_dynamic_formula_with_format(row, col, f.as_str(), &fmt)?;
            }
        }
        Kind::Blank => {
            ws.write_blank(row, col, &fmt)?;
        }
    }
    Ok(())
}

/// 补上前导 `=`。
fn prefix_formula(formula: &str) -> String {
    if formula.starts_with('=') {
        formula.to_string()
    } else {
        format!("={formula}")
    }
}

/// 解析 `B4:E4` 形式的区域，返回 0 起算的 (row1, col1, row2, col2)。
fn parse_ref(range: &str) -> Option<(u32, u16, u32, u16)> {
    let (a, b) = range.split_once(':').unwrap_or((range, range));
    let (r1, c1) = parse_cell(a)?;
    let (r2, c2) = if b.is_empty() { (r1, c1) } else { parse_cell(b)? };
    Some((r1, c1, r2, c2))
}

fn parse_cell(cell: &str) -> Option<(u32, u16)> {
    let col0 = 0u32;
    let mut col = col0;
    let mut i = 0;
    let bytes = cell.as_bytes();
    while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
        col = col * 26 + (bytes[i].to_ascii_uppercase() as u32 - b'A' as u32 + 1);
        i += 1;
    }
    let row: u32 = cell[i..].parse().ok()?;
    if col == 0 {
        return None;
    }
    Some((row.saturating_sub(1), (col - 1) as u16))
}
