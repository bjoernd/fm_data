# FM Image Tool Implementation Plan

This document outlines the step-by-step implementation plan for the `fm_image` tool that extracts player data from Football Manager PNG screenshots using OCR processing.

## Overview

The `fm_image` tool will integrate with the existing fm_data architecture, reusing CLI patterns, configuration systems, authentication, and progress tracking. It will use Tesseract OCR for local image processing and output tab-separated player data.

## Prerequisites

- Tesseract OCR library installed on the system
- Rust dependencies: `tesseract` crate, `image` crate for PNG processing
- Integration with existing fm_data CLI and configuration infrastructure

## Implementation Steps

### Step 1: Add Dependencies and Basic Structure ✅ COMPLETED
**Goal**: Set up project dependencies and create the basic binary structure

**Tasks**:
1. ✅ Add new dependencies to `Cargo.toml`:
   - `tesseract = "0.14"` for OCR processing
   - `image = "0.25"` for PNG image handling
   - Consider `imageproc` for image preprocessing if needed
2. ✅ Create `src/bin/fm_image.rs` following existing binary patterns
3. ✅ Add basic CLI structure using existing `CommonCLIArgs` trait
4. ✅ Create placeholder main function with proper error handling

**Testing Requirements**:
- ✅ Binary compiles successfully: `cargo build --bin fm_image`
- ✅ Basic CLI help works: `cargo run --bin fm_image -- --help`
- ✅ All existing tests still pass: `cargo test`
- ✅ Linting passes: `cargo clippy`

**Validation**:
- ✅ Ensure Tesseract system dependency is available
- ✅ Verify binary is properly registered in `Cargo.toml`

**Implementation Notes**:
- Added Tesseract OCR and Leptonica system dependencies via Homebrew
- Created `ImageCLI` struct with PNG file validation and basic Google integration
- Binary properly integrates with existing `AppRunnerBuilder` patterns
- PNG file validation includes magic byte checking for file format verification

### Step 2: Extend CLI Interface for Image Input
**Goal**: Extend the CLI system to support image file input parameter

**Tasks**:
1. Create new CLI variant in `src/cli.rs`:
   - Add `ImageCLI` struct with image file parameter
   - Implement `CommonCLIArgs` trait for `ImageCLI`
   - Add validation for PNG file existence and readability
2. Update `src/bin/fm_image.rs` to use the new CLI interface
3. Integrate with existing `AppRunnerBuilder` pattern
4. Add proper help text and examples following existing patterns

**Testing Requirements**:
- Unit tests for CLI argument parsing and validation
- Test file existence validation
- Test invalid file path handling
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- CLI accepts PNG file parameter correctly
- Proper error messages for missing/invalid files
- Integration with existing config system works

### Step 3: Create Image Processing Module
**Goal**: Implement core OCR functionality for extracting text from PNG screenshots

**Tasks**:
1. Create `src/image_processor.rs` module with:
   - Function to load PNG images using `image` crate
   - Tesseract OCR initialization and configuration
   - Text extraction from image regions
   - Basic image preprocessing (if needed for OCR accuracy)
2. Add module exports to `src/lib.rs`
3. Implement error handling for OCR failures and image loading issues
4. Add logging for debugging OCR extraction process

**Testing Requirements**:
- Unit tests with mock/test PNG images
- Test OCR text extraction accuracy
- Test error handling for invalid images
- Test error handling for OCR failures
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- OCR can extract basic text from test PNG images
- Proper error messages for OCR and image loading failures
- Module integrates with existing error handling patterns

### Step 4: Implement Player Data Structure and Parsing
**Goal**: Define data structures for extracted player information and parsing logic

**Tasks**:
1. Create `src/image_data.rs` module with:
   - `ImagePlayer` struct containing all required attributes (name, age, footedness, all attributes)
   - `PlayerType` enum (Goalkeeper, FieldPlayer)
   - Footedness enum (`LeftFooted`, `RightFooted`, `BothFooted`)
2. Implement parsing functions:
   - Extract player name and age from OCR text
   - Detect player type (presence of "GOALKEEPING" section)
   - Parse attribute sections (TECHNICAL, MENTAL, PHYSICAL, GOALKEEPING)
   - Extract attribute name-value pairs from OCR text
3. Add comprehensive error handling for missing required data
4. Add module exports to `src/lib.rs`

**Testing Requirements**:
- Unit tests for data structure creation and validation
- Unit tests for parsing functions with mock OCR text
- Test error handling for missing required attributes
- Test parser robustness with various text formatting
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- All required attributes can be parsed from mock data
- Proper error reporting for missing sections/attributes
- Data structures align with output format requirements

### Step 5: Implement Footedness Detection
**Goal**: Extract player footedness from colored circle indicators in screenshots

**Tasks**:
1. Extend `src/image_processor.rs` with color detection:
   - Locate "LEFT FOOT" and "RIGHT FOOT" text regions
   - Find circles between these regions using image processing
   - Detect green/yellow color tones in circles
   - Implement color classification logic
2. Integrate footedness detection with player data parsing
3. Add comprehensive error handling for unclear colors
4. Add logging for debugging color detection process

**Testing Requirements**:
- Unit tests with mock images containing different colored circles
- Test both left-footed, right-footed, and both-footed scenarios
- Test error handling when colors cannot be determined
- Test robustness with different image qualities
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- Color detection works accurately with test images
- Proper error reporting when colors are unclear
- Integration with player data structure is seamless

### Step 6: Implement Output Formatting
**Goal**: Format extracted player data into required tab-separated output

