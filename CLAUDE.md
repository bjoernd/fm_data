# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Football Manager data analysis toolkit with two main binaries:

1. **`fm_google_up`** - Extracts player data from HTML exports and uploads them to Google Sheets
2. **`fm_team_selector`** - Analyzes player data from Google Sheets to find optimal team assignments

Both tools share common authentication, configuration, and Google Sheets integration infrastructure.

## Build Commands

```bash
# Build the project (both binaries)
cargo build --release

# Build specific binary
cargo build --release --bin fm_google_up
cargo build --release --bin fm_team_selector

# Run the data uploader with verbose logging
cargo run --bin fm_google_up -- -v

# Run the team selector with role file
cargo run --bin fm_team_selector -- -r test_roles.txt -v

# Run with custom config
cargo run --bin fm_google_up -- -c custom_config.json
cargo run --bin fm_team_selector -- -r roles.txt -c custom_config.json

# Run with progress bar disabled (useful for scripting)
cargo run --bin fm_google_up -- --no-progress
cargo run --bin fm_team_selector -- -r roles.txt --no-progress

# Run tests (comprehensive unit and integration test suites)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run only integration tests
cargo test --test integration_tests

# Run clippy for code quality checks
cargo clippy

# Format code
cargo fmt
```

## Architecture

The codebase is now organized into a library (`src/lib.rs`) with separate modules for each concern:

### Library Modules

- **`config.rs`**: Configuration management with hierarchical priority (CLI > config file > defaults)
- **`table.rs`**: HTML table extraction, validation, and data processing
- **`auth.rs`**: Google OAuth2 authentication handling
- **`sheets_client.rs`**: Google Sheets API operations wrapper (read/write)
- **`progress.rs`**: Progress tracking and user feedback using indicatif
- **`selection.rs`**: Team selection algorithm, player/role data structures, and assignment logic

### Main Components

- **Data Uploader**: `src/bin/player_uploader.rs` - Extracts HTML data and uploads to Google Sheets
- **Team Selector**: `src/bin/fm_team_selector.rs` - Downloads player data and finds optimal team assignments
- **Library**: `src/lib.rs` - Core functionality exposed as reusable modules
- **Integration Tests**: `tests/integration_tests.rs` - End-to-end testing with mock data

### Key Dependencies

- `table-extract`: Extracts tables from HTML exports
- `sheets`: Google Sheets API client
- `yup-oauth2`: OAuth2 authentication for Google APIs
- `clap`: Command-line argument parsing
- `serde`/`serde_json`: Configuration serialization
- `anyhow`: Error handling
- `tokio`: Async runtime
- `indicatif`: Progress bars and spinners for CLI feedback

### Configuration System

The application uses a hierarchical configuration system:
1. CLI arguments (highest priority)
2. JSON config file (default: `config.json`)
3. Hardcoded defaults (lowest priority)

**Flexible Configuration**: The config file supports partial configurations using serde defaults. Missing fields are automatically filled with appropriate default values, allowing users to create minimal config files containing only the settings they want to override.

Configuration includes Google API credentials, spreadsheet IDs, sheet names, input HTML file paths, and role file paths for team selection.

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
- Uses greedy algorithm to assign players to roles for maximum total score
- Supports duplicate roles (e.g., multiple goalkeepers)
- Outputs clean team assignments in format "$ROLE -> $PLAYER_NAME"

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

The team selector requires a role file containing exactly 11 Football Manager roles, one per line:

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

**Valid Roles**: Any of the 96 predefined Football Manager roles (see `selection.rs` for complete list)
**Duplicate Roles**: Allowed (e.g., two "GK" roles for multiple goalkeepers)
**File Format**: Plain text, one role per line, whitespace is trimmed

### Example Usage

```bash
# Basic team selection
cargo run --bin fm_team_selector -- -r my_formation.txt

# With custom spreadsheet and credentials
cargo run --bin fm_team_selector -- -r roles.txt -s YOUR_SPREADSHEET_ID --credfile creds.json

# Using config file
cargo run --bin fm_team_selector -- -c config.json

# Verbose mode for debugging
cargo run --bin fm_team_selector -- -r roles.txt -v

# Scripting mode (no progress bar)
cargo run --bin fm_team_selector -- -r roles.txt --no-progress
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
    "role_file": "my_formation.txt"
  }
}
```

## Testing

The codebase includes comprehensive unit and integration tests:

### Unit Tests (92 tests)
- **Config tests**: JSON parsing, path resolution hierarchy, error handling, partial configuration support
- **Table tests**: HTML parsing, data validation, transformations, size limits  
- **Auth tests**: Credentials validation, file handling, error cases
- **Sheets tests**: Data structure validation, range formatting
- **Progress tests**: Progress tracker creation, message handling, no-op behavior
- **Selection tests**: Role validation, player parsing, assignment algorithm, output formatting

### Integration Tests (9 tests)
- **End-to-end workflow**: Role file → mock data → assignment → output
- **Error handling**: Invalid roles, missing files, insufficient players
- **Edge cases**: Exact player counts, large datasets, duplicate roles
- **Performance testing**: 50+ players processed in <1 second
- **Mock data**: Realistic Football Manager players (Alisson, Van Dijk, Haaland, etc.)
- **Assignment quality**: Verification of logical team selections

## Development Notes

- The crate name is `FMData` (contains both `fm_google_up` and `fm_team_selector` binaries)
- Logging uses `env_logger` with configurable verbosity levels
- OAuth tokens are cached to disk in `tokencache.json`
- Both tools validate credentials and input files before processing
- Data range validation:
  - Uploader: A2:AX58 (57 data rows max)
  - Team Selector: A2:EQ58 (reads player data including 96 role ratings)
- Role file validation ensures exactly 11 valid Football Manager roles
- All modules include comprehensive unit tests (101 tests total: 92 unit + 9 integration)

## Code Quality

The codebase follows Rust best practices and coding standards:

- **Clippy compliance**: All clippy lints are resolved, including modern format string usage
- **Consistent naming**: Method names follow standard Rust conventions (e.g., `Config::create_default()`)
- **Error handling**: Comprehensive error context using `anyhow` throughout
- **Testing**: Comprehensive test coverage with 101 tests (unit + integration) for all public APIs
- **Documentation**: Inline documentation and comprehensive CLAUDE.md guidance