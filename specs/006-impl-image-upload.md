# Implementation Plan: Image Upload Feature

This document breaks down the implementation of the image upload feature into detailed, actionable steps.

## ✅ Step 1: Extend CLI Arguments for fm_image - COMPLETED

**File**: `src/cli.rs`

**Task**: Add Google Sheets related arguments to the image CLI structure.

**Details**:
- ✅ Located the existing image CLI argument structure (`ImageCLI`)
- ✅ Added Google Sheets arguments via `CommonCLI` flattening:
  - `--spreadsheet` / `-s`: Spreadsheet ID
  - `--credfile`: Path to Google credentials JSON file  
- ✅ Added specific `--sheet`: Sheet name (default: "Scouting")
- ✅ Used consistent help text and validation patterns from other tools
- ✅ Updated all test functions to include the new `sheet` field

**Acceptance Criteria**:
- ✅ `cargo run --bin fm_image -- --help` shows the new Google Sheets arguments
- ✅ Arguments are optional and have appropriate defaults (sheet defaults to "Scouting")
- ✅ All existing tests pass without regressions (171 unit + 17 integration tests)
- ✅ Argument parsing works correctly and doesn't break existing functionality

## ✅ Step 2: Update Configuration Structure - COMPLETED

**File**: `src/config.rs`

**Task**: Extend the configuration system to support image tool Google Sheets settings.

**Details**:
- ✅ Added new constant `SCOUTING_SHEET = "Scouting"` to `constants.rs`
- ✅ Added `image_file` field to `InputConfig` for consistency with other input configurations
- ✅ Added `scouting_sheet` field to `GoogleConfig` with default function
- ✅ Updated `GoogleConfig::default()` implementation to include new field
- ✅ Created comprehensive `resolve_image_paths()` method that:
  - Resolves spreadsheet ID, credentials file, image file, and sheet name
  - Uses hierarchical priority: CLI > config > defaults
  - Performs full validation including PNG file validation
  - Returns (spreadsheet, credfile, imagefile, sheet) tuple
- ✅ Maintained backward compatibility with existing config files using serde defaults
- ✅ Updated all test struct initializers to include new fields
- ✅ Added comprehensive tests for new functionality

**Acceptance Criteria**:
- ✅ Configuration can be loaded from JSON with Google Sheets settings (tested in `test_config_image_fields_and_resolve_paths`)
- ✅ Missing Google Sheets settings use appropriate defaults ("Scouting" sheet, empty image_file)
- ✅ Existing config files continue to work without modification (backward compatibility maintained)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (linting passes)

## ✅ Step 3: Add Google Sheets Dependencies to Image Binary - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Import and initialize Google Sheets functionality.

**Details**:
- ✅ Added Google Sheets functionality through existing `AppRunnerBuilder` infrastructure
- ✅ Added new `setup_for_image_processor` method to `AppRunner` that handles:
  - Path resolution using `config.resolve_image_paths()`
  - Google Sheets authentication setup
  - Progress tracking integration
- ✅ Updated main function to:
  - Use the new setup method with proper CLI argument passing
  - Handle resolved paths for spreadsheet, credentials, image file, and sheet name
  - Prepare application structure for both stdout and Google Sheets output
- ✅ Maintained backward compatibility and proper error handling patterns

**Acceptance Criteria**:
- ✅ Code compiles with new imports (builds successfully)
- ✅ No runtime errors when running with existing arguments (validated)
- ✅ New arguments are properly parsed and stored (help shows all Google Sheets arguments)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 4: Implement Google Sheets Authentication - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Add Google Sheets authentication to the image processing workflow.

**Details**:
- ✅ Used existing authentication infrastructure through `AppRunner.complete_authentication()`
- ✅ Added progress indicator for authentication step (at 95% progress)
- ✅ Implemented stdout-first principle: always output to stdout, then attempt Google Sheets operations
- ✅ Added graceful error handling: authentication failures are logged as warnings, not fatal errors
- ✅ Added conditional authentication: only attempts when Google Sheets arguments are provided
- ✅ Separated path resolution from authentication for better control flow
- ✅ Added helper function `attempt_google_sheets_upload()` for clean separation of concerns

