use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info, warn, error};
use sheets::{
    self,
    types::{
        ClearValuesRequest, DateTimeRenderOption, Dimension, ValueInputOption, ValueRange,
        ValueRenderOption,
    },
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use table_extract::Table;
use tokio;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

fn read_table(html_file: &str) -> Result<Table> {
    let html_content = fs::read_to_string(Path::new(html_file))
        .with_context(|| format!("Error reading HTML file {}", html_file))?;

    Table::find_first(&html_content)
        .ok_or_else(|| anyhow::anyhow!("No table found in the provided HTML document"))
}

fn validate_table_structure(table: &Table) -> Result<()> {
    // Check if table has at least one row
    if table.is_empty() {
        return Err(anyhow::anyhow!("Table is empty"));
    }
    
    // Check if all rows have consistent number of columns
    let first_row_len = table[0].len();
    for (i, row) in table.iter().enumerate() {
        if row.len() != first_row_len {
            return Err(anyhow::anyhow!(
                "Inconsistent row length: row {} has {} columns, expected {}",
                i, row.len(), first_row_len
            ));
        }
    }
    
    Ok(())
}

// Default paths that will be overridden by config or CLI args
fn get_default_paths() -> (String, String, String) {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    
    let default_spreadsheet = String::from("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
    
    let default_creds = home.join("Downloads")
        .join("client_secret.json")
        .to_string_lossy()
        .to_string();
    
    let default_html = home.join("Library")
        .join("Application Support")
        .join("Sports Interactive")
        .join("Football Manager 2024")
        .join("bd.html")
        .to_string_lossy()
        .to_string();
    
    (default_spreadsheet, default_creds, default_html)
}

#[derive(Parser, Debug)]
#[command(version, about="Upload FM Player data to Google sheets", long_about = None)]
struct CLIArguments {
    #[arg(short,long)]
    spreadsheet: Option<String>,
    #[arg(short,long)]
    credfile: Option<String>,
    #[arg(short,long)]
    input: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    // Initialize logging
    env_logger::init();
    
    let cli = CLIArguments::parse();
    
    // Set up logging level based on verbose flag
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }

    info!("Starting FM player data uploader");
    
    // Get default paths
    let (default_spreadsheet, default_creds, default_html) = get_default_paths();
    
    // Use CLI args or defaults
    let spreadsheet = cli.spreadsheet.unwrap_or(default_spreadsheet);
    let credfile = cli.credfile.unwrap_or(default_creds);
    let input = cli.input.unwrap_or(default_html);
    
    debug!("Using spreadsheet: {}", spreadsheet);
    debug!("Using credentials file: {}", credfile);
    debug!("Using input HTML file: {}", input);

    // Validate input files exist
    if !Path::new(&input).exists() {
        error!("Input file does not exist: {}", input);
        return Err(anyhow::anyhow!("Input file does not exist: {}", input));
    }

    if !Path::new(&credfile).exists() {
        error!("Credentials file does not exist: {}", credfile);
        return Err(anyhow::anyhow!("Credentials file does not exist: {}", credfile));
    }

    // OAuth setup
    let secret = yup_oauth2::read_application_secret(&credfile)
        .await
        .with_context(|| format!("JSON file not found: {}", credfile))?;

    let auth = InstalledFlowAuthenticator::builder(
        secret.clone(),
        InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .with_context(|| "Failed to build authenticator")?;

    let scopes = &["https://www.googleapis.com/auth/spreadsheets"];

    let t = auth.token(scopes)
        .await
        .with_context(|| "Failed to obtain OAuth token")?;
    
    info!("Successfully obtained access token");

    // Create sheets client
    let sheet_c = sheets::Client::new(
        secret.client_id,
        secret.client_secret,
        secret.redirect_uris[0].clone(),
        t.token().ok_or_else(|| anyhow::anyhow!("Failed to get token string"))?,
        t.token().ok_or_else(|| anyhow::anyhow!("Failed to get token string"))?,
    );

    let s = sheets::spreadsheets::Spreadsheets { client: sheet_c };

    // Spreadsheet metadata
    let sc = s.get(&spreadsheet, false, &[])
        .await
        .with_context(|| format!("Failed to access spreadsheet {}", spreadsheet))?;
    
    info!("Connected to spreadsheet {}", sc.body.spreadsheet_id);

    // Verify the sheet exists in the spreadsheet
    let sheet_name = "Squad";
    let sheet_exists = sc.body.sheets.iter().any(|sheet| {
        sheet.properties.title == sheet_name
    });
    
    if !sheet_exists {
        error!("Sheet '{}' not found in spreadsheet", sheet_name);
        return Err(anyhow::anyhow!("Sheet '{}' not found in spreadsheet", sheet_name));
    }

    // Read table from HTML
    let table = read_table(&input)
        .with_context(|| format!("Failed to extract table from {}", input))?;
    
    // Validate table structure
    validate_table_structure(&table)
        .with_context(|| "Invalid table structure")?;
    
    info!("Got table with {} rows", table.len());
    debug!("Table first row has {} columns", table[0].len());

    // Clear spreadsheet target area
    s.values_clear(&spreadsheet, "Squad!A2:AX58", &ClearValuesRequest {})
        .await
        .with_context(|| "Error clearing data")?;

    info!("Cleared old data");

    // Process table data
    let mut matrix = vec![];
    for row in &table {
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

    let new_range = format!("Squad!A2:AX{}", matrix.len() + 1);
    let update_body = ValueRange {
        values: matrix,
        major_dimension: Some(Dimension::Rows),
        range: new_range.clone(),
    };

    debug!("Updating range: {}", new_range);
    
    // Update spreadsheet
    let update = s
        .values_update(
            &spreadsheet,
            &new_range,
            false,
            DateTimeRenderOption::FormattedString,
            ValueRenderOption::FormattedValue,
            ValueInputOption::UserEntered,
            &update_body,
        )
        .await
        .with_context(|| "Failed to upload data to spreadsheet")?;
    
    info!("Updated data: {}", update.status);
    info!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );

    Ok(())
}
