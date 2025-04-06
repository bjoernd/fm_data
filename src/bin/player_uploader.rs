use anyhow::{Context, Result};
use clap::Parser;
use sheets::{
    self,
    types::{
        ClearValuesRequest, DateTimeRenderOption, Dimension, ValueInputOption, ValueRange,
        ValueRenderOption,
    },
};
use std::fs;
use std::path::Path;
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

static SPREAD: &str = "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc";
static CREDS: &str = "/Users/bjoernd/Downloads/client_secret_159115558609-mkiidqjgej4ds1615oukp125c4nn2qcf.apps.googleusercontent.com.json";
static HTML: &str =
    "/Users/bjoernd/Library/Application Support/Sports Interactive/Football Manager 2024/bd.html";

#[derive(Parser, Debug)]
#[command(version, about="Upload FM Player data to Google sheets", long_about = None)]
struct CLIArguments {
    #[arg(short,long,default_value_t = SPREAD.to_string())]
    spreadsheet: String,
    #[arg(short,long,default_value_t = CREDS.to_string())]
    credfile: String,
    #[arg(short,long,default_value_t = HTML.to_string())]
    input: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    let cli = CLIArguments::parse();

    // OAuth setup
    let secret = yup_oauth2::read_application_secret(&cli.credfile)
        .await
        .with_context(|| format!("JSON file not found: {}", cli.credfile))?;

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
    
    println!("Got access token");

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
    let sc = s.get(&cli.spreadsheet, false, &[])
        .await
        .with_context(|| format!("Failed to access spreadsheet {}", cli.spreadsheet))?;
    
    println!("Connected to spreadsheet {}", sc.body.spreadsheet_id);

    // Read table from HTML
    let table = read_table(&cli.input)
        .with_context(|| format!("Failed to extract table from {}", cli.input))?;
    
    println!("Got table with {} rows", table.len());

    // Clear spreadsheet target area
    s.values_clear(&cli.spreadsheet, "Squad!A2:AX58", &ClearValuesRequest {})
        .await
        .with_context(|| "Error clearing data")?;

    println!("Cleared old data");

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

    // Update spreadsheet
    let update = s
        .values_update(
            &cli.spreadsheet,
            &new_range,
            false,
            DateTimeRenderOption::FormattedString,
            ValueRenderOption::FormattedValue,
            ValueInputOption::UserEntered,
            &update_body,
        )
        .await
        .with_context(|| "Failed to upload data to spreadsheet")?;
    
    println!("Updated data: {}", update.status);
    println!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );

    Ok(())
}
