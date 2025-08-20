# Player Browser

We are going to implement a new tool that allows us to browse the attributes of
all players from the Google Spreadsheet.

## Technical Architecture

- **Binary**: New binary called `fm_player_browser`
- **Data Structures**: Expand existing player data structures to combine player attributes with per-position strength ratings
- **Configuration**: Same configuration system as other tools (JSON config file, CLI arguments)
- **Authentication**: Reuse existing Google Sheets authentication system

## Player Data Source

The player browser will read squad data from a Google squad spreadsheet using range **A2:EQ58**.

- **Spreadsheet**: Same spreadsheet used by other tools (configurable via settings)
- **Sheet Name**: Configurable sheet name (following existing pattern)
- **Data Range**: A2:EQ58 - includes all player attributes and per-position ratings
- **Authentication**: Uses same Google OAuth2 flow as `fm_google_up` and `fm_team_selector`

## Browser UI

The browser will be a terminal-based UI with the following characteristics:

### Table Display
- **Single View**: All data displayed in one scrollable table
- **Column Headers**: Attribute short names
- **Data Format**: 
  - Attribute values as floats with 2 decimal digits
  - Player name and footedness as strings
  - All player data visible through scrolling

### Navigation
- **Vertical Scrolling**: Arrow keys (up/down) to move between player rows
- **Horizontal Scrolling**: Arrow keys (left/right) to move between attribute columns
- **Full Data Access**: All attributes and player data accessible via scrolling
- **No Filtering/Sorting**: Basic navigation only (filtering/sorting for future implementation)

### Interaction
- **View Only**: Table display with navigation only
- **No Additional Features**: No export, comparison, or detail views in initial implementation

## Data Structure Requirements

The browser needs to handle data in this exact column order from the spreadsheet:

### Column Structure (A2:EQ58 - 147 columns total)
1. **Player Metadata** (Columns A-C): Name, Age, Foot
2. **Technical Attributes** (Columns D-Q): Cor, Cro, Dri, Fin, Fir, Fre, Hea, Lon, L Th, Mar, Pas, Pen, Tck, Tec
3. **Mental Attributes** (Columns R-AE): Agg, Ant, Bra, Cmp, Cnt, Dec, Det, Fla, Ldr, OtB, Pos, Tea, Vis, Wor
4. **Physical Attributes** (Columns AF-AM): Acc, Agi, Bal, Jum, Nat, Pac, Sta, Str
5. **Goalkeeping Attributes** (Columns AN-AX): Aer, Cmd, Com, Ecc, Han, Kic, 1v1, Pun, Ref, Rus, Thr
6. **Additional Data** (Column AY): DNA
7. **Position Ratings** (Columns AZ-EQ): All 96 Football Manager role ratings in specific order

### Data Validation
- **Valid Players**: Only display rows with non-empty player names
- **Invalid Values**: Display "--" values as-is (represent missing/invalid data)
- **Data Types**: Player name and footedness as strings, all other values as floats with 2 decimal places

## Terminal UI Implementation

### Technical Framework
- **UI Library**: `ratatui` for built-in table widget, scrolling, and event handling
- **Dependencies**: `ratatui` + `crossterm` for cross-platform terminal control
- **Color Support**: Required for cell highlighting and visual feedback
- **Minimum Terminal Size**: TBD based on column width requirements (147 columns)

### User Interface Design
- **Table Widget**: Scrollable table with fixed headers showing attribute short names
- **Cell Selection**: Visual highlighting of current cell position using ratatui styling
- **Header Behavior**: Column headers remain visible during vertical scrolling

### Navigation Controls
- **Arrow Keys**: Up/Down for rows, Left/Right for columns
- **Exit**: 'q' key or 'Esc' key to quit application
- **Initial Position**: Cursor starts at first player, first column (Name)
- **Boundary Behavior**: Stop at edges (no wrap-around)

### Data Display
- **Loading State**: Progress indicator during Google Sheets data fetch
- **Error Handling**: Authentication errors abort with clear error message
- **Performance**: Load all data at once (no pagination needed)
- **Memory**: Full dataset loaded into memory (acceptable for 57×147 data size)

## Configuration Integration

### CLI Arguments
- **Minimal Configuration**: No additional CLI parameters beyond existing tools
- **Spreadsheet Configuration**: Uses existing spreadsheet ID and credentials setup
- **Sheet Name**: Configurable target sheet name (inherits from existing config system)

### Error Handling
- **Authentication Errors**: Program aborts with descriptive error message
- **Network Errors**: Program aborts with connection error details
- **Data Errors**: Continue execution, display invalid values as "--"

### Integration Pattern
- **JSON Config**: Extends existing config.json structure
- **Credentials**: Same Google service account authentication flow
- **Logging**: Inherits existing logging configuration and verbosity controls
