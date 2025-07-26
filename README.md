# FM Data Uploader

A  CLI tool for uploading Football Manager player data to Google Sheets.

## Overview
This tool extracts player data from Football Manager HTML exports and uploads it to Google Sheets for advanced analysis and visualization. Built with Rust for performance and reliability.

## 🚀 Quick Start

### Basic Usage
```bash
# Upload with progress bar (default)
fm_google_up -i player_data.html

# Upload with verbose logging (progress bar auto-disabled)
fm_google_up -v -i player_data.html

# Upload for automation/scripting (no progress bar)
fm_google_up --no-progress -i player_data.html
```

### Command-Line Options
```
fm_google_up [OPTIONS]

Options:
  -s, --spreadsheet <SPREADSHEET>  Google Spreadsheet ID
      --credfile <CREDFILE>        Path to Google OAuth credentials file
  -i, --input <INPUT>              Path to Football Manager HTML export file
  -c, --config <CONFIG>            Path to config file [default: config.json]
  -v, --verbose                    Enable verbose logging
      --no-progress                Disable progress bar (useful for scripting)
  -h, --help                       Print help
  -V, --version                    Print version
```

## ⚙️ Configuration

The application uses a hierarchical configuration system with the following priority:
1. **CLI arguments** (highest priority)
2. **Configuration file** (default: `config.json`)
3. **Built-in defaults** (lowest priority)

### Configuration File Format

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

## 🏗️ Development

### Building
```bash
# Development build
cargo build

# Optimized release build
cargo build --release
```

### Testing
```bash
# Run comprehensive test suite (29 tests)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Linting
```bash
# Run Clippy linter
cargo clippy

# Run formatter
cargo fmt
```

## 🔧 Advanced Usage

### Custom Configuration
```bash
# Use custom config file
fm_google_up -c production_config.json

# Override specific settings
fm_google_up -s "different_spreadsheet_id" -i custom_data.html
```

### Automation Integration
```bash
# For CI/CD pipelines
fm_google_up --no-progress --credfile "$GOOGLE_CREDS" -i "$DATA_FILE"

# With error handling
if ! fm_google_up --no-progress -i data.html; then
    echo "Upload failed"
    exit 1
fi
```

## 🛠️ Troubleshooting

### Common Issues

**Authentication Errors**
```bash
Error: Credentials file does not exist
```
- Ensure your Google OAuth credentials file exists and is accessible
- Verify the path in your config file or CLI argument

**Sheet Not Found**
```bash
Error: Sheet 'Squad' not found in spreadsheet
```
- Check that the target sheet exists in your Google Spreadsheet
- Verify the sheet name matches exactly (case-sensitive)

**Data Size Limits**
```bash
Error: Data has 65 rows but maximum allowed is 57 rows
```
- The tool enforces a hardcoded range limit (A2:AX58)
- Reduce your data size or modify the range limits in the code

### Getting Help

- Use `fm_google_up --help` for command-line options
- Enable verbose logging with `-v` for detailed error information
- Check the [CLAUDE.md](CLAUDE.md) file for development guidance

## 📈 Progress Bar Examples

### Normal Operation
```
 [████████████████████>      ] 80% Uploading 45 rows of data...
```

### Verbose Mode (Progress Disabled)
```bash
fm_google_up -v -i data.html
[INFO] Starting FM player data uploader
[DEBUG] Using spreadsheet: 1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc
[DEBUG] Using credentials file: /path/to/creds.json
[INFO] Successfully obtained access token
[INFO] Connected to spreadsheet 1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc
[INFO] Got table with 45 rows
[INFO] Cleared old data from Squad!A2:AX58
[INFO] Updated data: success
[INFO] Program finished in 3247 ms
```

## 📄 License

This project is licensed under the terms specified in the [LICENSE](LICENSE) file.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

For major changes, please open an issue first to discuss what you would like to change.
