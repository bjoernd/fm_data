## Overview

We are going to implement a way to upload data read from a screenshot image into Google Sheets. The `fm_image` tool will continue to print results to stdout AND additionally upload the data to Google Sheets.

## Google Sheets Interaction

The package already contains configurations for authenticating with the Google Sheets API and selecting a spreadsheet. We are going to reuse this infrastructure and use the same CLI arguments and configuration system as the other tools (e.g., `--credfile`, `--spreadsheet`, config.json).

## Feature Implementation

### Core Functionality
The `fm_image` tool currently prints TSV results to stdout. We will extend it to:
1. **Always print to stdout** (preserve existing behavior)
2. **Additionally upload to Google Sheets** using the same TSV data

### Google Sheets Integration
- **Target sheet**: "Scouting" sheet within the configured spreadsheet
- **Data range**: Columns A to AX (matching the TSV output format exactly)
- **Data placement**: Starting from row 4 or later
- **Data format**: Each tab-separated value from the TSV output goes into one cell

### Update/Insert Logic
Player data will be either updated or inserted based on player name matching:

1. **Player identification**: Match by exact player name in column A (case-sensitive)
2. **Search scope**: Check only the first 100 rows of the "Scouting" sheet
3. **Update existing**: If player name found, overwrite that entire row with new data
4. **Insert new**: If player not found, add data to the first completely empty row
5. **Empty row definition**: A row with NO data in any column from A to AX

### Error Handling
- **Missing "Scouting" sheet**: Raise error and terminate (still print to stdout first)
- **Authentication failure**: Raise error and terminate (still print to stdout first)
- **Network/upload failure**: Raise error and terminate (still print to stdout first)
- **Principle**: Always attempt stdout output before Google Sheets operations

### Progress Feedback
Include progress indicators for:
- Authentication process
- Sheet validation
- Data upload operation
- Follow the same progress bar patterns as other tools

### Technical Requirements
- **No data transformation**: TSV format maps directly to spreadsheet cells
- **No concurrency handling**: Assume single-instance usage
- **Exact name matching**: No case-insensitive or fuzzy matching
- **CLI compatibility**: Use identical argument parsing and configuration as existing tools
