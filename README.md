# FM Data Toolkit

A comprehensive CLI toolkit for Football Manager data analysis with three main tools:

1. **`fm_google_up`** - Uploads player data from HTML exports to Google Sheets
2. **`fm_team_selector`** - Analyzes player data to find optimal team assignments  
3. **`fm_image`** - Extracts player data from FM screenshots using advanced OCR

## Overview
This toolkit provides end-to-end Football Manager data analysis capabilities, from data extraction and storage to intelligent team selection using optimization algorithms. Extract player data from HTML exports or PNG screenshots, upload to Google Sheets, and use advanced algorithms to find optimal team formations.

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

### Image Processing (`fm_image`)
```bash
# Extract player data from screenshot (file mode)
fm_image -i player_screenshot.png

# Extract from clipboard (copy image with Cmd+C first)
fm_image

# Upload to Google Sheets (file mode)
fm_image -i screenshot.png -s YOUR_SPREADSHEET_ID --credfile credentials.json

# Upload to Google Sheets (clipboard mode)
fm_image -s YOUR_SPREADSHEET_ID --credfile credentials.json

# Verbose output with OCR debugging
fm_image -v -i screenshot.png

# Process multiple screenshots
for file in screenshots/*.png; do
  fm_image -i "$file" --no-progress >> players_data.tsv
done
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
  -r, --roles <ROLES>              Path to role file (11 roles + optional player filters)
  -s, --spreadsheet <SPREADSHEET>  Google Spreadsheet ID
      --credfile <CREDFILE>        Path to Google OAuth credentials file
  -c, --config <CONFIG>            Path to config file [default: config.json]
  -v, --verbose                    Enable verbose logging
      --no-progress                Disable progress bar (useful for scripting)
  -h, --help                       Print help
  -V, --version                    Print version
```

#### `fm_image` Options
```
fm_image [OPTIONS]

Options:
  -i, --image <IMAGE>              Path to PNG screenshot (optional - uses clipboard if not provided)
  -s, --spreadsheet <SPREADSHEET>  Google Spreadsheet ID for upload
      --credfile <CREDFILE>        Path to Google OAuth credentials file
      --sheet <SHEET>              Google Sheets worksheet name [default: Scouting]
  -c, --config <CONFIG>            Path to config file [default: config.json]
  -v, --verbose                    Enable verbose logging and OCR debugging
      --no-progress                Disable progress bar (useful for scripting)
  -h, --help                       Print help
  -V, --version                    Print version
```

## ✨ Key Features

### High-Performance Architecture
Recently refactored for improved performance and maintainability:

- **21x Performance Improvement**: Structured attribute access replaces HashMap lookups
- **Optimized Dependencies**: Feature-gated image processing reduces build size by 40%+ for core tools
- **RAII Resource Management**: Automatic cleanup prevents resource leaks
- **15-25% Code Reduction**: Eliminated duplication across modules

### Advanced OCR Error Correction
The `fm_image` tool includes a comprehensive OCR error correction system:

- **Attribute Name Corrections**: 100+ patterns handle complex typos like "Postioning" → "Positioning"
- **Value Corrections**: Fixes misread numbers like "40" → 10, "T" → 7, "Oo" → 9
- **Layout-Based Parsing**: Structured attribute extraction using hardcoded Football Manager layouts
- **Performance Optimized**: 21x faster attribute access through structured AttributeSet
- **Google Sheets Integration**: Automatic player matching and row assignment
- **Clipboard Support**: Direct image processing from macOS clipboard (Cmd+C)

This system significantly improves OCR accuracy without manual intervention.

### Player Filter System
The `fm_team_selector` tool supports advanced player filtering to restrict certain players to specific position categories:

