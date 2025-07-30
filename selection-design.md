# Team Selection Assistant - Design Document

## Overview

This document outlines the implementation steps for the team selection assistant tool (`fm_team_selector`), which reads player data from Google Sheets and finds optimal player-to-role assignments.

## Architecture

The tool will be implemented as a new Rust binary that reuses existing infrastructure from the `fm_google_up` tool, specifically:
- Configuration system (`config.rs`)
- Google Sheets authentication (`auth.rs`) 
- Google Sheets client (`sheets_client.rs`)
- Progress tracking (`progress.rs`)

## Implementation Steps

### Step 1: Define Data Structures ✅ **COMPLETED**
**Starting assumption**: Empty project with existing fm_google_up infrastructure
**Completion criteria**: All data structures compile and have appropriate Debug/Clone derives with comprehensive unit tests

1.1. ✅ Create `src/selection.rs` module
1.2. ✅ Define `Role` struct with validation for the predefined role list (96 roles)
1.3. ✅ Define `Player` struct with name, age, footedness, abilities (47 Vec<Option<f32>>), DNA score, and role ratings (96)
1.4. ✅ Define `Assignment` struct to represent a player-role pairing with score
1.5. ✅ Define `Team` struct to hold 11 assignments with total score calculation
1.6. ✅ **Write unit tests for data structures** (14 tests):
   - ✅ Test Role validation with valid/invalid role strings
   - ✅ Test Player creation with complete/incomplete data
   - ✅ Test Assignment score calculation
   - ✅ Test Team total score calculation and validation (exactly 11 assignments)
   - ✅ Test Footedness parsing and display
   - ✅ Test duplicate player/role validation
   - ✅ Test error handling for edge cases
1.7. ✅ **Run code quality checks**:
   - ✅ `cargo fmt`
   - ✅ `cargo clippy --fix`

### Step 2: Role File Parser ✅ **COMPLETED**
**Starting assumption**: Data structures from Step 1 exist
**Completion criteria**: Can read and validate role files, returning appropriate errors for invalid roles with comprehensive test coverage

2.1. ✅ Add role file path to configuration (extend existing config system)
2.2. ✅ Implement `parse_role_file()` function using `tokio::fs::read_to_string`
2.3. ✅ Validate each role against the predefined role list (tab-separated string from spec)
2.4. ✅ Return error if file doesn't contain exactly 11 valid roles (duplicate roles allowed)
2.5. ✅ **Write comprehensive unit tests for role file parser** (7 tests):
   - ✅ Test parsing valid 11-role file
   - ✅ Test error handling for non-existent files
   - ✅ Test error handling for files with wrong number of roles (0, 10, 12 roles)
   - ✅ Test error handling for files with invalid role names
   - ✅ Test handling of duplicate roles (duplicate roles are allowed)
   - ✅ Test handling of whitespace and empty lines in role files
2.6. ✅ **Run code quality checks**:
   - ✅ `cargo fmt`
   - ✅ `cargo clippy --fix`

### Step 3: Google Sheets Data Parser ✅ **COMPLETED**
**Starting assumption**: Role parser from Step 2 exists, existing sheets_client available
**Completion criteria**: Can download and parse player data from Google Sheets range A2:EQ58 with thorough test coverage

3.1. ✅ Extend sheets client to read from 'Squad' sheet range A2:EQ58
3.2. ✅ Implement `parse_player_data()` function to convert raw sheet data into Player structs
3.3. ✅ Skip rows where column A (player name) is empty
3.4. ✅ Parse abilities (columns D-AX) treating empty cells as 0.0
3.5. ✅ Parse role ratings (columns AZ-EQ) treating empty cells as 0.0
3.6. ✅ Map column positions to the correct ability/role indices
3.7. ✅ **Write comprehensive unit tests for player data parser** (8 tests):
   - ✅ Test parsing complete player data (all fields populated)
   - ✅ Test parsing player data with missing abilities (empty cells → None)
   - ✅ Test parsing player data with missing role ratings (empty cells → None)
   - ✅ Test skipping rows with empty player names
   - ✅ Test handling invalid footedness values
   - ✅ Test handling malformed numeric data in abilities/ratings
   - ✅ Test parsing with different sheet data sizes (fewer/more than expected columns)
   - ✅ Test column-to-index mapping correctness for all 47 abilities and 96 roles
