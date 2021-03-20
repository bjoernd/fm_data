extern crate rusoto_core;
extern crate rusoto_s3;
use clap::Clap;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::time::Instant;

static GOOGLE_CREDS_FILE: &str = r#"D:\src\fm_data\src\google-credentials.json"#;
static TOKEN_STORE_FILE: &str = r#"D:\src\fm_data\src\token.json"#;
static FM_SPREADSHEET_NAME: &str = "Football Manager Spreadsheet";

static FM_DATA_SHEET: &str = "Leipzig";
static FM_DATA_HTML: &str = r#"D:\FM21\chemie.html"#;

static FM_LPERF_SHEET: &str = "LE_Stats_Division";
static FM_LPERF_HTML: &str = r#"D:\FM21\2liga-perf.html"#;

static FM_TPERF_SHEET: &str = "LE_Stats_Team";
static FM_TPERF_HTML: &str = r#"D:\FM21\chemie-perf.html"#;

#[derive(Clap)]
#[clap(version = "1.0", author = "Bjoern Doebel <bjoern.doebel@gmail.com>")]
struct Options {
    /// Credentials for the Google API
    #[clap(long, default_value=GOOGLE_CREDS_FILE)]
    creds_file: String,
    /// File to store the OAuth Token for subsequent Google API invocations
    #[clap(long, default_value=TOKEN_STORE_FILE)]
    token_file: String,

    /// Name of the data management spreadsheet in Google Sheets
    #[clap(short('s'), long, default_value=FM_SPREADSHEET_NAME)]
    spreadsheet_name: String,
    /// Name of the team performance table in Google Sheets
    #[clap(short('p'), long, default_value=FM_LPERF_SHEET)]
    league_perf_sheet: String,
    /// Name of the league performance table in Google Sheets
    #[clap(short('l'), long, default_value=FM_TPERF_SHEET)]
    team_perf_sheet: String,
    /// Name of the team attribute table within the spreadsheet
    #[clap(short('t'), long, default_value=FM_DATA_SHEET)]
    team_attr_sheet: String,

    /// Exported team performance data file (HTML)
    #[clap(short('P'), long, default_value=FM_TPERF_HTML)]
    team_perf_html_file: String,
    /// Exported league performance data file (HTML)
    #[clap(short('L'), long, default_value=FM_LPERF_HTML)]
    league_perf_html_file: String,
    /// Exported team attribute file (HTML)
    #[clap(short('T'), long, default_value=FM_DATA_HTML)]
    html_file: String,

    /// Specify in-game date to store along with the data
    #[clap(short, long, default_value = "NA")]
    ingame_date: String,
}

fn do_update_s3(/*opts: &Configuration*/) {
    /*
    if opts.ingame_date == "NA" {
        println!("--ingame_date not set, skipping S3 upload");
    } else {
        let ingame_date = &opts.ingame_date;
        println!("Setting date to {}", ingame_date);

        let buf: Vec<u8> = "hello world data".into();

        let s3_client = S3Client::new(Region::EuCentral1);
        s3_client
            .put_object(PutObjectRequest {
                bucket: String::from("bjoernd-fm-data"),
                key: String::from("data"),
                body: Some(buf.into()),
                ..Default::default()
            })
            .sync()
            .unwrap();
    }
    */
}

fn main() {
    let start_time = Instant::now();
    //let config = ini!("config.ini");
    //do_update_s3(&config);
    println!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );
}
