use table_extract::Table;

/* TODO: Make this configurable */
static FM_SPREADSHEET_NAME : &str = "Football Manager Spreadsheet";
static FM_TARGET_SHEET : &str = "Leipzig";
static GOOGLE_CREDS_FILE : &str = r#"D:\src\fm_data\src\google-credentials.json"#;
static TOKEN_STORE_FILE : &str = r#"D:\src\fm_data\src\token.json"#;

use fm_google_api::{google_drive_hub,google_sheets_hub,sheet_to_id,clear_sheet_area,upload_attributes};

fn read_table() -> Table {
    // TODO: make file name a parameter
    Table::find_first(&std::fs::read_to_string(r#"D:\FM21\chemie.html"#).unwrap()).unwrap()
}

fn main() {
    /* Step 1: We need the sheet ID for our spreadsheet and we get that from Google Drive. */
    let gdh = google_drive_hub(GOOGLE_CREDS_FILE, TOKEN_STORE_FILE);

    let sheet_id = sheet_to_id(&gdh, FM_SPREADSHEET_NAME);
    println!("FM sheet has ID '{}'", sheet_id);

    /* Step 2: Now we can use the sheet ID to clear the target range and upload new data. */
    let hub = google_sheets_hub(GOOGLE_CREDS_FILE, TOKEN_STORE_FILE);
    
    println!("Clearing...");
    clear_sheet_area(&hub, sheet_id.as_str(), FM_TARGET_SHEET, "A2:AX58");

    let tab = read_table();
    println!("Updating...");
    upload_attributes(&hub, &tab, sheet_id.as_str(), FM_TARGET_SHEET);
}
