# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Football Manager data analysis toolkit with three main binaries:

1. **`fm_google_up`** - Extracts player data from HTML exports and uploads them to Google Sheets
2. **`fm_team_selector`** - Analyzes player data from Google Sheets to find optimal team assignments
3. **`fm_image`** - Extracts player data from Football Manager PNG screenshots using OCR and optionally uploads to Google Sheets

All tools share common authentication, configuration, and Google Sheets integration infrastructure.

## Build Commands

```bash
# Build the project (all binaries with default features)
cargo build --release

# Build specific binary
cargo build --release --bin fm_google_up
cargo build --release --bin fm_team_selector
cargo build --release --bin fm_image

# Build without image processing dependencies (lighter build, 40%+ smaller)
cargo build --release --no-default-features --bin fm_google_up
cargo build --release --no-default-features --bin fm_team_selector

# Build with all features explicitly (same as default for development)
cargo build --release --features full

# Run the data uploader with verbose logging
cargo run --bin fm_google_up -- -v

# Run the team selector with role file
cargo run --bin fm_team_selector -- -r tests/test_roles.txt -v

# Run the image processor with screenshot file
cargo run --bin fm_image -- -i player_screenshot.png -v

# Run the image processor with clipboard paste (copy image with Cmd+C first)
cargo run --bin fm_image -- -v

# Run the image processor with Google Sheets upload (file mode)
cargo run --bin fm_image -- -i player_screenshot.png -s YOUR_SPREADSHEET_ID --credfile credentials.json -v

# Run the image processor with Google Sheets upload (clipboard mode)
cargo run --bin fm_image -- -s YOUR_SPREADSHEET_ID --credfile credentials.json -v

# Run with custom config
cargo run --bin fm_google_up -- -c custom_config.json
cargo run --bin fm_team_selector -- -r roles.txt -c custom_config.json
cargo run --bin fm_image -- -i screenshot.png -c custom_config.json

# Run with progress bar disabled (useful for scripting)
cargo run --bin fm_google_up -- --no-progress
cargo run --bin fm_team_selector -- -r roles.txt --no-progress
cargo run --bin fm_image -- -i screenshot.png --no-progress

# Run image processor with Google Sheets upload (no progress bar)
cargo run --bin fm_image -- -i screenshot.png -s YOUR_SPREADSHEET_ID --credfile credentials.json --no-progress

# Run tests (257 total tests)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run only integration tests
cargo test --test integration_tests

# Run single test by name
cargo test test_name

# Run tests in specific module
cargo test image_data::tests

# Run performance benchmarks
cargo bench

# Run clippy for code quality checks
cargo clippy --allow-dirty --fix

# Format code
cargo fmt
```

## Architecture

The codebase is organized into a library (`src/lib.rs`) with separate modules for each concern, recently refactored for improved maintainability and performance:

### Library Modules

#### Core Infrastructure
- **`cli.rs`**: Consolidated CLI argument parsing with shared CommonCLI structure
- **`config.rs`**: Configuration management with hierarchical priority (CLI > config file > defaults)
- **`table.rs`**: HTML table extraction, validation, and data processing
- **`auth.rs`**: Google OAuth2 authentication handling
- **`sheets_client.rs`**: Google Sheets API operations wrapper (read/write)
- **`sheets_repository.rs`**: Repository pattern for Google Sheets operations with dependency injection
- **`progress.rs`**: Progress tracking and user feedback using indicatif
- **`error.rs`**: Core error types and definitions for the application
- **`error_helpers.rs`**: Error context helpers and standardized error construction patterns
- **`error_messages.rs`**: Standardized error codes and message formatting system
- **`app_builder.rs`**: Application builder pattern for creating configured app runners
- **`app_runner.rs`**: Main application execution logic and CLI argument validation
- **`constants.rs`**: Application-wide constants and configuration defaults
- **`domain.rs`**: Domain-specific value objects (PlayerId, RoleId, SpreadsheetId)
- **`validation.rs`**: Core validation trait definitions and interfaces
- **`validators.rs`**: Concrete validator implementations for different data types
- **`test_helpers.rs`**: Shared test utilities and mock data generation
- **`test_builders.rs`**: Builder pattern implementations for test data construction
- **`types.rs`**: Shared type definitions (PlayerType, Footedness) for cross-feature compatibility

