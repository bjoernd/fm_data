use clap::Clap;
use fm_google_api::{google_drive_hub,google_sheets_hub,GSheetsHub,sheet_to_id,clear_sheet_area,upload_attributes};
use table_extract::Table;
use async_std::task;
use futures::stream::{FuturesUnordered, StreamExt};
use std::time::Instant;

static GOOGLE_CREDS_FILE : &str = r#"D:\src\fm_data\src\google-credentials.json"#;
static TOKEN_STORE_FILE : &str = r#"D:\src\fm_data\src\token.json"#;
static FM_SPREADSHEET_NAME : &str = "Football Manager Spreadsheet";

static FM_DATA_SHEET : &str = "Leipzig";
static FM_DATA_HTML : &str = r#"D:\FM21\chemie.html"#;

static FM_LPERF_SHEET : &str = "LE_Stats_Division";
static FM_LPERF_HTML : &str = r#"D:\FM21\2liga-perf.html"#;

static FM_TPERF_SHEET : &str = "LE_Stats_Team";
static FM_TPERF_HTML : &str = r#"D:\FM21\chemie-perf.html"#;

#[derive(Clap)]
#[clap(version = "1.0", author = "Bjoern Doebel <bjoern.doebel@gmail.com>")]
struct Options {
    /// Credentials for the Google API
    #[clap(long, default_value=GOOGLE_CREDS_FILE)]
    creds_file : String,
    /// File to store the OAuth Token for subsequent Google API invocations
    #[clap(long, default_value=TOKEN_STORE_FILE)]
    token_file : String,

    /// Name of the data management spreadsheet in Google Sheets
    #[clap(short('s'), long, default_value=FM_SPREADSHEET_NAME)]
    spreadsheet_name : String,
    /// Name of the team performance table in Google Sheets
    #[clap(short('p'), long, default_value=FM_LPERF_SHEET)]
    league_perf_sheet : String,    
    /// Name of the league performance table in Google Sheets
    #[clap(short('l'), long, default_value=FM_TPERF_SHEET)]
    team_perf_sheet : String,
    /// Name of the team attribute table within the spreadsheet
    #[clap(short('t'), long, default_value=FM_DATA_SHEET)]
    team_attr_sheet : String,
    
    /// Exported team performance data file (HTML)
    #[clap(short('P'), long, default_value=FM_TPERF_HTML)]
    team_perf_html_file : String,
    /// Exported league performance data file (HTML)
    #[clap(short('L'), long, default_value=FM_LPERF_HTML)]
    league_perf_html_file : String,
    /// Exported team attribute file (HTML)
    #[clap(short('T'), long, default_value=FM_DATA_HTML)]
    html_file : String,
    
    /// Specify in-game date to store along with the data
    #[clap(short, long, default_value="NA")]
    ingame_date : String
}

fn read_table(html_file : &str) -> Table {
    Table::find_first(&std::fs::read_to_string(html_file).unwrap()).unwrap()
}

async fn html_to_gsheet(hub: &GSheetsHub, sheetid : &str, sheetname : &str, clear_area : &str, html_data : &str) {
    println!("Clearing data in '{}!{}'...", &sheetname, &clear_area);
    clear_sheet_area(&hub, sheetid, sheetname, clear_area);
    let tab = read_table(html_data);
    println!("Updating '{}' with data read from '{}'...", sheetname, html_data);
    upload_attributes(hub, &tab, sheetid, sheetname);
}

fn update_google_sheets(hub: &GSheetsHub, sheet_id : &str, opts:&Options)
{
    let mut futures = FuturesUnordered::new();
    futures.push(html_to_gsheet(&hub, sheet_id, &opts.team_attr_sheet, "A2:AX58", &opts.html_file));
    futures.push(html_to_gsheet(&hub, sheet_id, &opts.team_perf_sheet, "A2:AW58", &opts.team_perf_html_file));
    futures.push(html_to_gsheet(&hub, sheet_id, &opts.league_perf_sheet, "A2:AU200", &opts.league_perf_html_file));
    task::block_on(async {
        while let Some(_value_returned_from_the_future) = futures.next().await {
        }
    });
}

fn do_update_google(opts: &Options) {
    /* Step 1: We need the sheet ID for our spreadsheet and we get that from Google Drive. */
    let gdh = google_drive_hub(&opts.creds_file, &opts.token_file);

    let sheet_id = sheet_to_id(&gdh, &opts.spreadsheet_name);
    println!("FM sheet '{}' has ID '{}'", &opts.spreadsheet_name, sheet_id);

    /* Step 2: Now we can use the sheet ID to clear the target range and upload new data. */
    let hub = google_sheets_hub(&opts.creds_file, &opts.token_file);
    update_google_sheets(&hub, sheet_id.as_str(), &opts);
}

fn main() {
    let start_time = Instant::now();
    let opts : Options = Options::parse();
    do_update_google(&opts);
    println!("Program finished in {} ms", start_time.elapsed().as_millis());
}