3.8. ✅ **Run code quality checks**:
   - ✅ `cargo fmt`
   - ✅ `cargo clippy --fix`

### Step 4: Assignment Algorithm ✅ **COMPLETED**
**Starting assumption**: Player and role data structures exist with parsed data
**Completion criteria**: Greedy algorithm finds valid assignments maximizing total score with comprehensive test suite

4.1. ✅ Implement `calculate_assignment_score()` function for player-role pairs
4.2. ✅ Implement greedy assignment algorithm:
   - ✅ For each role, find the unassigned player with highest score for that role
   - ✅ Assign that player to the role
   - ✅ Remove player from available pool
   - ✅ Continue until all 11 roles are assigned
4.3. ✅ Handle edge cases (fewer than 11 players, ties in scores)
4.4. ✅ **Write comprehensive unit tests for assignment algorithm** (9 tests):
   - ✅ Test assignment with exactly 11 players (optimal case)
   - ✅ Test assignment with more than 11 players (selection required)
   - ✅ Test assignment with fewer than 11 players (error case)
   - ✅ Test handling of tied scores (deterministic behavior)
   - ✅ Test algorithm correctness with known optimal solutions (small datasets)
   - ✅ Test edge case where player has 0.0 rating for assigned role
   - ✅ Test algorithm performance with realistic dataset sizes (50+ players)
   - ✅ Test wrong number of roles error handling
   - ✅ Test calculate_assignment_score function
4.5. ✅ **Run code quality checks**:
   - ✅ `cargo fmt`
   - ✅ `cargo clippy --fix`

### Step 5: Output Formatting ✅ **COMPLETED**
**Starting assumption**: Assignment algorithm produces valid Team struct
**Completion criteria**: Clean stdout output in format "$ROLE -> $PLAYER_NAME" with tested formatting

5.1. ✅ Implement `format_team_output()` function
5.2. ✅ Sort assignments by role or by score (chose role sorting for consistent output)
5.3. ✅ Include total team score in output
5.4. ✅ **Write unit tests for output formatting** (7 tests):
   - ✅ Test correct format "$ROLE -> $PLAYER_NAME" for all assignments
   - ✅ Test output includes total team score
   - ✅ Test output ordering (consistent and predictable)
   - ✅ Test handling of long player names and role names  
   - ✅ Test output with edge cases (minimum/maximum scores)
   - ✅ Test duplicate roles handling
   - ✅ Test decimal precision formatting
5.5. ✅ **Run code quality checks**:
   - ✅ `cargo fmt`
   - ✅ `cargo clippy --fix`

### Step 6: Main Binary Implementation ✅ **COMPLETED**
**Starting assumption**: All previous steps complete
**Completion criteria**: New binary runs end-to-end with proper error handling

6.1. ✅ Create `src/bin/fm_team_selector.rs`
6.2. ✅ Set up command line argument parsing using clap (reuse existing patterns)
6.3. ✅ Integrate with existing configuration system for Google Sheets credentials
6.4. ✅ Add role file path as required command line argument
6.5. ✅ Implement main flow:
   - ✅ Load and validate configuration
   - ✅ Parse role file
   - ✅ Authenticate with Google Sheets
   - ✅ Download player data
   - ✅ Run assignment algorithm
   - ✅ Output results
6.6. ✅ Add comprehensive error handling with anyhow
6.7. ✅ Add progress tracking for long operations
6.8. ✅ **Run code quality checks**:
   - ✅ `cargo fmt`
   - ✅ `cargo clippy --fix`

### Step 7: Integration Testing
**Starting assumption**: Complete binary implementation
**Completion criteria**: Tool works with real Google Sheets data and role files with comprehensive test coverage