#### Image Processing (Feature-Gated)
- **`image_processor.rs`**: OCR text extraction with ImageProcessor struct and configurable pipeline
- **`image_processor_pool.rs`**: Resource pooling for OCR processors with efficient memory management
- **`image_data.rs`**: Player data structures optimized with structured attribute parsing
- **`image_output.rs`**: High-performance formatting using direct attribute access
- **`image_constants.rs`**: Organized constants for OCR settings, thresholds, and magic numbers
- **`layout_manager.rs`**: Dynamic layout loading with embedded fallbacks for attribute parsing
- **`ocr_corrections.rs`**: Table-driven OCR error correction system
- **`attributes.rs`**: Unified attribute storage with single enum and O(1) access
- **`clipboard.rs`**: Cross-platform clipboard management for image paste operations
- **`selection/`**: Team selection functionality split into focused sub-modules:
  - **`types.rs`**: Core data structures (Player, Role, Team, Assignment, etc.)
  - **`categories.rs`**: Player position categories and role mappings
  - **`parser.rs`**: Role file parsing (both legacy and sectioned formats)
  - **`algorithm.rs`**: Player data parsing and optimal assignment algorithms
  - **`formatter.rs`**: Team output formatting for display

### Main Components

- **Data Uploader**: `src/bin/player_uploader.rs` - Extracts HTML data and uploads to Google Sheets
- **Team Selector**: `src/bin/fm_team_selector.rs` - Downloads player data and finds optimal team assignments
- **Image Processor**: `src/bin/fm_image.rs` - Extracts player data from PNG screenshots using OCR
- **Library**: `src/lib.rs` - Core functionality exposed as reusable modules
- **Integration Tests**: `tests/integration_tests.rs` - End-to-end testing with mock data
- **Layout Files**: `layout-specs/layout-field.txt` and `layout-specs/layout-gk.txt` - Define structured attribute layouts for parsing

### Key Dependencies

#### Core Dependencies
- `table-extract`: Extracts tables from HTML exports
- `sheets`: Google Sheets API client
- `yup-oauth2`: OAuth2 authentication for Google APIs
- `clap`: Command-line argument parsing
- `serde`/`serde_json`: Configuration serialization
- `anyhow`: Error handling
- `thiserror`: Structured error types with derive macros
- `tokio`: Async runtime (optimized features: rt-multi-thread, macros, fs, time)
- `indicatif`: Progress bars and spinners for CLI feedback
- `tempfile`: Temporary file management with RAII cleanup
- `dirs`: Cross-platform directory paths
- `zeroize`: Secure memory clearing for sensitive data

#### Feature-Gated Dependencies (Image Processing)
- `tesseract`: OCR text extraction from images (optional)
- `image`: Image loading and processing (optional)
- `arboard`: Cross-platform clipboard access for image operations (optional)

#### Development Dependencies
- `criterion`: Performance benchmarking with HTML reports

### Configuration System

The application uses a hierarchical configuration system:
1. CLI arguments (highest priority)
2. JSON config file (default: `config.json`)
3. Hardcoded defaults (lowest priority)

**Flexible Configuration**: The config file supports partial configurations using serde defaults. Missing fields are automatically filled with appropriate default values, allowing users to create minimal config files containing only the settings they want to override.

Configuration includes Google API credentials, spreadsheet IDs, sheet names, input HTML file paths, role file paths for team selection, and PNG image paths for OCR processing.

### Data Processing

#### Data Uploader (`fm_google_up`)
- Extracts HTML tables from Football Manager exports
- Validates table structure for consistency
- Processes specific data values (e.g., "Left" → "l", "Right" → "r")
- Enforces hardcoded range limits (max 57 data rows for range A2:AX58)
- Clears existing data before uploading new data

