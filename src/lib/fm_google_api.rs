/* Some functions to access the FM spreadsheet via the Google API.
 *
 * Initialization:
 *   - You'll need a credentials JSON from Google.
 *   - That JSON is passed as `creds_file` parameter to the auth_with_google() function and its wrappers.
 *   - The wrappers create hubs to talk to Google Drive and Google Sheets right now
 * 
 * Usable functions:
 *   - list_google_sheets() -> dump all spreadsheets in Google Drive
 *   - sheet_to_id() -> return the internal spreadsheet ID for a given file name (this ID is used by sheet manipulation functions)
 *   - clear_sheet_area() -> clear data out of a given range in a spreadsheet
 *   - upload_attributes() -> take a parsed HTML table exported from BjoernD's FM attribute view and import
 *                            this data into BjoernD's FM Attribute spreadsheet
 */

extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_sheets4 as sheets4;
extern crate google_drive3 as drive3;

use drive3::DriveHub;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, DiskTokenStorage};
use sheets4::{Sheets};
use table_extract::Table;

type ConcreteAuthenticator = Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>;
pub type GDriveHub = DriveHub<hyper::Client, ConcreteAuthenticator>;
pub type GSheetsHub = Sheets<hyper::Client, ConcreteAuthenticator>;

fn auth_with_google(creds_file : &str, token_file : &str) -> ConcreteAuthenticator {
    let secret: ApplicationSecret = oauth2::read_application_secret(
        std::path::Path::new(creds_file)).unwrap();

    Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
        oauth2::DiskTokenStorage::new(&std::string::String::from(token_file)).unwrap(),
        Some(oauth2::FlowType::InstalledInteractive)
    )
}

pub fn google_drive_hub(creds_file : &str, token_file : &str) -> GDriveHub {
    let auth = auth_with_google(creds_file, token_file);
    DriveHub::new(
        hyper::Client::with_connector(
            hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
        auth)
}

pub fn google_sheets_hub(creds_file : &str, token_file : &str) -> GSheetsHub {
    let auth = auth_with_google(creds_file, token_file);
    Sheets::new(
        hyper::Client::with_connector(
            hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
        auth)
}

pub fn list_google_sheets(hub: &GDriveHub) {
    let result = hub.files()
                    .list()
                    .q("mimeType = 'application/vnd.google-apps.spreadsheet'")
                    .doit();

    match result {
        Err(e) => {  println!("{}", e) },
        Ok(res) => {
            let fl : drive3::FileList = res.1;
            for f in fl.files.unwrap() {
                println!("{:?} {:?}", f.name.unwrap(), f.id.unwrap());
            }
        }
    };
}

pub fn sheet_to_id(hub: &GDriveHub, sheetname : &str) -> std::string::String {
    let result = hub.files()
                    .list()
                    .q("mimeType = 'application/vnd.google-apps.spreadsheet'")
                    .doit();

    match result {
        Err(e) => { println!("{}", e) },
        Ok(res) => {
            let fl : drive3::FileList = res.1;
            for f in fl.files.unwrap() {
                if f.name.unwrap() == sheetname { return f.id.unwrap(); }
            }
        }
    };

    String::from("")
}

pub fn clear_sheet_area(hub: &GSheetsHub, sheetid: &str, sheetname: &str, sheetrange: &str) {
    let request_fmt = format!("{}!{}", sheetname, sheetrange);
    
    let req = sheets4::BatchClearValuesRequest { ranges : Some(vec!(request_fmt)) };
    
    let result = hub.spreadsheets().values_batch_clear(req,
                                                       sheetid).doit();
    
    match result {
        Err(e) => { println!("{}", e) },
        Ok(_) => {
            println!("Range cleared.");
        }
    };
}

pub fn upload_attributes(hub: &GSheetsHub, tab: &Table, sheetid: &str, sheetname: &str) {
    let mut matrix = Vec::new();
    
    for row in tab {
        let mut line = Vec::new();
        for cell in row {
            let value = match cell.as_str() {
                "Left"|"Left Only" => "l",
                "Right"|"Right Only" => "r",
                "Either" => "rl",
                "-" => "0",
                _ => cell
            };
            line.push(String::from(value))
        }
        matrix.push(line);
    }

    let mut req = sheets4::ValueRange::default();
    let request_fmt = format!("{}!A2:AX{}", sheetname, &matrix.len()+1);
    req.values = Some(matrix);
    //println!("{:?}", req.values);
    let result = hub.spreadsheets().values_update(req, sheetid, request_fmt.as_str())
                .value_input_option("USER_ENTERED")
                .doit();
    match result {
        Err(e) => { println!("{}", e) },
        Ok(_) => {
            println!("Range updated.");
        }
    };
}
