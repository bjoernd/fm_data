use clap::Clap;
use fm_google_api::{google_drive_hub,google_sheets_hub,sheet_to_id,clear_sheet_area,upload_attributes};
use table_extract::Table;

static FM_SPREADSHEET_NAME : &str = "Football Manager Spreadsheet";
static FM_TARGET_SHEET : &str = "Leipzig";
static GOOGLE_CREDS_FILE : &str = r#"D:\src\fm_data\src\google-credentials.json"#;
static TOKEN_STORE_FILE : &str = r#"D:\src\fm_data\src\token.json"#;
static FM_DATA_HTML : &str = r#"D:\FM21\chemie.html"#;

#[derive(Clap)]
#[clap(version = "1.0", author = "Bjoern Doebel <bjoern.doebel@gmail.com>")]
struct Options {
    /// Credentials for the Google API
    #[clap(short, long, default_value=GOOGLE_CREDS_FILE)]
    creds_file : String,
    /// File to store the OAuth Token for subsequent Google API invocations
    #[clap(short, long, default_value=TOKEN_STORE_FILE)]
    token_file : String,
    /// Name of the data management spreadsheet in Google Sheets
    #[clap(long, default_value=FM_SPREADSHEET_NAME)]
    spreadsheet_name : String,
    /// Name of the team attribute sheet within the spreadsheet
    #[clap(long, default_value=FM_TARGET_SHEET)]
    target_sheet : String,
    /// Exported attribute file (HTML)
    #[clap(long, default_value=FM_DATA_HTML)]
    html_file : String
}

fn read_table(html_file : &str) -> Table {
    Table::find_first(&std::fs::read_to_string(html_file).unwrap()).unwrap()
}

fn main() {

    let opts : Options = Options::parse();

    /* Step 1: We need the sheet ID for our spreadsheet and we get that from Google Drive. */
    let gdh = google_drive_hub(&opts.creds_file, &opts.token_file);

    let sheet_id = sheet_to_id(&gdh, &opts.spreadsheet_name);
    println!("FM sheet has ID '{}'", sheet_id);

    /* Step 2: Now we can use the sheet ID to clear the target range and upload new data. */
    let hub = google_sheets_hub(&opts.creds_file, &opts.token_file);
    
    println!("Clearing...");
    clear_sheet_area(&hub, sheet_id.as_str(), &opts.target_sheet, "A2:AX58");

    let tab = read_table(&opts.html_file);
    println!("Updating...");
    upload_attributes(&hub, &tab, sheet_id.as_str(), &opts.target_sheet);
}