#### Team Selector (`fm_team_selector`)
- Downloads player data from Google Sheets (range A2:EQ58)
- Parses 96 Football Manager roles and 47 player abilities
- Validates role files containing exactly 11 required roles
- **Player filtering**: Optional player category restrictions (NEW FEATURE)
- Uses greedy algorithm to assign players to roles for maximum total score
- Supports duplicate roles (e.g., multiple goalkeepers)
- Supports player filters to restrict players to specific position categories
- Outputs clean team assignments in format "$ROLE -> $PLAYER_NAME (score: X.X)" with individual role scores for transparency

#### Image Processor (`fm_image`)
- Processes PNG screenshots of Football Manager player attributes pages using configurable pipeline
- Uses Tesseract OCR with ImageProcessor struct providing RAII resource management
- Parses all Football Manager attributes using structured layouts
- **Advanced OCR Error Correction**: Table-driven system with 100+ correction patterns
- **Performance Optimized**: Direct attribute access through unified PlayerAttributes structure
- Detects player footedness through color analysis with optional fallback
- **Layout Management**: Dynamic layout loading with embedded fallbacks for reliability
- **Automatic Cleanup**: Temporary file management with proper error handling
- **Google Sheets Integration**: Optional upload to "Scouting" sheet with automatic player matching and row assignment
- Outputs tab-separated player data compatible with spreadsheet import
- Supports verbose mode for OCR debugging and processing details

### Error Handling

Uses `anyhow` for comprehensive error handling throughout the application, with proper context propagation for debugging.

### Async I/O Architecture

The application uses non-blocking async I/O operations throughout:

- **File operations**: All file reads, directory creation, and permission setting use `tokio::fs` instead of blocking `std::fs`
- **Network operations**: Google Sheets API calls and OAuth flows are fully async
- **Performance**: Prevents blocking the async runtime during I/O operations, maintaining responsiveness
- **Scalability**: Enables efficient concurrent processing of multiple operations

### Progress Tracking

The application features a comprehensive progress tracking system:

- **Visual feedback**: Real-time progress bar showing current operation and completion percentage
- **Informative messages**: Clear descriptions of each step (authentication, data processing, upload)
- **Optional display**: Progress bar automatically disabled when using `--verbose` or `--no-progress` flags
- **Non-blocking**: Progress updates don't impact performance
- **Professional UX**: Spinner for indeterminate operations, progress bar for measured operations
- **Clean logging**: INFO messages are suppressed in normal mode to avoid interfering with progress bar display

### Logging Configuration

The application uses smart logging that adapts to the runtime mode:

- **Normal mode**: Only warnings and errors are shown, allowing clean progress bar display
- **Verbose mode (-v)**: All INFO and DEBUG messages are displayed for detailed troubleshooting
- **Progress integration**: Logging is coordinated with progress display to avoid visual conflicts

## Team Selector Usage

### Role File Format

The team selector supports two role file formats:

#### Legacy Format (Backward Compatible)

Simple format with exactly 11 Football Manager roles, one per line:

```
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
```

#### Sectioned Format with Player Filters (NEW)

Enhanced format with role definitions and optional player category filters:

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
# Restrict goalkeeper to only goal roles
Alisson: goal
# Van Dijk can play central defender or wing back roles  
Van Dijk: cd, wb
# Robertson limited to wing back positions
Robertson: wb
# Salah can play wing or attacking midfielder
Salah: wing, am
# Henderson restricted to central and defensive midfield
Henderson: cm, dm
```

**Valid Roles**: Any of the 96 predefined Football Manager roles (see `selection.rs` for complete list)
**Duplicate Roles**: Allowed (e.g., two "GK" roles for multiple goalkeepers)
**File Format**: Plain text, supports comments with `#`, whitespace is trimmed
**Player Filters**: Optional restrictions using player category short names

### Player Categories and Role Mappings

The player filter system uses 9 positional categories that map to Football Manager roles:

#### 1. Goal (`goal`) - 4 roles
- **GK** - Goalkeeper
- **SK(d)** - Sweeper Keeper (Defend)
- **SK(s)** - Sweeper Keeper (Support)
- **SK(a)** - Sweeper Keeper (Attack)

