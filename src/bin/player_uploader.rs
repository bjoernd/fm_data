use sheets::{
    self,
    types::{
        ClearValuesRequest, DateTimeRenderOption, Dimension, ValueInputOption, ValueRange,
        ValueRenderOption,
    },
};
use std::time::Instant;
use table_extract::Table;
use tokio;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use clap::Parser;

fn read_table(html_file: &str) -> Table {
    Table::find_first(&std::fs::read_to_string(html_file).unwrap()).unwrap()
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
    input: String
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    let cli = CLIArguments::parse();

    /* This is how we OAuth today.
     *   1. Create a new OAuth json in Google Cloud console.
     *   2. Download OAuth config JSON (aka CREDS here)
     *   3. Read the secrets into yup_oauth2...
     */
    let secret = yup_oauth2::read_application_secret(cli.credfile)
        .await
        .expect("JSON file not found");

    /* Here we build an Authenticator that will either use a cached token or redirect the user to
     * a Google page asking to confirm authorization. The fancy new thing here is the HTTPRedirect
     * return method, which means the auth page will redirect to a local HTTP socket and thus signal
     * the application to continue as soon as authentication succeeded.
     */
    let auth = InstalledFlowAuthenticator::builder(
        secret.clone(),
        InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    /* Here we define what we want to access. In our case this is Spreadsheet access only. */
    let scopes = &["https://www.googleapis.com/auth/spreadsheets"];

    let t = auth.token(scopes).await.unwrap();
    println!("Got access token");

    /* Create the sheets client that we will use for our requests below. */
    let sheet_c = sheets::Client::new(
        secret.client_id,
        secret.client_secret,
        secret.redirect_uris[0].clone(),
        t.token().unwrap(),
        t.token().unwrap(),
    );

    let s = sheets::spreadsheets::Spreadsheets { client: sheet_c };

    /* Spreadsheet metadata */
    let sc = s.get(&cli.spreadsheet, false, &[]).await.unwrap();
    println!("Connected to spreadsheet {}", sc.body.spreadsheet_id);

    /* Read our table from the input HTML file */
    let table = read_table(&cli.input);
    println!("Got table {:?}", table);

    /* Clear spreadsheet target area */
    s.values_clear(&cli.spreadsheet, "Squad!A2:AX58", &ClearValuesRequest {})
        .await
        .expect("Error clearing data.");

    println!("Cleared old data");

    /* Some minor massaging of the input data to suit the Google Sheet processing */
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

    /* And now send the update request... */
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
        .expect("Failed to upload new data");
    println!("Updated data: {}", update.status);

    println!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );
}
