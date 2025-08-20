# Player Browser Implementation Plan

This document outlines the step-by-step implementation plan for the `fm_player_browser` tool based on the specifications in `feature-player-browser.md`.

## Implementation Overview

The implementation is structured in 6 phases, progressing from basic infrastructure to the complete terminal UI application.

## Phase 1: Foundation and Data Structures

### Step 1.1: Add ratatui Dependencies **[COMPLETED]**
**File**: `Cargo.toml`
- Add `ratatui` dependency with required features
- Add `crossterm` for cross-platform terminal control
- Update feature flags if needed for the new binary

**Implementation**:
```toml
[dependencies]
ratatui = "0.24"
crossterm = "0.27"
```

**Test**: `cargo build` should succeed without errors

### Step 1.2: Create Player Browser Data Structure **[COMPLETED]**
**File**: `src/browser.rs` (new module)
- Create `BrowserPlayer` struct with all 147 fields
- Fields: Name(String), Age(f64), Foot(String), 47 FM attributes(f64), DNA(f64), 96 role ratings(f64)
- Implement validation for non-empty names
- Add methods for parsing from spreadsheet row data

**Implementation**:
```rust
pub struct BrowserPlayer {
    // Player Metadata (Columns A-C)
    pub name: String,
    pub age: f64,
    pub foot: String,
    
    // Technical Attributes (Columns D-Q) - 14 attributes
    pub corners: f64,
    pub crossing: f64,
    // ... (all 14 technical attributes)
    
    // Mental Attributes (Columns R-AE) - 14 attributes
    pub aggression: f64,
    pub anticipation: f64,
    // ... (all 14 mental attributes)
    
    // Physical Attributes (Columns AF-AJ) - 8 attributes
    pub acceleration: f64,
    pub agility: f64,
    // ... (all 8 physical attributes)
    
    // Goalkeeping Attributes (Columns AK-AR) - 11 attributes
    pub aerial_reach: f64,
    pub command_of_area: f64,
    // ... (all 11 goalkeeping attributes)
    
    // Additional Data (Column AS)
    pub dna: f64,
    
    // Position Ratings (Columns AT-EQ) - 96 ratings
    pub w_s_r: f64,
    pub w_s_l: f64,
    // ... (all 96 role ratings in exact spreadsheet order)
}

impl BrowserPlayer {
    pub fn from_row(row: &[String]) -> Option<Self> {
        // Validate name is not empty
        // Parse all 147 values with proper error handling
        // Convert "--" values to appropriate defaults
    }
    
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
    }
}
```

**Test**: Create unit tests in `src/browser.rs` for:
- Valid player parsing from row data
- Invalid name handling (empty/whitespace)
- "--" value handling
- All field parsing accuracy

**Commands**: `cargo test browser::tests --lib`

### Step 1.3: Create Column Header Constants
**File**: `src/browser.rs`
- Create static array with all 147 column headers in exact order
- Headers should match the attribute short names from specification

**Implementation**:
```rust
pub const COLUMN_HEADERS: [&str; 147] = [
    "Name", "Age", "Foot",
    "Cor", "Cro", "Dri", "Fin", "Fir", "Fre", "Hea", "Lon", "L Th", "Mar", "Pas", "Pen", "Tck", "Tec",
    "Agg", "Ant", "Bra", "Cmp", "Cnt", "Dec", "Det", "Fla", "Ldr", "OtB", "Pos", "Tea", "Vis", "Wor",
    "Acc", "Agi", "Bal", "Jum", "Nat", "Pac", "Sta", "Str",
    "Aer", "Cmd", "Com", "Ecc", "Han", "Kic", "1v1", "Pun", "Ref", "Rus", "Thr",
    "DNA",
    // All 96 role ratings...
    "W(s) R", "W(s) L", "W(a) R", "W(a) L", // ... rest of roles
];
```

**Test**: Verify header count is exactly 147
**Commands**: `cargo test`

### Step 1.4: Add Browser Module to Library
**File**: `src/lib.rs`
- Add `pub mod browser;` 
- Export key types for the binary to use

**Commands**: `cargo build --lib`

## Phase 2: Google Sheets Integration

### Step 2.1: Create Browser Data Repository **[COMPLETED]**
**File**: `src/browser_repository.rs` (new module)
- Create `BrowserRepository` struct that extends existing sheets functionality
- Method to fetch A2:EQ58 range data
- Convert raw spreadsheet data to `Vec<BrowserPlayer>`
- Filter out players with empty names
- Handle authentication and network errors