#### 2. Central Defender (`cd`) - 12 roles
- **CD(d)**, **CD(s)**, **CD(c)** - Centre-Back (Defend/Support/Cover)
- **BPD(d)**, **BPD(s)**, **BPD(c)** - Ball Playing Defender (Defend/Support/Cover)
- **NCB(d)** - No-Nonsense Centre-Back (Defend)
- **WCB(d)**, **WCB(s)**, **WCB(a)** - Wide Centre-Back (Defend/Support/Attack)
- **L(s)**, **L(a)** - Libero (Support/Attack)

#### 3. Wing Back (`wb`) - 24 roles
- **FB(d/s/a) R/L** - Full-Back Right/Left (Defend/Support/Attack)
- **WB(d/s/a) R/L** - Wing-Back Right/Left (Defend/Support/Attack)
- **IFB(d) R/L** - Inverted Full-Back (Defend)
- **IWB(d/s/a) R/L** - Inverted Wing-Back (Defend/Support/Attack)
- **CWB(s/a) R/L** - Complete Wing-Back (Support/Attack)

#### 4. Defensive Midfielder (`dm`) - 11 roles
- **DM(d)**, **DM(s)** - Defensive Midfielder (Defend/Support)
- **HB** - Half Back
- **BWM(d)**, **BWM(s)** - Ball Winning Midfielder (Defend/Support)
- **A** - Anchor Man
- **CM(d)** - Central Midfielder (Defend)
- **DLP(d)** - Deep Lying Playmaker (Defend)
- **BBM** - Box to Box Midfielder
- **SV(s)**, **SV(a)** - Segundo Volante (Support/Attack)

#### 5. Central Midfielder (`cm`) - 10 roles
- **CM(d)**, **CM(s)**, **CM(a)** - Central Midfielder (Defend/Support/Attack)
- **DLP(d)**, **DLP(s)** - Deep Lying Playmaker (Defend/Support)
- **RPM** - Roaming Playmaker
- **BBM** - Box to Box Midfielder
- **CAR** - Carrilero
- **MEZ(s)**, **MEZ(a)** - Mezzala (Support/Attack)

#### 6. Winger (`wing`) - 19 roles
- **WM(d/s/a)** - Wide Midfielder (Defend/Support/Attack)
- **WP(s/a)** - Wide Playmaker (Support/Attack)
- **W(s/a) R/L** - Winger Right/Left (Support/Attack)
- **IF(s/a)** - Inside Forward (Support/Attack)
- **IW(s/a)** - Inverted Winger (Support/Attack)
- **WTM(s/a)** - Wide Target Man (Support/Attack)
- **TQ(a)** - Trequartista (Attack)
- **RD(A)** - Raumdeuter (Attack)
- **DW(d/s)** - Defensive Winger (Defend/Support)

#### 7. Attacking Midfielder (`am`) - 10 roles
- **SS** - Shadow Striker
- **EG** - Enganche
- **AM(s)**, **AM(a)** - Attacking Midfielder (Support/Attack)
- **AP(s)**, **AP(a)** - Advanced Playmaker (Support/Attack)
- **CM(a)** - Central Midfielder (Attack)
- **MEZ(a)** - Mezzala (Attack)
- **IW(a)**, **IW(s)** - Inverted Winger (Attack/Support)

#### 8. Playmaker (`pm`) - 8 roles
- **DLP(d)**, **DLP(s)** - Deep Lying Playmaker (Defend/Support)
- **AP(s)**, **AP(a)** - Advanced Playmaker (Support/Attack)
- **WP(s)**, **WP(a)** - Wide Playmaker (Support/Attack)
- **RGA** - Regista
- **RPM** - Roaming Playmaker

#### 9. Striker (`str`) - 14 roles
- **AF** - Advanced Forward
- **P** - Poacher
- **DLF(s)**, **DLF(a)** - Deep Lying Forward (Support/Attack)
- **CF(s)**, **CF(a)** - Complete Forward (Support/Attack)
- **F9** - False 9
- **TM(s)**, **TM(a)** - Target Man (Support/Attack)
- **PF(d/s/a)** - Pressing Forward (Defend/Support/Attack)
- **IF(a)**, **IF(s)** - Inside Forward (Attack/Support)