**Acceptance Criteria**:
- ✅ Authentication works with existing credential files (uses same `complete_authentication` method)
- ✅ Progress bar shows authentication step (at 95% completion)
- ✅ Authentication errors are handled appropriately (warnings, not failures)
- ✅ Stdout output always works regardless of authentication status
- ✅ Tool works with and without Google Sheets arguments (backward compatibility maintained)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 5: Implement Sheet Validation - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Validate that the "Scouting" sheet exists in the spreadsheet.

**Details**:
- ✅ Added sheet validation after authentication using `sheets_manager.verify_sheet_exists()`
- ✅ Used existing `SheetsManager::verify_sheet_exists()` method for consistency with other tools
- ✅ Added progress indicator at 96% for validation step with descriptive message
- ✅ Added proper error handling with descriptive messages including sheet and spreadsheet IDs
- ✅ Error handling follows stdout-first principle (authentication failures are handled gracefully)
- ✅ Fixed trait compatibility issues by using `app_runner.progress_reporter()` instead of `app_runner.progress()`
- ✅ Code follows same patterns as other binaries (`fm_team_selector` and `player_uploader`)

**Acceptance Criteria**:
- ✅ Successfully identifies when target sheet exists (uses existing `verify_sheet_exists` logic)
- ✅ Raises clear error when target sheet is missing (descriptive error with sheet and spreadsheet info)
- ✅ Progress bar shows validation step (at 96% with descriptive message)
- ✅ Error handling follows the stdout-first principle (preserves stdout output on authentication/validation failures)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 6: Implement Data Reading from Sheet - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Read existing player data from the "Scouting" sheet.

**Details**:
- ✅ Added `read_existing_data()` function that reads data from range A4:AX104 (101 rows total for player data)
- ✅ Uses existing `SheetsManager::read_data()` method for consistency with other tools
- ✅ Added `create_player_name_mapping()` function to parse player names from column A
- ✅ Creates HashMap mapping player names to their sheet row numbers (1-based indexing)
- ✅ Handles empty cells and missing data appropriately using trim() and empty checks
- ✅ Added progress indicator at 97% with descriptive message "Reading existing player data..."
- ✅ Added comprehensive debug logging for data reading and mapping creation
- ✅ Integrated into the `attempt_google_sheets_upload()` workflow

**Acceptance Criteria**:
- ✅ Successfully reads existing data from the sheet using range A4:AX104
- ✅ Correctly identifies player names from column A and maps them to row numbers (4-104)
- ✅ Handles sheets with no existing data (empty HashMap created)
- ✅ Progress tracking shows data reading step at 97% completion
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy suggested `row.first()` instead of `row.get(0)`, implemented)

## ✅ Step 7: Implement Row Finding Logic - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Implement logic to find target row for player data.

**Details**:
- ✅ Created `find_target_row()` function that takes player name, existing data mapping, and existing data
- ✅ Implemented logic to check if player exists in mapping and return existing row number
- ✅ Added scanning for first completely empty row when player is new (no data in columns A:AX)
- ✅ Scanning covers rows 4-104 (101 total rows) as specified
- ✅ Returns 1-based sheet row numbers for Google Sheets consistency
- ✅ Added comprehensive error handling for edge case when no empty rows are available
- ✅ Integrated row finding into `attempt_google_sheets_upload()` workflow
- ✅ Added debug logging for troubleshooting player lookups and empty row detection

**Acceptance Criteria**:
- ✅ Correctly identifies existing players for updates using HashMap lookup
- ✅ Finds first empty row when player is new by scanning all cells in each row
- ✅ Handles edge cases with descriptive error when no empty rows available (rows 4-104 full)
- ✅ Row numbering is consistent with Google Sheets (1-based, converts from 0-based array indices)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 8: Implement TSV to Cells Conversion - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Convert TSV output format to individual cell values.