**Implementation**:
```rust
use crate::{browser::BrowserPlayer, sheets_client::SheetsClient};

pub struct BrowserRepository {
    sheets_client: SheetsClient,
}

impl BrowserRepository {
    pub fn new(sheets_client: SheetsClient) -> Self {
        Self { sheets_client }
    }
    
    pub async fn fetch_players(&self, spreadsheet_id: &str, sheet_name: &str) -> anyhow::Result<Vec<BrowserPlayer>> {
        let range = format!("{}!A2:EQ58", sheet_name);
        let values = self.sheets_client.get_values(spreadsheet_id, &range).await?;
        
        let players: Vec<BrowserPlayer> = values
            .into_iter()
            .filter_map(|row| BrowserPlayer::from_row(&row))
            .filter(|player| player.is_valid())
            .collect();
            
        Ok(players)
    }
}
```

**Test**: Create integration tests in `tests/` directory:
- Mock Google Sheets responses
- Test player filtering (valid names only)
- Test data parsing accuracy
- Test error handling for network/auth issues

**Commands**: `cargo test --test integration_tests`

### Step 2.2: Update Configuration System **[COMPLETED]**
**File**: `src/config.rs`
- Add `browser_sheet` field to config structure
- Default value should be "Squad" or similar
- Update CLI parsing if needed

**Test**: Verify config parsing works with new field
**Commands**: `cargo test config::tests --lib`

### Step 2.3: Run Tests and Linting
**Commands**: 
```bash
cargo test
cargo clippy --allow-dirty --fix
cargo fmt
```

## Phase 3: Basic Terminal UI Structure

### Step 3.1: Create Browser UI Module **[COMPLETED]**
**File**: `src/browser_ui.rs` (new module)
- Create `BrowserApp` struct with ratatui integration
- Basic terminal setup and cleanup
- Simple table widget creation with headers
- Basic event loop structure (no navigation yet)

**Implementation**:
```rust
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Table, Row, Cell},
    Terminal, Frame, layout::Constraint,
};
use std::io;

pub struct BrowserApp {
    players: Vec<BrowserPlayer>,
    should_quit: bool,
}

impl BrowserApp {
    pub fn new(players: Vec<BrowserPlayer>) -> Self {
        Self {
            players,
            should_quit: false,
        }
    }
    
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;
            
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }
            
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    
    fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        // Create table with all players and headers
        // Basic rendering without navigation
    }
}
```

**Test**: Create basic UI tests (terminal setup/teardown)
**Commands**: `cargo test browser_ui::tests --lib`

### Step 3.2: Implement Basic Table Rendering
**File**: `src/browser_ui.rs`
- Implement `draw()` method with ratatui Table widget
- Display all player data in table format
- Show column headers
- Handle string vs numeric formatting (2 decimal places)

**Test**: Manual testing with sample data
**Commands**: Create a simple test that verifies table creation

### Step 3.3: Run Tests and Linting
**Commands**:
```bash
cargo test
cargo clippy --allow-dirty --fix  
cargo fmt
```

## Phase 4: Navigation and Scrolling

### Step 4.1: Add Navigation State
**File**: `src/browser_ui.rs`
- Add `selected_row` and `selected_col` fields to `BrowserApp`
- Implement boundary checking (0 to players.len()-1, 0 to 146)
- Add methods for moving cursor up/down/left/right

**Implementation**:
```rust
pub struct BrowserApp {
    players: Vec<BrowserPlayer>,
    selected_row: usize,
    selected_col: usize,
    should_quit: bool,
}

impl BrowserApp {
    fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
        }
    }
    
    fn move_down(&mut self) {
        if self.selected_row < self.players.len().saturating_sub(1) {
            self.selected_row += 1;
        }
    }
    
    fn move_left(&mut self) {
        if self.selected_col > 0 {
            self.selected_col -= 1;
        }
    }
    
    fn move_right(&mut self) {
        if self.selected_col < 146 {
            self.selected_col += 1;
        }
    }
}
```

### Step 4.2: Implement Arrow Key Handling
**File**: `src/browser_ui.rs`
- Update event handling in `run()` method
- Map arrow keys to navigation methods
- Ensure 'q' and 'Esc' still work for quitting

### Step 4.3: Add Cell Selection Highlighting
**File**: `src/browser_ui.rs`
- Update `draw()` method to highlight selected cell
- Use ratatui styling for visual feedback
- Ensure selected cell is visible (basic scrolling logic)

**Test**: Create navigation tests:
- Test boundary conditions
- Test arrow key event handling
- Test selection highlighting

**Commands**: `cargo test browser_ui::tests --lib`

### Step 4.4: Run Tests and Linting
**Commands**:
```bash
cargo test
cargo clippy --allow-dirty --fix
cargo fmt
```

## Phase 5: Advanced Scrolling and Display

### Step 5.1: Implement Viewport Scrolling
**File**: `src/browser_ui.rs`
- Add viewport offset fields for both rows and columns
- Calculate visible area based on terminal size
- Implement scrolling logic to keep selected cell in view
- Handle horizontal scrolling for wide tables