**Note**: Some roles appear in multiple categories (e.g., `DLP(d)` is in both Defensive Midfielder, Central Midfielder, and Playmaker categories). This allows flexible player assignments based on tactical interpretation.

### Example Usage

```bash
# Basic team selection (legacy format)
cargo run --bin fm_team_selector -- -r examples/formation_legacy.txt

# Team selection with player filters (new sectioned format)
cargo run --bin fm_team_selector -- -r examples/formation_with_filters.txt

# Complex filtering scenario
cargo run --bin fm_team_selector -- -r examples/formation_mixed_restrictions.txt

# With custom spreadsheet and credentials
cargo run --bin fm_team_selector -- -r roles.txt -s YOUR_SPREADSHEET_ID --credfile creds.json

# Using config file
cargo run --bin fm_team_selector -- -c config.json

# Verbose mode for debugging (shows filter processing)
cargo run --bin fm_team_selector -- -r examples/formation_with_filters.txt -v

# Scripting mode (no progress bar)
cargo run --bin fm_team_selector -- -r examples/formation_legacy.txt --no-progress
```

### Configuration Example

```json
{
  "google": {
    "spreadsheet_name": "YOUR_SPREADSHEET_ID",
    "creds_file": "path/to/credentials.json",
    "team_sheet": "Squad"
  },
  "input": {
    "role_file": "examples/formation_with_filters.txt"
  }
}
```

### Error Messages and Troubleshooting

#### Role File Validation Errors

**"Role file must contain exactly 11 roles, found X"**
- Ensure the `[roles]` section contains exactly 11 Football Manager roles
- Check for empty lines or comments within the roles section

**"Invalid role: 'ROLE_NAME'"**
- Verify the role name matches one of the 96 valid Football Manager roles
- Check for typos, extra spaces, or incorrect formatting
- See the complete role list in `src/selection.rs`

**"Duplicate role found: 'ROLE_NAME'"**
- This is informational - duplicate roles are allowed for formations requiring multiple players in the same position

#### Player Filter Validation Errors

**"Invalid category 'CATEGORY' for player 'PLAYER_NAME' on line X"**
- Use only valid category short names: `goal`, `cd`, `wb`, `dm`, `cm`, `wing`, `am`, `pm`, `str`
- Check for typos or unsupported category names
- Categories are case-insensitive but must match exactly

**"Duplicate player filter for 'PLAYER_NAME' on line X"**
- Each player can only have one filter entry in the `[filters]` section
- Combine multiple categories for a player into a single line: `Player: cat1, cat2, cat3`

**"Invalid filter format on line X: expected 'PLAYER_NAME: CATEGORIES'"**
- Ensure filter lines follow the format: `PlayerName: category1, category2`
- Use colon (`:`) to separate player name from categories
- Use commas (`,`) to separate multiple categories

#### Assignment Warnings

**"Warning: Player 'PLAYER_NAME' could not be assigned due to filter restrictions"**
- The player's allowed categories don't include any roles needed for the formation
- Consider expanding the player's allowed categories or adjusting the formation
- Check if the player's natural position matches their filter categories

**"Warning: X players could not be assigned due to filter restrictions"**
- Multiple players are filtered out of all available roles
- Review filter settings and formation requirements
- Some unfiltered players may be assigned instead

#### File Format Issues

**"Missing [roles] section in role file"**
- When using sectioned format, the `[roles]` section is required
- Ensure section headers are enclosed in square brackets: `[roles]`, `[filters]`

**"No filters section found - using roles-only mode"**
- This is informational when using sectioned format without filters
- The `[filters]` section is optional

#### Common Solutions

