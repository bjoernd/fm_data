# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Football Manager data analysis tool that extracts player data from HTML exports and uploads them to Google Sheets. The project consists of a single binary `fm_google_up` that handles authentication, HTML parsing, data processing, and Google Sheets integration.

## Build Commands

```bash
# Build the project
cargo build --release

# Run the main binary with verbose logging
cargo run --bin fm_google_up -- -v

# Run with custom config
cargo run --bin fm_google_up -- -C custom_config.json

# Run with progress bar disabled (useful for scripting)
cargo run --bin fm_google_up -- --no-progress

# Run tests (comprehensive unit test suite included)
cargo test

# Run tests with output
cargo test -- --nocapture

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
- **`sheets_client.rs`**: Google Sheets API operations wrapper
- **`progress.rs`**: Progress tracking and user feedback using indicatif

### Main Components

- **Binary**: `src/bin/player_uploader.rs` - Main application entry point using the library modules
- **Library**: `src/lib.rs` - Core functionality exposed as reusable modules

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

Configuration includes Google API credentials, spreadsheet IDs, sheet names, and input HTML file paths.

### Data Processing

- Extracts HTML tables from Football Manager exports
- Validates table structure for consistency
- Processes specific data values (e.g., "Left" → "l", "Right" → "r")
- Enforces hardcoded range limits (max 57 data rows for range A2:AX58)
- Clears existing data before uploading new data

### Error Handling

Uses `anyhow` for comprehensive error handling throughout the application, with proper context propagation for debugging.

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

## Testing

The codebase includes comprehensive unit tests for all modules:

- **Config tests**: JSON parsing, path resolution hierarchy, error handling
- **Table tests**: HTML parsing, data validation, transformations, size limits  
- **Auth tests**: Credentials validation, file handling, error cases
- **Sheets tests**: Data structure validation, range formatting
- **Progress tests**: Progress tracker creation, message handling, no-op behavior

Integration tests with Google APIs are not included due to authentication complexity but could be added for CI/CD environments.

## Development Notes

- The crate name is `FMData` (note: inconsistent with binary name `fm_google_up`)
- Logging uses `env_logger` with configurable verbosity levels
- OAuth tokens are cached to disk in `tokencache.json`
- The application validates that both input HTML files and Google credentials exist before processing
- Range validation ensures data fits within the hardcoded Google Sheets range (A2:AX58)
- All modules include comprehensive unit tests (29 tests total)

## Code Quality

The codebase follows Rust best practices and coding standards:

- **Clippy compliance**: All clippy lints are resolved, including modern format string usage
- **Consistent naming**: Method names follow standard Rust conventions (e.g., `Config::create_default()`)
- **Error handling**: Comprehensive error context using `anyhow` throughout
- **Testing**: 100% module coverage with unit tests for all public APIs
- **Documentation**: Inline documentation and comprehensive CLAUDE.md guidance