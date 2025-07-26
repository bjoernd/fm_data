use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use table_extract::Table;

pub fn read_table(html_file: &str) -> Result<Table> {
    let html_content = fs::read_to_string(Path::new(html_file))
        .with_context(|| format!("Error reading HTML file {}", html_file))?;

    Table::find_first(&html_content)
        .ok_or_else(|| anyhow::anyhow!("No table found in the provided HTML document"))
}

pub fn validate_table_structure(table: &Table) -> Result<()> {
    if table.iter().count() == 0 {
        return Err(anyhow::anyhow!("Table is empty"));
    }

    let first_row = table.iter().next().unwrap();
    let first_row_len = first_row.len();

    for (i, row) in table.iter().enumerate() {
        if row.len() != first_row_len {
            return Err(anyhow::anyhow!(
                "Inconsistent row length: row {} has {} columns, expected {}",
                i,
                row.len(),
                first_row_len
            ));
        }
    }

    Ok(())
}

pub fn process_table_data(table: &Table) -> Vec<Vec<String>> {
    let mut matrix = vec![];
    for row in table {
        let mut line = vec![];
        for cell in row {
            let value = match cell.as_str() {
                "Left" | "Left Only" => "l",
                "Right" | "Right Only" => "r",
                "Either" => "rl",
                "-" => "0",
                _ => cell,
            };
            line.push(String::from(value))
        }
        matrix.push(line);
    }
    matrix
}

pub fn validate_data_size(row_count: usize) -> Result<()> {
    const MAX_DATA_ROWS: usize = 57;
    if row_count > MAX_DATA_ROWS {
        return Err(anyhow::anyhow!(
            "Data has {} rows but maximum allowed is {} rows (hardcoded range limit)",
            row_count,
            MAX_DATA_ROWS
        ));
    }
    Ok(())
}