1. **Backward Compatibility**: Old role files (11 lines without sections) continue to work unchanged
2. **Section Headers**: Use `[roles]` and `[filters]` exactly (case-insensitive)
3. **Comments**: Use `#` at the start of lines for comments in both sections
4. **Whitespace**: Leading/trailing spaces are automatically trimmed
5. **Case Sensitivity**: Category names are case-insensitive (`GOAL` = `goal` = `Goal`)

## Image Processor Usage

The `fm_image` tool supports two input modes for processing Football Manager player attributes:

### Input Modes

#### 1. File Mode
Provide a PNG file path with the `-i` flag:
```bash
fm_image -i player_screenshot.png
```

#### 2. Clipboard Mode (NEW)
Copy an image with Cmd+C and run without the `-i` flag:
```bash
# Copy image to clipboard first, then run:
fm_image
```

The tool will wait for you to press Enter and then read the image directly from your macOS clipboard.

**Note**: Progress bar is automatically disabled in clipboard mode for better interactive experience.

### Screenshot Requirements

Both input modes require PNG images of Football Manager player attributes pages:

- **Format**: PNG images only (validated by CLI and image processor)
- **Content**: Player attributes page showing technical, mental, physical, and optionally goalkeeping attributes
- **Visibility**: All relevant attributes should be visible and clearly readable
- **Resolution**: Higher resolution images provide better OCR accuracy

### OCR Processing

The tool uses Tesseract OCR with optimized settings and comprehensive error correction:

- **Character whitelist**: Limited to alphanumeric characters and common punctuation for better accuracy
- **Page segmentation**: Configured for uniform text blocks typical in FM screenshots
- **Language**: English language model for consistent text recognition
- **Advanced Error Correction**: Multi-layered system handling both attribute names and values

#### OCR Error Correction System

The image processor includes comprehensive correction for common OCR misreads:

**Attribute Name Corrections:**
- Spacing issues: "OffThe Ball" → "Off the Ball"
- Character typos: "Postioning" → "Positioning", "Agtity" → "Agility"
- Complex typos: "Tendeney" → "Tendency"

**Attribute Value Corrections:**
- Number misreads: "40" → 10, "T" → 7, "Oo" → 9
- Character confusion: "n" → 11, "rn" → 12, "ll" → 11
- Invalid range handling: Out-of-range values (>20) are corrected or rejected

This system significantly improves OCR accuracy for Football Manager screenshots without requiring manual intervention.

### Player Data Extraction

The image processor uses **structured layout-based parsing** to extract data reliably:

- **Player name**: Always extracted from the first line of OCR text
- **Age**: Parsed using "X years old" pattern (more reliable than standalone numbers)
- **Player type**: Detected by presence of "GOALKEEPING" text (goalkeeper vs field player)
- **Footedness**: Detected through color analysis of foot icons (left/right/both feet)
- **Attributes**: Extracted using deterministic layouts with unified attribute system
  - **All players**: 15 rows × 3 columns containing all 47 Football Manager attributes
  - **Unified access**: Single attribute enum covering technical, mental, physical, and goalkeeping attributes

### Structured Attribute Parsing

The tool uses hardcoded layouts that match the fixed structure of FM screenshots:

- **Layout files**: `layout-specs/layout-field.txt` and `layout-specs/layout-gk.txt` define expected attribute positions
- **Position-based extraction**: Numbers are matched to attributes by their column position
- **Advanced OCR correction**: Comprehensive handling of attribute name typos and value misreads
- **Reliability**: Eliminates guesswork by leveraging known FM attribute layout

### Usage Examples

#### File Mode Examples
```bash
# Basic screenshot processing (stdout output only)
cargo run --bin fm_image -- -i player_screenshot.png

# With verbose OCR debugging
cargo run --bin fm_image -- -i screenshot.png -v

# Upload to Google Sheets "Scouting" sheet (default sheet name)
cargo run --bin fm_image -- -i screenshot.png -s YOUR_SPREADSHEET_ID --credfile credentials.json

# Upload to custom sheet name
cargo run --bin fm_image -- -i screenshot.png -s YOUR_SPREADSHEET_ID --credfile credentials.json --sheet "PlayerScouts"

# Using configuration file (can include Google Sheets settings)
cargo run --bin fm_image -- -i screenshot.png -c image_config.json

# Scripting mode (no progress bar)
cargo run --bin fm_image -- -i screenshot.png --no-progress
```

