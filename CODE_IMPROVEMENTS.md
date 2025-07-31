# FM Data Code Improvement Suggestions

This document contains suggestions for further code size reduction and quality improvements, organized by priority and impact.

## High Impact Improvements

## Medium Impact Improvements

### 1. Test Helper Consolidation (Medium Priority)
**Impact:** 30-40 lines reduction  
**Effort:** Medium

Create `src/test_helpers.rs` module to consolidate repeated test patterns:

**Duplicated patterns found:**
- Temporary file creation (config.rs:252, auth.rs:277, table.rs:67)
- Player creation for tests (selection.rs:488, 575, 594)
- Test data generation patterns

**Example consolidation:**
```rust
// src/test_helpers.rs
pub fn create_test_credentials_file() -> NamedTempFile { /* ... */ }
pub fn create_test_player(name: &str, role_index: usize) -> Player { /* ... */ }
pub fn create_test_html_file(content: &str) -> NamedTempFile { /* ... */ }
```

### 2. Validation Module Consolidation (Medium Priority)
**Impact:** 15-20 lines reduction  
**Effort:** Low

Merge the three validator structs into a single `Validator`:

**Current structure:**
```rust
pub struct PathValidator;     // 4 methods
pub struct IdValidator;       // 2 methods  
pub struct DataValidator;     // 3 methods
```

**Proposed structure:**
```rust
pub struct Validator;
impl Validator {
    // Path validation methods
    pub fn validate_file_exists(path: &str, file_type: &str) -> Result<()> { /* ... */ }
    pub fn validate_file_extension(path: &str, expected_ext: &str) -> Result<()> { /* ... */ }
    
    // ID validation methods  
    pub fn validate_spreadsheet_id(id: &str) -> Result<()> { /* ... */ }
    pub fn validate_sheet_name(name: &str) -> Result<()> { /* ... */ }
    
    // Data validation methods
    pub fn validate_table_size(rows: usize, max_rows: usize) -> Result<()> { /* ... */ }
    pub fn validate_row_consistency(rows: &[Vec<String>]) -> Result<()> { /* ... */ }
    pub fn validate_non_empty_data(data: &[Vec<String>]) -> Result<()> { /* ... */ }
}
```

### 3. Path Resolution Deduplication (Medium Priority)  
**Impact:** 20 lines reduction  
**Effort:** Low

The `Config` struct has duplicated path resolution logic:

**Example duplication in config.rs:130-177:**
```rust
let resolved_spreadsheet = spreadsheet
    .or_else(|| Some(self.google.spreadsheet_name.clone()))
    .filter(|s| !s.is_empty())
    .unwrap_or(default_spreadsheet);
```

This pattern is repeated for `credfile` and `input` paths.

**Proposed helper:**
```rust
impl Config {
    fn resolve_with_fallback<T: Clone>(
        cli_value: Option<T>, 
        config_value: T, 
        default_value: T
    ) -> T where T: AsRef<str> {
        cli_value
            .or_else(|| Some(config_value))
            .filter(|s| !s.as_ref().is_empty())
            .unwrap_or(default_value)
    }
}
```

## Low Impact Improvements

### 4. Constants Consolidation (Low Priority)
**Impact:** 10-15 lines reduction, better maintainability  
**Effort:** Low

Create `src/constants.rs` module for magic numbers and repeated strings:

**Current scattered constants:**
- Sheet ranges: `"A2:AX58"`, `"A2:EQ58"` (sheets_client.rs:126, fm_team_selector.rs:240)
- Default sheet names: `"Squad"`, `"Stats_Team"` (config.rs:7-16)  
- Max data rows: `57` (table.rs:56)
- Column mappings and ability counts in selection.rs

**Proposed constants.rs:**
```rust
pub mod ranges {
    pub const UPLOAD_RANGE: &str = "A2:AX58";
    pub const DOWNLOAD_RANGE: &str = "A2:EQ58";
    pub const MAX_DATA_ROWS: usize = 57;
}

pub mod defaults {
    pub const TEAM_SHEET: &str = "Squad";
    pub const TEAM_PERF_SHEET: &str = "Stats_Team";
    pub const LEAGUE_PERF_SHEET: &str = "Stats_Division";
}

pub mod data_layout {
    pub const ABILITIES_START_COL: usize = 3;  // Column D
    pub const DNA_SCORE_COL: usize = 50;       // Column AY  
    pub const ROLE_RATINGS_START_COL: usize = 51; // Column AZ
}
```

### 5. Error Constructor Macro (Low Priority)
**Impact:** 20 lines reduction  
**Effort:** Low

The error module has repetitive constructor methods. Use a macro:

**Current repetitive pattern:**
```rust
pub fn config<T: Into<String>>(message: T) -> Self { /* identical */ }
pub fn auth<T: Into<String>>(message: T) -> Self { /* identical */ }
// ... 6 more identical patterns
```

**Proposed macro:**
```rust
macro_rules! error_constructors {
    ($($name:ident),*) => {
        $(
            pub fn $name<T: Into<String>>(message: T) -> Self {
                Self::$name { message: message.into() }
            }
        )*
    };
}

impl FMDataError {
    error_constructors!(config, auth, table, sheets_api, progress, selection);
}
```

## Dependencies and Architecture

### 6. No Major Dependency Optimizations Needed
The crate uses focused, well-chosen dependencies without significant overlap:
- `clap` for CLI parsing
- `tokio` for async runtime
- `serde`/`serde_json` for serialization
- `sheets` for Google Sheets API
- `yup-oauth2` for OAuth2 authentication
- `table-extract` for HTML table parsing
- `indicatif` for progress bars

All dependencies serve distinct purposes and consolidation is not recommended.

## Implementation Priority

### Phase 1 (Quick Wins)
1. ✅ **COMPLETED:** Remove `src/tryout/` directory (-97 lines)
2. Create constants module (-10-15 lines)
3. Error constructor macro (-20 lines)

**Total Phase 1 Impact:** ~30-35 lines reduction (remaining items)

### Phase 2 (Medium Effort)
1. Validation module consolidation (-15-20 lines)
2. Path resolution deduplication (-20 lines)
3. Test helper consolidation (-30-40 lines)

**Total Phase 2 Impact:** ~65-80 lines reduction

### Phase 3 (Larger Refactoring)
1. ✅ **COMPLETED:** Authentication consolidation (-50+ lines)

**Total Phase 3 Impact:** ~50+ lines reduction (completed)

## Total Potential Impact

**Conservative estimate:** 85-125 lines reduction (remaining items)  
**Optimistic estimate:** 115-165 lines reduction (remaining items)

**Already completed:**
- Binary consolidation: Eliminated ~56 lines of duplication
- Experimental code removal: -97 lines
- Authentication consolidation: -50+ lines

Combined with future improvements, the total impact could reduce the overall codebase by **12-18%** while significantly improving maintainability and code organization.

## Notes

- All suggestions maintain backward compatibility
- Test coverage should be maintained throughout
- Consider implementing in phases to minimize risk
- Some improvements (like constants) provide more maintainability benefit than size reduction
- The binary consolidation already implemented provides the foundation for future improvements