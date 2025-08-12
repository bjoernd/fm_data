use clap::Parser;
use fm_data::error::{FMDataError, Result};
use fm_data::error_messages::{ErrorBuilder, ErrorCode};
use fm_data::{
    clipboard::ClipboardManager, format_player_data, format_player_data_verbose, load_image,
    parse_player_from_ocr, AppRunner, AppRunnerBuilder, CLIArgumentValidator, CommonCLIArgs,
    ImageCLI, ImageProcessor,
};
use log::{debug, error, info};
use std::collections::HashMap;
use tempfile::NamedTempFile;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Extract player data from Football Manager PNG screenshots using OCR",
    long_about = "A tool to extract player data from Football Manager PNG screenshots using OCR processing.
The tool processes PNG screenshots of player attributes pages and outputs tab-separated player data.

The screenshot should show a player's attributes page with all technical, mental, physical, 
and (optionally) goalkeeping attributes visible. The tool uses Tesseract OCR to extract 
text from the image and parse it into structured player data.

INPUT MODES:
    1. File mode: Provide a PNG file path with -i flag
    2. Clipboard mode: Copy image (Cmd+C) and run without -i flag

Examples:
    # File mode: Basic usage with screenshot file
    fm_image -i player_screenshot.png
    
    # Clipboard mode: Copy image first, then run
    fm_image
    
    # With configuration file
    fm_image -i screenshot.png -c my_config.json
    
    # Clipboard mode with Google Sheets upload
    fm_image -s SPREADSHEET_ID --credfile creds.json
    
    # Verbose mode for debugging OCR processing
    fm_image -i screenshot.png -v
    
    # Scripting mode (no progress bar)
    fm_image -i screenshot.png --no-progress"
)]
struct CLIArguments {
    #[command(flatten)]
    common: ImageCLI,
}

impl CLIArgumentValidator for CLIArguments {
    fn validate(&self) -> Result<()> {
        self.common.validate_common()
    }

    fn is_verbose(&self) -> bool {
        self.common.common.verbose
    }

    fn is_no_progress(&self) -> bool {
        self.common.common.no_progress
    }