#### Clipboard Mode Examples (NEW)
```bash
# Basic clipboard processing (copy image with Cmd+C first, then run)
cargo run --bin fm_image -- 

# Clipboard with verbose OCR debugging
cargo run --bin fm_image -- -v

# Clipboard with Google Sheets upload
cargo run --bin fm_image -- -s YOUR_SPREADSHEET_ID --credfile credentials.json

# Clipboard with custom sheet name
cargo run --bin fm_image -- -s YOUR_SPREADSHEET_ID --credfile credentials.json --sheet "PlayerScouts"

# Clipboard with configuration file
cargo run --bin fm_image -- -c image_config.json

# Clipboard scripting mode (no progress bar)
cargo run --bin fm_image -- --no-progress
```

#### Batch Processing Examples
```bash
# Batch processing with Google Sheets upload
for file in screenshots/*.png; do
  cargo run --bin fm_image -- -i "$file" -s YOUR_SPREADSHEET_ID --credfile credentials.json --no-progress
done

# Processing multiple screenshots (stdout only, for importing elsewhere)
for file in screenshots/*.png; do
  cargo run --bin fm_image -- -i "$file" --no-progress >> players_data.tsv
done
```

### Configuration Examples

#### Basic Configuration (stdout output only)
```json
{
  "input": {
    "image_file": "player_screenshot.png"
  }
}
```

#### Configuration with Google Sheets Upload
```json
{
  "google": {
    "spreadsheet_name": "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc",
    "creds_file": "path/to/credentials.json",
    "scouting_sheet": "PlayerScouts"
  },
  "input": {
    "image_file": "player_screenshot.png"
  }
}
```

#### Full Configuration with All Options
```json
{
  "google": {
    "spreadsheet_name": "YOUR_SPREADSHEET_ID",
    "creds_file": "credentials.json",
    "scouting_sheet": "Scouting"
  },
  "input": {
    "image_file": "screenshots/messi.png"
  }
}
```

### Output Format

The tool outputs tab-separated values (TSV) format with the following columns:

1. Player name
2. Age
3. Footedness (Left, Right, Both)
4. Player type (Outfield, Goalkeeper)
5. All 47 Football Manager attributes in consistent order
   - Technical attributes (14 values)
   - Mental attributes (14 values)
   - Physical attributes (5 values)
   - Goalkeeping attributes (7 values)
   - Additional attributes (7 values)

This format is compatible with spreadsheet applications and can be imported directly into Google Sheets for further analysis.

### Troubleshooting OCR Issues

**Low OCR accuracy**:
- Ensure screenshot is high resolution and clearly readable
- Verify all attribute text is visible and not obscured
- Use verbose mode (`-v`) to see OCR debugging information

**Missing or incorrect attributes**:
- Most common OCR errors are automatically corrected (see OCR Error Correction System above)
- Check that screenshot shows the complete attributes page in the standard FM layout
- Ensure all attribute section headers are visible and readable
- Verify the image is in PNG format
- Use verbose mode (`-v`) to see structured parsing and correction debug information

**Layout parsing issues**:
- The tool expects FM attributes in a 15-row × 3-column layout
- Each row should contain 3 attributes with their values (e.g., "Corners 7 Aggression 8 Acceleration 11")
- Advanced OCR correction handles attribute name typos and value misreads automatically
- Player name should be clearly visible in the first line of the screenshot

**Footedness detection errors**:
- Ensure foot icons are visible in the screenshot
- Check that icons have sufficient color contrast
- Foot color analysis works best with default FM skin colors

**Age parsing issues**:
- Age must appear in "X years old" format in the OCR text
- The tool no longer relies on standalone numbers for age detection (more reliable)

### Troubleshooting Google Sheets Upload Issues

