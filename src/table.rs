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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_html_file(html_content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(html_content.as_bytes()).unwrap();
        temp_file
    }

    #[test]
    fn test_read_table_valid_html() -> Result<()> {
        let html_content = r#"
            <html>
                <body>
                    <table>
                        <tr><td>Name</td><td>Age</td><td>Position</td></tr>
                        <tr><td>Player1</td><td>25</td><td>Left</td></tr>
                        <tr><td>Player2</td><td>30</td><td>Right</td></tr>
                    </table>
                </body>
            </html>
        "#;

        let temp_file = create_test_html_file(html_content);
        let table = read_table(temp_file.path().to_str().unwrap())?;

        let row_count = table.iter().count();
        assert!(row_count >= 2); // At least 2 data rows
        
        let first_row: Vec<String> = table.iter().next().unwrap().iter().map(|s| s.to_string()).collect();
        assert_eq!(first_row, vec!["Name", "Age", "Position"]);

        Ok(())
    }

    #[test]
    fn test_read_table_no_table() {
        let html_content = r#"
            <html>
                <body>
                    <p>No table here</p>
                </body>
            </html>
        "#;

        let temp_file = create_test_html_file(html_content);
        let result = read_table(temp_file.path().to_str().unwrap());
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No table found"));
    }

    #[test]
    fn test_read_table_nonexistent_file() {
        let result = read_table("/nonexistent/file.html");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Error reading HTML file"));
    }

    #[test]
    fn test_validate_table_structure_valid() -> Result<()> {
        let html_content = r#"
            <table>
                <tr><td>A</td><td>B</td><td>C</td></tr>
                <tr><td>1</td><td>2</td><td>3</td></tr>
                <tr><td>X</td><td>Y</td><td>Z</td></tr>
            </table>
        "#;

        let temp_file = create_test_html_file(html_content);
        let table = read_table(temp_file.path().to_str().unwrap())?;
        
        // Should not error
        validate_table_structure(&table)?;
        Ok(())
    }

    #[test]
    fn test_validate_table_structure_inconsistent_columns() -> Result<()> {
        let html_content = r#"
            <table>
                <tr><td>A</td><td>B</td><td>C</td></tr>
                <tr><td>1</td><td>2</td></tr>
                <tr><td>X</td><td>Y</td><td>Z</td></tr>
            </table>
        "#;

        let temp_file = create_test_html_file(html_content);
        let table = read_table(temp_file.path().to_str().unwrap())?;
        
        let result = validate_table_structure(&table);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Inconsistent row length"));
        Ok(())
    }

    #[test]
    fn test_process_table_data_transformations() -> Result<()> {
        let html_content = r#"
            <table>
                <tr><td>Left</td><td>Right</td><td>Either</td><td>-</td><td>Normal</td></tr>
                <tr><td>Left Only</td><td>Right Only</td><td>Either</td><td>-</td><td>Value</td></tr>
            </table>
        "#;

        let temp_file = create_test_html_file(html_content);
        let table = read_table(temp_file.path().to_str().unwrap())?;
        
        let processed = process_table_data(&table);
        
        assert_eq!(processed.len(), 2);
        assert_eq!(processed[0], vec!["l", "r", "rl", "0", "Normal"]);
        assert_eq!(processed[1], vec!["l", "r", "rl", "0", "Value"]);
        
        Ok(())
    }

    #[test]
    fn test_process_table_data_no_transformations() -> Result<()> {
        let html_content = r#"
            <table>
                <tr><td>Name</td><td>Age</td><td>Score</td></tr>
                <tr><td>Player1</td><td>25</td><td>100</td></tr>
            </table>
        "#;

        let temp_file = create_test_html_file(html_content);
        let table = read_table(temp_file.path().to_str().unwrap())?;
        
        let processed = process_table_data(&table);
        
        assert_eq!(processed.len(), 2);
        assert_eq!(processed[0], vec!["Name", "Age", "Score"]);
        assert_eq!(processed[1], vec!["Player1", "25", "100"]);
        
        Ok(())
    }

    #[test]
    fn test_validate_data_size_valid() {
        assert!(validate_data_size(1).is_ok());
        assert!(validate_data_size(57).is_ok()); // max allowed
        assert!(validate_data_size(30).is_ok());
    }

    #[test]
    fn test_validate_data_size_too_large() {
        let result = validate_data_size(58);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("maximum allowed is 57 rows"));
        
        let result = validate_data_size(100);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hardcoded range limit"));
    }

    #[test]
    fn test_validate_data_size_edge_cases() {
        assert!(validate_data_size(0).is_ok());
        assert!(validate_data_size(1).is_ok());
        assert!(validate_data_size(57).is_ok());
        assert!(validate_data_size(58).is_err());
    }
}