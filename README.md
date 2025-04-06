# fm_data
Data Analysis for Football Manager

## Overview
This project provides tools for extracting and analyzing data from Football Manager games. The main functionality is uploading player data from HTML exports to Google Sheets for further analysis.

## Usage
```
fm_google_up [OPTIONS]

Options:
  -s, --spreadsheet <SPREADSHEET>  Google Spreadsheet ID
  -c, --credfile <CREDFILE>        Path to Google OAuth credentials file
  -i, --input <INPUT>              Path to Football Manager HTML export file
  -C, --config <CONFIG>            Path to config file [default: config.json]
  -v, --verbose                    Enable verbose logging
  -h, --help                       Print help
  -V, --version                    Print version
```

## Configuration
The application can be configured using a JSON file (default: `config.json`):

```json
{
    "google" : {
        "creds_file" : "/path/to/google-credentials.json",
        "token_file" : "/path/to/token.json",
        "spreadsheet_name" : "your-spreadsheet-id",
        "team_sheet" : "Squad",
        "team_perf_sheet" : "Stats_Team",
        "league_perf_sheet" : "Stats_Division"
    },
    "input" : {
        "data_html" : "/path/to/attributes.html",
        "league_perf_html" : "/path/to/league-perf.html",
        "team_perf_html" : "/path/to/team-perf.html"
    }
}
```

## Building
```
cargo build --release
```

## Running
```
cargo run --bin fm_google_up -- -v
```