**Implementation**:
```rust
pub struct BrowserApp {
    players: Vec<BrowserPlayer>,
    selected_row: usize,
    selected_col: usize,
    row_offset: usize,
    col_offset: usize,
    should_quit: bool,
}

impl BrowserApp {
    fn update_viewport(&mut self, terminal_height: u16, terminal_width: u16) {
        // Calculate visible rows/columns
        // Adjust offsets to keep selection visible
    }
}
```

### Step 5.2: Implement Data Formatting
**File**: `src/browser_ui.rs`
- Create methods to format different data types
- Numeric values: 2 decimal places
- String values: as-is
- Handle "--" values properly

### Step 5.3: Add Loading State
**File**: `src/browser_ui.rs`
- Create loading screen while fetching data
- Use ratatui progress indicator or spinner
- Display meaningful loading messages

**Test**: Test viewport calculations and scrolling behavior
**Commands**: `cargo test browser_ui::tests --lib`

### Step 5.4: Run Tests and Linting
**Commands**:
```bash
cargo test
cargo clippy --allow-dirty --fix
cargo fmt
```

## Phase 6: Binary Implementation and Integration

### Step 6.1: Create Binary Entry Point
**File**: `src/bin/fm_player_browser.rs`
- Create main binary that integrates all components
- Set up terminal, authentication, data fetching, and UI
- Use existing CLI and config system
- Handle errors gracefully (abort on auth errors)

**Implementation**:
```rust
use anyhow::Result;
use fm_data::{
    app_runner::AppRunner, 
    browser_repository::BrowserRepository,
    browser_ui::BrowserApp,
    cli::CommonCLI,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CommonCLI::parse();
    
    // Set up terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let result = run_app(&mut terminal, cli).await;
    
    // Cleanup terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    result
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>, 
    cli: CommonCLI
) -> Result<()> {
    // Create app runner (reuse existing infrastructure)
    let app_runner = AppRunner::new(cli).await?;
    let sheets_client = app_runner.create_sheets_client().await?;
    
    // Fetch player data
    let repository = BrowserRepository::new(sheets_client);
    let players = repository.fetch_players(
        &app_runner.config.google.spreadsheet_name,
        &app_runner.config.google.browser_sheet.unwrap_or("Squad".to_string())
    ).await?;
    
    // Run UI
    let mut app = BrowserApp::new(players);
    app.run(terminal)?;
    
    Ok(())
}
```

### Step 6.2: Update Cargo.toml for New Binary
**File**: `Cargo.toml`
- Add binary configuration for `fm_player_browser`

**Implementation**:
```toml
[[bin]]
name = "fm_player_browser"
path = "src/bin/fm_player_browser.rs"
```

### Step 6.3: Add Integration Tests
**File**: `tests/browser_integration_tests.rs`
- Test end-to-end functionality with mock data
- Test error handling paths
- Test configuration integration

### Step 6.4: Update Documentation
**File**: `CLAUDE.md`
- Add documentation for the new binary
- Update build commands section
- Add usage examples

**Implementation**: Add entries like:
```bash
# Run the player browser
cargo run --bin fm_player_browser

# Run with custom config
cargo run --bin fm_player_browser -- -c browser_config.json
```

### Step 6.5: Final Testing and Validation
**Commands**:
```bash
# Run all tests
cargo test

# Test the new binary specifically
cargo build --release --bin fm_player_browser

# Integration test
cargo test --test browser_integration_tests

# Clippy and formatting
cargo clippy --allow-dirty --fix
cargo fmt

# Verify all binaries still work
cargo build --release
```

## Testing Strategy

### Unit Tests
- Data structure parsing and validation (`src/browser.rs`)
- Navigation logic and boundary checking (`src/browser_ui.rs`)
- Repository data fetching (`src/browser_repository.rs`)

### Integration Tests
- End-to-end data flow from Google Sheets to UI
- Configuration system integration
- Error handling scenarios

### Manual Testing
- Terminal UI functionality with real data
- Navigation and scrolling behavior
- Display formatting and visual appearance
- Performance with full dataset (57 × 147 data points)

## Success Criteria

1. **Binary Creation**: `fm_player_browser` binary successfully builds and runs
2. **Data Integration**: Successfully fetches and parses all 147 columns from A2:EQ58
3. **UI Functionality**: Smooth navigation with arrow keys, proper cell highlighting
4. **Error Handling**: Graceful handling of authentication and network errors
5. **Configuration**: Seamless integration with existing config system
6. **Performance**: Responsive UI with full dataset loaded
7. **Testing**: All tests pass, clippy clean, proper formatting

## Implementation Notes

- Follow existing code patterns from `fm_team_selector` and `fm_google_up`
- Reuse authentication, configuration, and Google Sheets client infrastructure  
- Implement proper error handling consistent with existing tools
- Maintain code quality standards (tests, clippy, formatting)
- Document new functionality in CLAUDE.md following established patterns