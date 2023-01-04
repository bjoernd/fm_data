mod fm_data;

use fm_data::google_api::{
    clear_sheet_area, google_sheets_hub, upload_attributes,
    GSheetsHub,
};
use fm_data::config::Configuration;
use std::time::Instant;
use table_extract::Table;

fn read_table(html_file: &str) -> Table {
    Table::find_first(&std::fs::read_to_string(html_file).unwrap()).unwrap()
}

fn html_to_gsheet(
    hub: &GSheetsHub,
    sheetid: &str,
    sheetname: &str,
    clear_area: &str,
    html_data: &str,
) {
    println!("Clearing data in '{}!{}'...", &sheetname, &clear_area);
    clear_sheet_area(&hub, sheetid, sheetname, clear_area);
    let tab = read_table(html_data);
    println!(
        "Updating '{}' with data read from '{}'...",
        sheetname, html_data
    );
    upload_attributes(hub, &tab, sheetid, sheetname);
}

fn update_google_sheets(hub: &GSheetsHub, sheet_id: &str, opts: &Configuration) {
    html_to_gsheet(
        &hub,
        sheet_id,
        &opts.google.team_sheet,
        "A2:AX58",
        &opts.input.data_html
    );
    /*html_to_gsheet(
        &hub,
        sheet_id,
        &opts.google.team_perf_sheet,
        "A2:AW58",
        &opts.input.team_perf_html
    );*/
    /*
    html_to_gsheet(
        &hub,
        sheet_id,
        &opts.google.league_perf_sheet,
        "A2:AU400",
        &opts.input.league_perf_html
    );
    */
}

fn do_update_google(opts: &Configuration) {

    /* Removed: we always use the same sheet ID and this call will always require another authentication for the Drive API,
       which is annoying. Hence, rather hard-code the sheet ID in the config.
    /* Step 1: We need the sheet ID for our spreadsheet and we get that from Google Drive. */
    let gdh = google_drive_hub(
        &opts.google.creds_file,
        &opts.google.token_file);

    let sheet_id = sheet_to_id(&gdh, opts.google.spreadsheet_name.as_str());
    println!(
        "FM sheet '{}' has ID '{}'",
        &opts.google.spreadsheet_name,
        sheet_id
    );
    */

    /* Step 2: Now we can use the sheet ID to clear the target range and upload new data. */
    let hub = google_sheets_hub(
        &opts.google.creds_file,
        &opts.google.token_file);

    update_google_sheets(&hub, &opts.google.spreadsheet_name, &opts);
}

fn main() {
    let start_time = Instant::now();

    let config = fm_data::config::read_configuration().unwrap();

    do_update_google(&config);
    println!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );
}