    fn config_path(&self) -> &str {
        &self.common.common.config
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut cli = CLIArguments::parse();

    // Detect clipboard mode and automatically disable progress bar
    let is_clipboard_mode = cli.common.image_file.is_none();
    if is_clipboard_mode {
        // Automatically disable progress bar in clipboard mode for better UX
        cli.common.common.no_progress = true;
    }

    // Use AppRunnerBuilder for consolidated setup (without authentication)
    let mut app_runner = AppRunnerBuilder::from_cli(&cli, "fm_image")
        .build_basic()
        .await?;

    if is_clipboard_mode {
        info!("Clipboard mode detected: progress bar disabled for interactive experience");
    }

    // Handle image source: either file path or clipboard
    let (image_file_path, _temp_file): (String, Option<NamedTempFile>) =
        if let Some(image_file) = cli.common.image_file.clone() {
            // File mode: use provided image file path
            (image_file, None)
        } else {
            // Clipboard mode: wait for user to paste image
            info!("No image file provided, entering clipboard mode...");
            let mut clipboard_manager = ClipboardManager::new().map_err(|e| {
                error!("Failed to initialize clipboard manager: {}", e);
                e
            })?;

            let temp_file = clipboard_manager.wait_for_image_paste().map_err(|e| {
                error!("Failed to get image from clipboard: {}", e);
                e
            })?;

            let path = temp_file.path().to_string_lossy().to_string();
            (path, Some(temp_file))
        };

    // Resolve paths for potential Google Sheets upload
    let (spreadsheet_id, credfile_path, _, sheet_name) = app_runner
        .config
        .resolve_image_paths(
            cli.common.common.spreadsheet.clone(),
            cli.common.common.credfile.clone(),
            Some(image_file_path.clone()),
            Some(cli.common.sheet.clone()),
        )
        .map_err(|e| {
            error!("Configuration validation failed: {}", e);
            e
        })?;

    // Check if Google Sheets upload is configured
    let sheets_upload_configured = !spreadsheet_id.is_empty() && !credfile_path.is_empty();
    if sheets_upload_configured {
        info!(
            "Google Sheets upload configured: will upload to '{}' sheet after stdout output",
            sheet_name
        );
    }

    info!(
        "Processing Football Manager screenshot: {}",
        image_file_path
    );

    // Step 1: Load and validate PNG image file
    app_runner
        .progress()
        .update(10, 100, "Loading PNG screenshot...");

    let _image = load_image(&image_file_path).map_err(|e| {
        FMDataError::image(format!("Failed to load PNG image '{image_file_path}': {e}"))
    })?;

    debug!("PNG image loaded successfully from: {}", image_file_path);
    info!("Image file validated: {}", image_file_path);

    // Step 2: Extract text using OCR with new ImageProcessor
    app_runner
        .progress()
        .update(30, 100, "Extracting text with OCR...");

    // Create ImageProcessor with default configuration for robust processing
    let processor = ImageProcessor::with_defaults()
        .map_err(|e| FMDataError::image(format!("Failed to initialize image processor: {e}")))?;

    let ocr_text = processor.extract_text(&image_file_path).map_err(|e| {
        FMDataError::image(format!(
            "OCR text extraction failed for '{image_file_path}': {e}"
        ))
    })?;

    debug!("OCR extracted {} characters of text", ocr_text.len());
    info!("OCR text extraction completed");

    if ocr_text.trim().is_empty() {
        return Err(ErrorBuilder::new(ErrorCode::E605)
            .with_context("ensure screenshot contains clear, readable player attribute text")
            .build());
    }

    // Step 3: Parse player data from OCR text (includes footedness detection)
    app_runner
        .progress()
        .update(60, 100, "Parsing player data and detecting footedness...");

    let player = parse_player_from_ocr(&ocr_text, &image_file_path).map_err(|e| {
        FMDataError::image(format!("Failed to parse player data from OCR text: {e}"))
    })?;

    info!(
        "Parsed player: {} (age: {}, type: {:?}, footedness: {:?})",
        player.name, player.age, player.player_type, player.footedness
    );
    let attr_hashmap = player.attributes.to_hashmap();
    debug!("Player has {} attributes", attr_hashmap.len());

    // Step 4: Format output data
    app_runner
        .progress()
        .update(90, 100, "Formatting output data...");

    let formatted_output = format_player_data(&player);

    debug!(
        "Formatted output has {} fields",
        formatted_output.split('\t').count()
    );
    info!("Output formatting completed");

    app_runner.finish("Image processing");

    // Output the player data to stdout FIRST (preserve existing behavior)
    if cli.is_verbose() {
        println!("\nDetailed Player Data:");
        println!("{}", format_player_data_verbose(&player));
        println!("\nTab-separated output:");
    }

    println!("{formatted_output}");

    // Attempt Google Sheets upload if configured (only after stdout output)
    if sheets_upload_configured {
        match attempt_google_sheets_upload(
            &mut app_runner,
            &spreadsheet_id,
            &credfile_path,
            &sheet_name,
            &formatted_output,
            &player.name,
        )
        .await
        {
            Ok(()) => {
                info!("Successfully uploaded player data to Google Sheets");
            }
            Err(e) => {
                // Don't fail the entire program - stdout output already succeeded
                eprintln!("Warning: Google Sheets upload failed: {e}");
                debug!("Google Sheets upload error details: {:?}", e);
            }
        }
    }

    Ok(())
}

/// Attempt to upload player data to Google Sheets
async fn attempt_google_sheets_upload(
    app_runner: &mut AppRunner,
    spreadsheet_id: &str,
    credfile_path: &str,
    sheet_name: &str,
    _formatted_output: &str,
    _player_name: &str,
) -> Result<()> {
    // Complete authentication setup (this will create the sheets manager)
    app_runner
        .complete_authentication(spreadsheet_id.to_string(), credfile_path.to_string(), 95)
        .await
        .map_err(|e| {
            FMDataError::auth(format!(
                "Google Sheets authentication failed: {e}. Ensure credentials file '{credfile_path}' is valid and accessible."
            ))
        })?;

    info!("Google Sheets authentication completed successfully");
    debug!(
        "Will upload to spreadsheet: {}, sheet: {}",
        spreadsheet_id, sheet_name
    );

    // Step 2: Validate that the target sheet exists
    app_runner
        .progress()
        .update(96, 100, &format!("Validating sheet '{sheet_name}'..."));

    let sheets_manager = app_runner.sheets_manager()?;
    sheets_manager
        .verify_sheet_exists(sheet_name, app_runner.progress_reporter())
        .await
        .map_err(|e| {
            FMDataError::sheets_api(format!(
                "Sheet validation failed: {e}. Ensure sheet '{sheet_name}' exists in spreadsheet '{spreadsheet_id}'."
            ))
        })?;

    info!("Sheet '{}' validation completed successfully", sheet_name);

    // Step 3: Read existing data from sheet to find player names and row positions
    app_runner
        .progress()
        .update(97, 100, "Reading existing player data...");

    let existing_data = read_existing_data(sheets_manager, sheet_name).await?;
    let player_mapping = create_player_name_mapping(&existing_data);

    debug!(
        "Found {} existing players in sheet: {:?}",
        player_mapping.len(),
        player_mapping.keys().collect::<Vec<_>>()
    );
    info!(
        "Successfully read existing data from sheet '{}'",
        sheet_name
    );

    // Step 4: Find target row for the player data
    let target_row = find_target_row(_player_name, &player_mapping, &existing_data)?;

    info!(
        "Target row determined: player '{}' will be written to row {}",
        _player_name, target_row
    );

    // Step 5: Convert TSV output to individual cell values
    app_runner
        .progress()
        .update(98, 100, "Converting data for upload...");

    let cell_values = convert_tsv_to_cells(_formatted_output)?;

    debug!(
        "Converted TSV to {} cell values for upload",
        cell_values.len()
    );
    info!("Data conversion for upload completed");

    // Step 6: Upload data to the determined row
    app_runner.progress().update(
        99,
        100,
        &format!("Uploading player data to row {target_row}..."),
    );

    upload_player_data(sheets_manager, sheet_name, target_row, &cell_values).await?;

    info!(
        "Successfully uploaded player '{}' data to row {} in sheet '{}'",
        _player_name, target_row, sheet_name
    );

    app_runner.finish("Google Sheets upload");

    Ok(())
}

/// Read existing data from the Scouting sheet (range A4:AX104)
async fn read_existing_data(
    sheets_manager: &fm_data::SheetsManager,
    sheet_name: &str,
) -> Result<Vec<Vec<String>>> {
    // Read data from range A4:AX104 (first 100 rows starting from row 4)
    // This covers rows 4-104 which gives us 101 rows total for player data
    let range = "A4:AX104";

    let progress_reporter = &fm_data::progress::NoOpProgressReporter;
    let data = sheets_manager
        .read_data(sheet_name, range, progress_reporter)
        .await?;

    debug!("Read {} rows from range {sheet_name}!{range}", data.len());

    Ok(data)
}

/// Create a mapping of player names to their row numbers in the sheet
fn create_player_name_mapping(data: &[Vec<String>]) -> HashMap<String, usize> {
    let mut mapping = HashMap::new();

    for (index, row) in data.iter().enumerate() {
        // Player names are in column A (index 0)
        if let Some(player_name) = row.first() {
            let name = player_name.trim();
            if !name.is_empty() {
                // Row numbers are 1-based, and we start reading from row 4
                // So index 0 in our data corresponds to row 4 in the sheet
                let sheet_row_number = index + 4;
                mapping.insert(name.to_string(), sheet_row_number);
            }
        }
    }

    debug!("Created player name mapping with {} entries", mapping.len());
    mapping
}

/// Find the target row for player data upload
/// Returns the existing row number if player exists, or first empty row if player is new
fn find_target_row(
    player_name: &str,
    player_mapping: &HashMap<String, usize>,
    existing_data: &[Vec<String>],
) -> Result<usize> {
    // Check if player already exists
    if let Some(&existing_row) = player_mapping.get(player_name) {
        debug!("Player '{}' exists at row {}", player_name, existing_row);
        return Ok(existing_row);
    }

    debug!(
        "Player '{}' not found, searching for empty row",
        player_name
    );

    // If we have existing data, first scan for empty rows within the downloaded data
    for (index, row) in existing_data.iter().enumerate() {
        // Check if the entire row is empty (all cells are empty or whitespace-only)
        let is_empty_row = row.iter().all(|cell| cell.trim().is_empty());

        if is_empty_row {
            let target_row = index + 4; // Convert to 1-based sheet row number
            debug!(
                "Found empty row within existing data at position {}",
                target_row
            );
            return Ok(target_row);
        }
    }

    // No empty rows found within existing data
    // If we have fewer than 101 rows, the next available row is after the last returned row
    if existing_data.len() < 101 {
        let target_row = existing_data.len() + 4; // First empty row after existing data
        debug!(
            "No empty rows in existing data, using next available row {} (after {} existing rows)",
            target_row,
            existing_data.len()
        );
        return Ok(target_row);
    }

    // If we've reached here, no empty rows were found in the scanned range
    // This is an edge case that should be handled gracefully
    Err(FMDataError::sheets_api(
        "No empty rows available for new player data. The scouting sheet may be full (rows 4-104 are all occupied).".to_string()
    ))
}

/// Convert TSV output format to individual cell values for Google Sheets upload
/// Takes tab-separated string and returns vector of cell values for columns A through AX (50 columns)
fn convert_tsv_to_cells(tsv_output: &str) -> Result<Vec<String>> {
    // Split the TSV string by tab characters
    let values: Vec<String> = tsv_output
        .trim() // Remove any leading/trailing whitespace
        .split('\t')
        .map(|s| s.to_string()) // Convert to owned strings
        .collect();

    debug!("TSV string split into {} values", values.len());

    // Validate that we have the expected number of columns (A through AX = 50 columns)
    const EXPECTED_COLUMNS: usize = 50; // A-Z (26) + AA-AX (24) = 50 total columns

    if values.len() != EXPECTED_COLUMNS {
        return Err(FMDataError::image(format!(
            "TSV output contains {} values, but expected exactly {} columns (A through AX). Verify the image processing output format.",
            values.len(),
            EXPECTED_COLUMNS
        )));
    }

    // Log a few sample values for debugging (without exposing sensitive data)
    if !values.is_empty() {
        debug!(
            "Sample cell values: first='{}', second='{}', last='{}'",
            values.first().unwrap_or(&"<empty>".to_string()),
            values.get(1).unwrap_or(&"<empty>".to_string()),
            values.last().unwrap_or(&"<empty>".to_string())
        );
    }

    info!("Successfully converted TSV to {} cell values", values.len());
    Ok(values)
}

/// Upload player data to a specific row in the Google Sheets
/// Writes data to range starting from column A through AX for the given row
async fn upload_player_data(
    sheets_manager: &fm_data::SheetsManager,
    sheet_name: &str,
    target_row: usize,
    cell_values: &[String],
) -> Result<()> {
    // Format the write request for range (e.g., "A5:AX5")
    let range = format!("A{target_row}:AX{target_row}");

    debug!(
        "Uploading {} cell values to range: {}!{}",
        cell_values.len(),
        sheet_name,
        range
    );

    // Convert string values to the format expected by SheetsManager
    // For a single row upload, we need a matrix with one row
    let row_data = vec![cell_values.to_vec()];

    // Use the new SheetsManager upload_data_to_range method
    let progress_reporter = &fm_data::progress::NoOpProgressReporter;
    sheets_manager
        .upload_data_to_range(sheet_name, &range, row_data, progress_reporter)
        .await
        .map_err(|e| {
            FMDataError::sheets_api(format!(
                "Failed to upload player data to row {target_row}: {e}. Ensure the sheet has proper write permissions."
            ))
        })?;

    debug!("Successfully wrote data to range: {}!{}", sheet_name, range);
    Ok(())
}
