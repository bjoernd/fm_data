# FM Data Toolkit

A comprehensive CLI toolkit for Football Manager data analysis with two main tools:

1. **`fm_google_up`** - Uploads player data from HTML exports to Google Sheets
2. **`fm_team_selector`** - Analyzes player data to find optimal team assignments

## Overview
This toolkit provides end-to-end Football Manager data analysis capabilities, from data extraction and storage to intelligent team selection using optimization algorithms.

## 🚀 Quick Start

### Player Data Upload (`fm_google_up`)
```bash
# Upload with progress bar (default)
fm_google_up -i player_data.html

# Upload with verbose logging (progress bar auto-disabled)
fm_google_up -v -i player_data.html

# Upload for automation/scripting (no progress bar)
fm_google_up --no-progress -i player_data.html
```

### Team Selection (`fm_team_selector`)
```bash
# Select optimal team using default config
fm_team_selector -r formation.txt

# Use custom configuration file
fm_team_selector -c config.json -r my_formation.txt

# Verbose output for debugging
fm_team_selector -v -r formation.txt
```

### Command-Line Options

#### `fm_google_up` Options
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

#### `fm_team_selector` Options
```
fm_team_selector [OPTIONS]

Options:
  -r, --roles <ROLES>              Path to role file (11 Football Manager roles)
  -s, --spreadsheet <SPREADSHEET>  Google Spreadsheet ID
      --credfile <CREDFILE>        Path to Google OAuth credentials file
  -c, --config <CONFIG>            Path to config file [default: config.json]
  -v, --verbose                    Enable verbose logging
  -h, --help                       Print help
  -V, --version                    Print version
```

## ⚙️ Configuration

The application uses a hierarchical configuration system with the following priority:
1. **CLI arguments** (highest priority)
2. **Configuration file** (default: `config.json`)
3. **Built-in defaults** (lowest priority)

### Configuration File Format

The config file supports **partial configurations** - you only need to specify the fields you want to override. Missing fields will automatically use sensible defaults.

**Complete config example:**
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
        "team_perf_html" : "/path/to/team-perf.html",
        "role_file" : "/path/to/formation.txt"
    }
}
```

**Team selector config example:**
```json
{
    "google" : {
        "creds_file" : "/path/to/my-credentials.json",
        "spreadsheet_name" : "your-spreadsheet-id",
        "team_sheet" : "Squad"
    },
    "input" : {
        "role_file" : "my_formation.txt"
    }
}
```

**Minimal config example (only override what you need):**
```json
{
    "google" : {
        "creds_file" : "/path/to/my-credentials.json",
        "spreadsheet_name" : "your-spreadsheet-id"
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
# Run comprehensive test suite (54 total tests: 45 unit + 9 integration)
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

### Data Upload Automation
```bash
# Use custom config file
fm_google_up -c production_config.json

# Override specific settings
fm_google_up -s "different_spreadsheet_id" -i custom_data.html

# For CI/CD pipelines
fm_google_up --no-progress --credfile "$GOOGLE_CREDS" -i "$DATA_FILE"

# With error handling
if ! fm_google_up --no-progress -i data.html; then
    echo "Upload failed"
    exit 1
fi
```

### Team Selection Workflows
```bash
# Create a formation file
echo -e "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)" > 4-3-3.txt

# Find optimal team for the formation
fm_team_selector -r 4-3-3.txt

# Pipeline: upload data then select team
fm_google_up -i player_data.html && fm_team_selector -r formation.txt

# Use different formations
fm_team_selector -r formations/defensive.txt  # 5-4-1
fm_team_selector -r formations/attacking.txt  # 3-5-2
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

**Role File Errors**
```bash
Error: Invalid role on line 3: "invalidrole"
```
- Check that all roles are valid Football Manager roles (see [ROLE_FILE_FORMAT.md](ROLE_FILE_FORMAT.md))
- Verify exact spelling and capitalization (e.g., "GK" not "gk")

**Insufficient Players**
```bash
Error: Need at least 11 players but found only 8
```
- Ensure your spreadsheet has enough player data
- Check that player names (column A) are not empty

### Getting Help

- Use `fm_google_up --help` or `fm_team_selector --help` for command-line options
- Enable verbose logging with `-v` for detailed error information
- Check the [CLAUDE.md](CLAUDE.md) file for development guidance
- See [ROLE_FILE_FORMAT.md](ROLE_FILE_FORMAT.md) for role file documentation

## 📈 Usage Examples

### Data Upload Progress
```
 [████████████████████>      ] 80% Uploading 45 rows of data...
```

### Team Selection Output
```bash
fm_team_selector -r formation.txt
CD(d) -> Van Dijk
CD(s) -> Dias  
CF(s) -> Haaland
CM(a) -> De Bruyne
CM(d) -> Rodri
CM(s) -> Modric
FB(d) L -> Robertson
FB(d) R -> Alexander-Arnold
GK -> Alisson
W(s) L -> Mane
W(s) R -> Salah
Total Score: 187.3
```

### Verbose Mode Examples
```bash
# Upload with detailed logging
fm_google_up -v -i data.html
[INFO] Starting FM player data uploader
[DEBUG] Using spreadsheet: 1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc
[INFO] Successfully obtained access token
[INFO] Got table with 45 rows
[INFO] Updated data: success

# Team selection with detailed logging  
fm_team_selector -v -r formation.txt
[INFO] Starting team selection assistant
[INFO] Parsed 11 roles from formation.txt
[INFO] Downloaded 23 players from spreadsheet
[INFO] Found optimal assignments with total score: 187.3
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
