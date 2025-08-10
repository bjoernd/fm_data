use clap::Parser;
use fm_data::error::{FMDataError, Result};
use fm_data::error_messages::{ErrorBuilder, ErrorCode};
use fm_data::{
    format_player_data, format_player_data_verbose, load_image, parse_player_from_ocr,
    AppRunnerBuilder, CLIArgumentValidator, CommonCLIArgs, ImageCLI, ImageProcessor,
};
use log::{debug, info};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Extract player data from Football Manager PNG screenshots using OCR",
    long_about = "A tool to extract player data from Football Manager PNG screenshots using OCR processing.
The tool processes PNG screenshots of player attributes pages and outputs tab-separated player data.

The screenshot should show a player's attributes page with all technical, mental, physical, 
and (optionally) goalkeeping attributes visible. The tool uses Tesseract OCR to extract 
text from the image and parse it into structured player data.

Examples:
    # Basic usage with screenshot file
    fm_image -i player_screenshot.png
    
    # With configuration file
    fm_image -i screenshot.png -c my_config.json
    
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
    let cli = CLIArguments::parse();

    // Use AppRunnerBuilder for consolidated setup
    let app_runner = AppRunnerBuilder::from_cli(&cli, "fm_image")
        .build_basic()
        .await?;

    // Get the image file path from CLI
    let image_file_path = cli.common.image_file.as_ref().unwrap();

    info!(
        "Processing Football Manager screenshot: {}",
        image_file_path
    );

    // Step 1: Load and validate PNG image file
    app_runner
        .progress()
        .update(10, 100, "Loading PNG screenshot...");

    let _image = load_image(image_file_path).map_err(|e| {
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

    let ocr_text = processor.extract_text(image_file_path).map_err(|e| {
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

    let player = parse_player_from_ocr(&ocr_text, image_file_path).map_err(|e| {
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

    // Output the player data to stdout - use verbose format if verbose mode is enabled
    if cli.is_verbose() {
        println!("\nDetailed Player Data:");
        println!("{}", format_player_data_verbose(&player));
        println!("\nTab-separated output:");
    }

    println!("{formatted_output}");

    Ok(())
}