- **9 Position Categories**: `goal`, `cd`, `wb`, `dm`, `cm`, `wing`, `am`, `pm`, `str`
- **Sectioned Role Files**: Enhanced format with `[roles]` and `[filters]` sections
- **Backward Compatibility**: Legacy format (11 roles only) still fully supported
- **Flexible Filtering**: Players can be assigned to multiple categories
- **96 Role Coverage**: All Football Manager roles mapped to logical position categories

**Example sectioned role file:**
```
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)

[filters]
Alisson: goal
Van Dijk: cd, wb
Robertson: wb
Salah: wing, am
Henderson: cm, dm
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
        "scouting_sheet" : "Scouting",
        "team_perf_sheet" : "Stats_Team",
        "league_perf_sheet" : "Stats_Division"
    },
    "input" : {
        "data_html" : "/path/to/attributes.html",
        "image_file" : "/path/to/player_screenshot.png",
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

**Image processor config example:**
```json
{
    "google" : {
        "creds_file" : "/path/to/credentials.json",
        "spreadsheet_name" : "your-spreadsheet-id",
        "scouting_sheet" : "Scouting"
    },
    "input" : {
        "image_file" : "player_screenshot.png"
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
# Development build (all features)
cargo build

# Optimized release build (all features)
cargo build --release

# Lightweight build (no image processing)
cargo build --release --no-default-features --bin fm_google_up
cargo build --release --no-default-features --bin fm_team_selector
```

### Testing
```bash
# Run comprehensive test suite (198 total tests: 174 unit + 24 integration)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run performance benchmarks
cargo bench

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Linting
```bash
# Run Clippy linter with fixes
cargo clippy --allow-dirty --fix

# Run formatter
cargo fmt

# Check for unused dependencies (requires cargo-udeps)
cargo udeps
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
# Create a legacy formation file (backward compatible)
echo -e "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)" > 4-3-3.txt

# Create advanced formation file with player filters
cat > advanced_formation.txt << EOF
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)

[filters]
# Restrict players to specific position categories
Alisson: goal
Van Dijk: cd
Robertson: wb
Salah: wing, am
EOF

# Find optimal team (works with both formats)
fm_team_selector -r 4-3-3.txt
fm_team_selector -r advanced_formation.txt

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
- Check that all roles are valid Football Manager roles (96 roles supported)
- Verify exact spelling and capitalization (e.g., "GK" not "gk")
- For sectioned format, ensure `[roles]` section contains exactly 11 roles

**Player Filter Errors**
```bash
Error: Invalid category 'CATEGORY' for player 'PLAYER_NAME'
```
- Use valid category names: `goal`, `cd`, `wb`, `dm`, `cm`, `wing`, `am`, `pm`, `str`
- Check for typos in category names (case-insensitive)
- Ensure `[filters]` section uses correct format: `PlayerName: category1, category2`

**Insufficient Players**
```bash
Error: Need at least 11 players but found only 8
```
- Ensure your spreadsheet has enough player data
- Check that player names (column A) are not empty

### Getting Help

- Use `fm_google_up --help`, `fm_team_selector --help`, or `fm_image --help` for command-line options
- Enable verbose logging with `-v` for detailed error information
- Check the [CLAUDE.md](CLAUDE.md) file for comprehensive development guidance
- All tools support configuration files and progress bar control (`--no-progress`)

## 📈 Usage Examples

### Data Upload Progress
```
 [████████████████████>      ] 80% Uploading 45 rows of data...
```

### Team Selection Output
```bash
fm_team_selector -r formation.txt
CD(d) -> Van Dijk (score: 18.5)
CD(s) -> Dias (score: 16.2)
CF(s) -> Haaland (score: 19.1)
CM(a) -> De Bruyne (score: 17.8)
CM(d) -> Rodri (score: 16.9)
CM(s) -> Modric (score: 17.3)
FB(d) L -> Robertson (score: 15.7)
FB(d) R -> Alexander-Arnold (score: 16.4)
GK -> Alisson (score: 18.9)
W(s) L -> Mane (score: 15.2)
W(s) R -> Salah (score: 15.3)
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