7.1. Create sample role file for testing
7.2. **Write integration tests for end-to-end functionality**:
   - Test complete workflow: role file → sheets data → assignment → output
   - Test with mock Google Sheets responses (avoid external dependencies in CI)
   - Test error conditions (invalid roles, missing data, authentication failures)
   - Test with edge cases (exactly 11 players, many players, duplicate scores)
   - Test CLI argument parsing and error messages
7.3. **Write manual test procedures** for real Google Sheets:
   - Document steps to test with actual Google Sheets data
   - Verify assignments make logical sense with known player data
   - Test performance with realistic dataset sizes
7.4. **Run code quality checks**:
   - `cargo fmt`
   - `cargo clippy --fix`

### Step 8: Documentation and Build Integration
**Starting assumption**: Working tool with tests
**Completion criteria**: Tool is documented, integrated into build system, and all tests pass

8.1. Add binary to Cargo.toml
8.2. Update CLAUDE.md with new binary usage examples
8.3. Add role file format documentation
8.4. Add example role file to repository
8.5. Update main README if needed (only if explicitly requested)
8.6. **Verify test coverage and quality**:
   - Ensure `cargo test` passes with all unit and integration tests
   - Run `cargo test -- --nocapture` to verify test output
   - Verify test coverage includes all major code paths
   - Ensure tests are deterministic and don't depend on external services
8.7. **Final code quality checks**:
   - Ensure `cargo clippy` passes with no warnings
   - Ensure `cargo fmt` passes (code is properly formatted)
   - Run `cargo build --release` to ensure release build works

## Key Technical Decisions

1. **Greedy Algorithm**: Use greedy approach for assignment optimization - simple and sufficient for the problem size
2. **Role Validation**: Hardcode the role list from specification for strict validation
3. **Duplicate Roles**: Allow duplicate roles in role files (e.g., multiple goalkeepers) - requirement updated
4. **Error Handling**: Use existing anyhow patterns for consistency
5. **Configuration**: Extend existing config system rather than creating new one
6. **Data Types**: Use f32 for ability scores, Option<f32> for potentially missing data
7. **Async**: Use tokio for file I/O to match existing codebase patterns

## Dependencies

Reuse existing dependencies where possible:
- `tokio` for async file operations  
- `anyhow` for error handling
- `serde` for configuration
- `clap` for CLI arguments
- Existing Google Sheets client dependencies

New dependencies (if needed):
- None anticipated - existing infrastructure should be sufficient

## Testing Strategy

The testing approach follows Rust best practices and ensures comprehensive coverage:

### Unit Tests (Required for each step)
- **Data structures**: Validation, score calculation, edge cases
- **Role parser**: File I/O, validation, error handling
- **Player parser**: Sheet data parsing, missing data handling, column mapping
- **Assignment algorithm**: Optimization correctness, edge cases, performance
- **Output formatting**: String formatting, ordering, edge cases

### Integration Tests  
- **End-to-end workflows**: Complete tool execution with mock data
- **Error scenarios**: Invalid inputs, missing files, authentication failures
- **CLI interface**: Argument parsing, help text, error messages

### Mock Testing Strategy
- Use mock Google Sheets responses to avoid external dependencies
- Create deterministic test data with known optimal solutions
- Test boundary conditions (empty data, maximum data sizes)

### Manual Testing Procedures
- Document steps for testing with real Google Sheets
- Performance testing with realistic dataset sizes (50+ players)
- Validation that assignments make logical football sense

### Test Quality Requirements
- All tests must be deterministic and repeatable
- No external service dependencies in automated tests
- Tests must cover both happy path and error conditions  
- Test coverage should include all public APIs and major code paths

## File Structure

```
src/
├── lib.rs (add selection module export)
├── selection.rs (new module)
├── bin/
│   ├── fm_google_up.rs (existing)
│   └── fm_team_selector.rs (new)
└── (existing modules)
```