**Details**:
- ✅ Created `convert_tsv_to_cells()` function that takes TSV string and returns vector of cell values
- ✅ Splits TSV by tab characters using `split('\t')` and converts to owned strings
- ✅ Added validation for expected column count (50 columns: A through AX)
- ✅ Added proper error handling with descriptive messages for invalid column counts
- ✅ Added debug logging for troubleshooting including sample values and conversion stats
- ✅ Integrated into `attempt_google_sheets_upload()` workflow with progress tracking at 98%
- ✅ Added comprehensive error handling using `FMDataError::image` for format errors

**Acceptance Criteria**:
- ✅ TSV string correctly splits into individual cell values using tab separation
- ✅ Handles edge cases like empty fields (preserved as empty strings in the vector)
- ✅ Output array validates correct length for columns A:AX (exactly 50 columns)
- ✅ No data loss or corruption during conversion (trim() applied to remove whitespace)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 9: Implement Data Upload to Sheet - COMPLETED

**File**: `src/bin/fm_image.rs` and `src/sheets_client.rs`

**Task**: Upload player data to the determined row in Google Sheets.

**Details**:
- ✅ Added new `upload_data_to_range()` method to `SheetsManager` that allows writing to specific ranges
- ✅ Method uses same underlying Google Sheets API (`values_update`) as existing methods
- ✅ Added proper data validation using existing `DataValidator` infrastructure
- ✅ Created `upload_player_data()` function that formats range (e.g., "A5:AX5") and calls the new method
- ✅ Added progress indicator at 99% completion with descriptive message showing target row
- ✅ Added comprehensive error handling with descriptive messages including row number and permissions guidance
- ✅ Integrated into `attempt_google_sheets_upload()` workflow after TSV conversion
- ✅ Follows stdout-first principle: upload happens only after stdout output succeeds

**Acceptance Criteria**:
- ✅ Data successfully uploads to correct row using range-specific upload method
- ✅ Existing player data gets updated properly (uses existing row number from mapping)
- ✅ New players get added to first empty row (determined by row finding logic)
- ✅ Upload errors are handled with appropriate messages and don't break stdout output
- ✅ Progress bar shows upload progress at 99% with descriptive message
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 10: Integrate Upload into Main Workflow - COMPLETED

**File**: `src/bin/fm_image.rs`

**Task**: Integrate Google Sheets upload into the main image processing workflow.

**Details**:
- ✅ Main function performs Google Sheets operations after image processing and stdout output
- ✅ Stdout output happens first, preserving existing behavior (lines 162-169)
- ✅ Google Sheets operations only attempt when required arguments are provided (lines 172-192)
- ✅ Added comprehensive progress tracking covering the entire workflow (10% → 99%)
- ✅ Maintained error handling: stdout always works, upload errors are logged as warnings
- ✅ Integrated `attempt_google_sheets_upload()` function with complete workflow
- ✅ Added graceful error handling that doesn't break existing functionality

**Acceptance Criteria**:
- ✅ Image processing and stdout output work exactly as before (backward compatible)
- ✅ Google Sheets upload happens as additional step after stdout output
- ✅ Progress bar covers entire workflow from image loading to upload completion
- ✅ Tool works with and without Google Sheets arguments (help shows all options)
- ✅ Error handling follows stdout-first principle (upload failures are warnings, not fatal errors)
- ✅ All 174 unit tests + 17 integration tests pass
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 11: Add Integration Tests - COMPLETED

**File**: `tests/integration_tests.rs`

**Task**: Create tests for the new Google Sheets upload functionality.