**Tasks**:
1. Create `src/image_output.rs` module with:
   - Function to format `ImagePlayer` data into tab-separated string
   - Ensure exact attribute order matches specification
   - Handle missing attributes (output as 0)
   - Support both goalkeeper and field player data
2. Add comprehensive output validation
3. Add module exports to `src/lib.rs`
4. Integrate with main binary execution flow

**Testing Requirements**:
- Unit tests for output formatting with different player types
- Test exact order of attributes in output
- Test handling of missing attributes (0 values)
- Test tab separation formatting
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- Output format exactly matches specification
- Both goalkeeper and field player data output correctly
- Missing attributes properly handled as 0 values

### Step 7: Complete Main Binary Integration
**Goal**: Integrate all components into the main fm_image binary

**Tasks**:
1. Update `src/bin/fm_image.rs` with complete workflow:
   - Load and validate PNG image file
   - Initialize OCR processing with progress tracking
   - Extract player data using all implemented modules
   - Format and output results
   - Integrate with existing `AppRunner` patterns
2. Add comprehensive error handling and user feedback
3. Integrate with existing progress tracking system
4. Add proper logging throughout the process

**Testing Requirements**:
- Integration tests with complete end-to-end workflow
- Test with various PNG screenshot formats
- Test error handling at each stage
- Test progress tracking and user feedback
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- Complete workflow processes PNG screenshots successfully
- Proper integration with existing infrastructure
- User feedback and progress tracking work correctly

### Step 8: Add Comprehensive Error Handling and Validation
**Goal**: Ensure robust error handling throughout the application

**Tasks**:
1. Review and enhance error handling in all modules:
   - Specific error messages for each failure type
   - Proper error context propagation using `anyhow`
   - User-friendly error messages
2. Add input validation:
   - PNG file format validation
   - Image content validation (contains expected sections)
   - OCR quality validation
3. Add comprehensive logging for debugging
4. Update error types in `src/error.rs` if needed

**Testing Requirements**:
- Unit tests for all error conditions
- Test error message clarity and usefulness
- Test error handling in various failure scenarios
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- All error scenarios provide clear, actionable error messages
- Error handling follows existing patterns
- Logging provides sufficient debugging information

### Step 9: Create Integration Tests
**Goal**: Ensure the complete system works with real-world scenarios

**Tasks**:
1. Create `tests/image_integration_tests.rs`:
   - End-to-end tests with sample PNG screenshots
   - Test both goalkeeper and field player processing
   - Test various image qualities and formats
   - Test error scenarios (missing sections, bad images)
2. Create mock PNG test images for different scenarios
3. Test CLI integration and output formatting
4. Performance testing with various image sizes

**Testing Requirements**:
- All integration tests pass: `cargo test --test image_integration_tests`
- Tests cover both success and failure scenarios
- Performance benchmarks meet acceptable thresholds
- Run `cargo test` and `cargo clippy` successfully

**Validation**:
- Complete workflow tested with realistic scenarios
- Edge cases and error conditions properly covered
- Performance is acceptable for typical screenshot sizes

### Step 10: Update Documentation and Build System
**Goal**: Complete the implementation with proper documentation and build configuration

**Tasks**:
1. Update `CLAUDE.md` with:
   - New binary information and usage examples
   - Dependencies and system requirements
   - Build and test commands for fm_image
2. Update `Cargo.toml` binary configuration:
   - Ensure fm_image binary is properly configured
   - Add any needed feature flags or optional dependencies
3. Add example PNG screenshots to repository (if appropriate)
4. Create usage examples and help documentation

**Testing Requirements**:
- Documentation is accurate and complete
- Build system properly includes new binary
- All examples in documentation work correctly
- Final comprehensive test run: `cargo test` and `cargo clippy`

**Validation**:
- Documentation matches actual implementation
- Binary builds and runs correctly from fresh checkout
- All examples and usage patterns are valid

## Success Criteria

Each step must meet the following criteria before proceeding:

1. **Build Success**: `cargo build --release` completes without errors
2. **Test Success**: `cargo test` passes all existing and new tests
3. **Linting Success**: `cargo clippy` passes without warnings
4. **Integration**: New code follows existing architectural patterns
5. **Error Handling**: Comprehensive error handling with clear messages
6. **Documentation**: Code is properly documented with inline comments

## Dependencies to Add

```toml
# Add to [dependencies] section in Cargo.toml
tesseract = "0.14"
image = "0.25"

# Add to [[bin]] section
[[bin]]
name = "fm_image"
path = "src/bin/fm_image.rs"
```

## System Requirements

- Tesseract OCR library installed (`brew install tesseract` on macOS, `apt-get install tesseract-ocr` on Ubuntu)
- Tesseract language data for English (`tesseract-ocr-eng` package)

## Architecture Integration

The new tool will integrate with existing infrastructure:

- **CLI**: Reuses existing `CommonCLIArgs` patterns and `AppRunnerBuilder`
- **Config**: Integrates with existing configuration system for future extensibility
- **Error Handling**: Uses existing `FMDataError` and `anyhow` patterns
- **Progress Tracking**: Uses existing `ProgressTracker` and `ProgressReporter`
- **Authentication**: Includes Google OAuth parameters for future Google Sheets integration
- **Logging**: Uses existing `env_logger` configuration and verbosity levels

## Testing Strategy

- **Unit Tests**: Each module has comprehensive unit tests
- **Integration Tests**: End-to-end testing with real PNG screenshots
- **Error Testing**: Comprehensive testing of all failure modes
- **Performance Testing**: Ensure acceptable performance with typical screenshot sizes
- **Compatibility Testing**: Ensure existing functionality remains unaffected