**Authentication failures**:
- Verify that the credentials file exists and is readable (`--credfile path/to/credentials.json`)
- Ensure the credentials file contains valid Google Service Account credentials
- Check that the service account has access to the target spreadsheet
- Use verbose mode (`-v`) to see authentication debugging information

**Spreadsheet access errors**:
- Verify the spreadsheet ID is correct and accessible
- Ensure the service account email has been granted access to the spreadsheet (Editor or Owner permissions)
- Check that the target sheet exists in the spreadsheet (default: "Scouting")
- Use `--sheet "CustomName"` to specify a different sheet name if needed

**Upload errors**:
- Ensure the target sheet has sufficient empty rows (scans rows 4-104 for available space)
- Check that the spreadsheet is not being edited by another user during upload
- Verify the sheet structure matches expected format (50 columns: A through AX)
- Upload errors are logged as warnings and do not prevent stdout output

**Configuration issues**:
- Use `scouting_sheet` in config file to specify custom sheet name
- Set `spreadsheet_name` and `creds_file` in the `google` section of config
- CLI arguments override config file settings
- Missing Google Sheets settings default to stdout-only mode (backward compatible)

**Data range conflicts**:
- The tool uses range A4:AX104 for player data (101 rows available)
- Existing players are updated in place based on name matching in column A
- New players are added to the first completely empty row
- If no empty rows are available, an error is logged but stdout continues to work

**Common solutions**:
- Always works in stdout-first mode: upload failures don't break console output
- Use `--no-progress` for scripting to avoid progress bar interference
- Enable verbose mode (`-v`) for detailed upload debugging
- Test authentication separately using other tools (`fm_google_up` or `fm_team_selector`)

## Testing

The codebase includes comprehensive unit and integration tests:

### Comprehensive Test Suite (257 tests)
- **Config tests**: JSON parsing, path resolution hierarchy, error handling, partial configuration support
- **Table tests**: HTML parsing, data validation, transformations, size limits  
- **Auth tests**: Credentials validation, file handling, error cases
- **Sheets tests**: Data structure validation, range formatting
- **Progress tests**: Progress tracker creation, message handling, no-op behavior
- **Selection tests**: Role validation, player parsing, assignment algorithm, output formatting
- **Player Category tests**: Category parsing, role mappings, filter validation
- **Role File Parser tests**: Sectioned format parsing, backward compatibility
- **Assignment Algorithm tests**: Filter-aware assignment, eligibility checking
- **Image Processing tests**: OCR text extraction, footedness detection, structured parsing
- **Image Data tests**: Player data structure validation, layout-based attribute parsing, type detection
- **Image Output tests**: TSV formatting, data serialization, output validation
- **Integration tests**: End-to-end workflows with mock data
- **Performance tests**: Benchmarking with realistic datasets
- **Error handling**: Edge cases, validation failures, resource constraints

## Development Notes

- The crate name is `FMData` (contains `fm_google_up`, `fm_team_selector`, and `fm_image` binaries)
- Logging uses `env_logger` with configurable verbosity levels
- OAuth tokens are cached to disk in `tokencache.json`
- All tools validate credentials and input files before processing
- Data range validation:
  - Uploader: A2:AX58 (57 data rows max)
  - Team Selector: A2:EQ58 (reads player data including 96 role ratings)
  - Image Processor: PNG format validation, OCR text extraction, and optional Google Sheets upload to A4:AX104 (101 rows)
- Role file validation ensures exactly 11 valid Football Manager roles
- Player filtering system with 9 positional categories covering all 96 roles
- Sectioned role file format with backward compatibility for legacy files
- All modules include comprehensive unit tests (257 total tests)
- Structured attribute parsing with hardcoded FM layouts for reliability

## Code Quality

The codebase follows Rust best practices and coding standards:

- **Clippy compliance**: All clippy lints are resolved, including modern format string usage
- **Consistent naming**: Method names follow standard Rust conventions (e.g., `Config::create_default()`)
- **Error handling**: Comprehensive error context using `anyhow` throughout
- **Testing**: Comprehensive test coverage with 257 tests for all public APIs
- **Documentation**: Inline documentation and comprehensive CLAUDE.md guidance