**Details**:
- ✅ Created comprehensive integration tests covering:
  - `test_image_upload_new_player`: Successful new player upload workflow with path resolution and AppRunnerBuilder
  - `test_image_tsv_conversion`: TSV data conversion to 50-column spreadsheet format (A:AX)
  - `test_image_row_finding`: Row finding logic for existing vs new players in sheet range 4-104
  - `test_image_upload_missing_sheet_error`: Error handling for missing credentials/authentication failures
  - `test_image_upload_config_integration`: Configuration loading and path resolution from config files
  - `test_image_upload_backward_compatibility`: Tool works without Google Sheets arguments (default values)
  - `test_image_upload_file_format_validation`: PNG file format validation with appropriate error handling
- ✅ Added 7 new integration tests using existing test infrastructure and patterns
- ✅ Tests use temporary files (PNG headers, config files, credentials) to avoid external dependencies
- ✅ Used existing `tempfile`, `tokio::fs`, and `Result<()>` patterns consistent with other tests
- ✅ Tests validate both success and error scenarios with appropriate assertions
- ✅ All tests run without requiring actual Google Sheets access or real credentials

**Acceptance Criteria**:
- ✅ Tests cover main success scenarios (path resolution, configuration loading, CLI argument parsing)
- ✅ Tests cover error scenarios (missing files, invalid formats, authentication failures)
- ✅ Tests integrate with existing test suite (24 total integration tests, all passing)
- ✅ Tests run without external dependencies (mock files and data used throughout)
- ✅ All 174 unit tests + 24 integration tests pass successfully
- ✅ Code quality maintained (clippy and fmt pass)

## ✅ Step 12: Update Documentation - COMPLETED

**File**: `CLAUDE.md`

**Task**: Update project documentation to reflect new functionality.

**Details**:
- ✅ Updated project overview to describe `fm_image` as extracting data "using OCR and optionally uploads to Google Sheets"
- ✅ Added Google Sheets upload examples in build commands section:
  - Basic Google Sheets upload with spreadsheet ID and credentials
  - Progress bar disabled mode with Google Sheets arguments
- ✅ Updated architecture description to include "Google Sheets Integration" in Image Processor features
- ✅ Enhanced usage examples with comprehensive Google Sheets scenarios:
  - Upload to default "Scouting" sheet
  - Upload to custom sheet name using `--sheet` argument
  - Stdout + Google Sheets upload with verbose logging
  - Batch processing with Google Sheets upload
  - Processing multiple screenshots (stdout only vs Google Sheets modes)
- ✅ Added comprehensive configuration examples:
  - Basic configuration (stdout output only)
  - Configuration with Google Sheets upload (spreadsheet_name, creds_file, scouting_sheet)
  - Full configuration showing all available options
- ✅ Added detailed troubleshooting section for Google Sheets upload issues:
  - Authentication failures (credentials, service account access)
  - Spreadsheet access errors (permissions, sheet existence)
  - Upload errors (empty rows, concurrent editing, sheet structure)
  - Configuration issues (CLI vs config priority, backward compatibility)
  - Data range conflicts (A4:AX104 range, existing vs new players)
  - Common solutions (stdout-first mode, progress bar handling, verbose debugging)
- ✅ Updated development notes with new data range validation for Image Processor
- ✅ Updated test count from 128 to 198 tests (174 unit + 24 integration)
- ✅ Maintained consistency with existing documentation style and format

**Acceptance Criteria**:
- ✅ Documentation accurately reflects new functionality across all sections
- ✅ Examples are complete and correct with real command-line usage patterns
- ✅ Troubleshooting covers common upload issues with actionable solutions
- ✅ Documentation style matches existing content (formatting, structure, terminology)
- ✅ All 174 unit tests + 24 integration tests pass successfully
- ✅ Code quality maintained (clippy and fmt pass)

## Implementation Notes

- **Dependencies**: No new external dependencies should be needed - reuse existing Google Sheets infrastructure
- **Error Handling**: Always prioritize stdout output, then handle upload errors
- **Progress Tracking**: Use existing progress bar infrastructure for consistency
- **Configuration**: Follow existing configuration patterns for Google Sheets settings
- **Testing**: Leverage existing test infrastructure and mocking patterns
- **Backward Compatibility**: Tool must work exactly as before when Google Sheets arguments are not provided