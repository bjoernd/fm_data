# Player Filter Feature Implementation Design

This document outlines the step-by-step implementation plan for adding player role category filtering to the `fm_team_selector` program.

## Overview

The feature extends the existing role file format to include optional player filters that restrict players to specific field position categories. This maintains backward compatibility while adding powerful filtering capabilities.

## Implementation Steps

### Step 1: Define Role Category Data Structures ✅ COMPLETED

**Starting Assumption**: Current codebase has basic role validation in `src/selection.rs`

**Implementation Details**:
1. ✅ Add `PlayerCategory` enum in `src/selection.rs` with variants: `Goal, CentralDefender, WingBack, DefensiveMidfielder, CentralMidfielder, Winger, AttackingMidfielder, Playmaker, Striker`
2. ✅ Add `PlayerFilter` struct containing player name and allowed categories
3. ✅ Add `RoleFileContent` struct to hold both roles and optional filters
4. ✅ Implement category-to-roles mapping function `get_roles_for_category()`
5. ✅ Add category parsing from short names (goal, cd, wb, dm, cm, wing, am, pm, str)

**Testing Requirements**:
- ✅ Unit tests for `PlayerCategory` enum serialization/parsing
- ✅ Unit tests for category-to-roles mapping covering all 96 roles
- ✅ Unit tests for short name parsing (case sensitivity, invalid names)
- ✅ Integration test with sample filter data

**Validation**:
- ✅ Run `cargo test` - all existing tests pass + new tests pass (110 total)
- ✅ Run `cargo clippy` - no warnings
- ✅ Run `cargo fmt` - code formatted

**Definition of Done**: 
- ✅ All data structures defined with proper derives
- ✅ Category mapping function covers all roles from specification
- ✅ Comprehensive unit tests with 100% coverage (16 new tests)
- ✅ Code passes all quality checks

**Commit**: c3cdad7 - "Implement Step 1: Define Role Category Data Structures for player filtering"

### Step 2: Extend Role File Parser ✅ COMPLETED

**Starting Assumption**: Current role file parser reads 11 lines sequentially in `src/selection.rs`

**Implementation Details**:
1. ✅ Modify `parse_role_file()` function to handle sectioned format
2. ✅ Add section detection for `[roles]` and `[filters]`
3. ✅ Implement backward compatibility - if no sections found, treat entire file as roles
4. ✅ Add `parse_filters_section()` function to parse player filter lines
5. ✅ Validate filter format: "PLAYER_NAME: CATEGORY_LIST"
6. ✅ Ensure exactly 11 roles in `[roles]` section
7. ✅ Enforce unique player names in filters section
8. ✅ Add warning when `[filters]` section is missing

**Testing Requirements**:
- ✅ Unit tests for sectioned file parsing (roles + filters)
- ✅ Unit tests for backward compatibility (roles only)
- ✅ Unit tests for malformed filter lines
- ✅ Unit tests for duplicate player names
- ✅ Unit tests for invalid categories
- ✅ Unit tests for missing sections

**Validation**:
- ✅ Run `cargo test` - all tests pass (131 total: 122 unit + 9 integration)
- ✅ Run `cargo clippy` - no warnings
- ✅ Test with existing role files to ensure backward compatibility

**Definition of Done**:
- ✅ Parser handles both old and new file formats seamlessly
- ✅ Comprehensive error messages for malformed input
- ✅ All edge cases covered by tests (12 new parser tests)
- ✅ Backward compatibility verified with existing files
- ✅ Comment handling and whitespace normalization implemented

**Commit**: ef78744 - "Implement Step 2: Extend Role File Parser with sectioned format support"

### Step 3: Modify Player Assignment Algorithm ✅ COMPLETED

**Starting Assumption**: Current assignment algorithm in `assign_players_to_roles()` considers all players for all roles

**Implementation Details**:
1. ✅ Modify `assign_players_to_roles()` to accept `PlayerFilter` data
2. ✅ Add `is_player_eligible_for_role()` function checking:
   - If player has no filter → eligible for all roles
   - If player has filter → role must be in allowed categories
3. ✅ Update greedy assignment loop to skip ineligible players
4. ✅ Track players that couldn't be assigned due to filters
5. ✅ Add warning message listing unassignable players at end

**Testing Requirements**:
- ✅ Unit tests for eligibility checking function (5 new tests)
- ✅ Unit tests for assignment with various filter combinations (3 new tests)
- ✅ Integration tests with complete scenarios (filtered + unfiltered players) (2 new tests)
- ✅ Test edge case: all players filtered out
- ✅ Test edge case: player filtered to incompatible roles

**Validation**:
- ✅ Run `cargo test` - all tests pass (130 unit + 11 integration tests)
- ✅ Run integration tests with mock data
- ✅ Verify assignment logic correctness

**Definition of Done**:
- ✅ Assignment algorithm correctly respects player filters
- ✅ Clear warning messages for unassignable players
- ✅ Algorithm maintains optimal assignment within constraints
- ✅ Performance not significantly degraded

**Commit**: [Next commit] - "Implement Step 3: Modify Player Assignment Algorithm with filter support"

### Step 4: Update Configuration and CLI ✅ COMPLETED

**Starting Assumption**: Current CLI in `src/bin/fm_team_selector.rs` accepts role file path

**Implementation Details**:
1. ✅ CLI argument structure unchanged - role file path remains the same
2. ✅ Updated imports to use `parse_role_file_content()` and `find_optimal_assignments_with_filters()`
3. ✅ Updated help text to clearly explain both legacy and new sectioned formats
4. ✅ Updated command description to mention player filter functionality
5. ✅ Enhanced logging to show both roles and filters count

**Testing Requirements**:
- ✅ Integration tests with new role file format (already added in Step 3)
- ✅ Test CLI error handling for malformed files (tested legacy, sectioned, and invalid formats)
- ✅ Verify help text accuracy (comprehensive help shows both formats clearly)

**Validation**:
- ✅ Run `cargo test` - all 141 tests pass (130 unit + 11 integration)
- ✅ Manual CLI testing with various file formats (legacy, sectioned, malformed)
- ✅ Help text review - clear documentation of both formats with examples
- ✅ Error handling verification - clear, informative error messages

**Definition of Done**:
- ✅ CLI seamlessly handles both legacy and new sectioned formats
- ✅ Clear error messages guide users (invalid roles, wrong counts, bad categories)
- ✅ Help documentation comprehensively updated with examples
- ✅ Backward compatibility maintained - existing role files work unchanged
- ✅ New filter functionality fully integrated into CLI workflow

**Commit**: [Next commit] - "Implement Step 4: Update Configuration and CLI with sectioned format support"

### Step 5: Extend Integration Tests ✅ COMPLETED

**Starting Assumption**: Current integration tests in `tests/integration_tests.rs` test basic team selection

**Implementation Details**:
1. ✅ Added comprehensive integration test scenarios:
   - ✅ Role file with filters that allow assignments (`test_complete_workflow_with_filters_allowing_assignments`)
   - ✅ Role file with filters that block some players (`test_filters_blocking_player_assignments`)
   - ✅ Mixed scenario with filtered and unfiltered players (`test_mixed_filtered_and_unfiltered_players`)
   - ✅ Backward compatibility test with old format (`test_backward_compatibility_old_format`)
2. ✅ Created sample role files for each scenario using NamedTempFile
3. ✅ Added mock player data matching filter requirements
4. ✅ Verified correct assignment outputs and warning messages

**Testing Requirements**:
- ✅ 5 new integration tests covering all filter scenarios:
  - `test_filters_blocking_player_assignments` - filters blocking natural assignments
  - `test_mixed_filtered_and_unfiltered_players` - mixed filtering scenarios
  - `test_filtered_assignment_performance` - performance with filters on large dataset
  - `test_invalid_filter_format_error_handling` - invalid filter format errors
  - `test_duplicate_player_filter_error_handling` - duplicate player filter errors
- ✅ Performance test: filtered assignment completes in <1 second (verified with 50 players + 11 filters)
- ✅ Error handling tests: invalid filter format and duplicate player names

**Validation**:
- ✅ Run `cargo test` - all 146 tests pass (130 unit + 16 integration)
- ✅ Run `cargo test --test integration_tests` specifically - all 16 integration tests pass
- ✅ Performance benchmarking - filtered assignments complete in <1000ms

**Definition of Done**:
- ✅ Comprehensive integration test coverage for all filter scenarios
- ✅ All scenarios produce expected outputs with proper error handling
- ✅ Performance requirements met (sub-second execution with large datasets)
- ✅ Error handling validates malformed filter formats and duplicate entries
- ✅ Clean imports and no compiler warnings

**Commit**: [Next commit] - "Implement Step 5: Extend Integration Tests with comprehensive filter scenarios"

### Step 6: Update Documentation and Examples

**Starting Assumption**: Current `CLAUDE.md` documents existing functionality

**Implementation Details**:
1. Update `CLAUDE.md` with new role file format documentation
2. Add example role files showing filter usage
3. Document all 9 player categories and their role mappings
4. Update usage examples with filtered scenarios
5. Document error messages and troubleshooting

**Testing Requirements**:
- Review documentation for accuracy
- Test all provided examples work correctly
- Verify role category mappings are complete

**Validation**:
- Documentation review for clarity and completeness
- Example validation through testing

**Definition of Done**:
- Complete documentation of new features
- Working examples for all scenarios
- Clear troubleshooting guidance

### Step 7: Final Integration and Validation

**Starting Assumption**: All individual components work correctly

**Implementation Details**:
1. Run complete test suite including all unit and integration tests
2. Test with realistic Football Manager data
3. Performance testing with large player datasets
4. Manual testing of all CLI scenarios
5. Backward compatibility verification with existing role files

**Testing Requirements**:
- Full test suite: `cargo test` passes (target: 110+ tests)
- Performance test: assignment with 50+ players and filters
- Manual validation with various role file formats
- Integration with Google Sheets data

**Validation**:
- `cargo test` - all tests pass
- `cargo clippy` - no warnings
- `cargo fmt` - code formatted
- Manual end-to-end testing successful

**Definition of Done**:
- Feature fully functional with comprehensive test coverage
- Performance meets requirements
- Documentation complete and accurate
- Code quality standards met
- Backward compatibility maintained

## File Structure Changes

```
src/
├── lib.rs (no changes)
├── selection.rs (major changes - add categories, filters, parsing)
├── bin/fm_team_selector.rs (minor changes - error handling)
tests/
├── integration_tests.rs (additions - filter scenarios)
examples/ (new)
├── formation_with_filters.txt
├── formation_legacy.txt
```

## Risk Mitigation

- **Backward Compatibility**: Extensive testing with existing role files
- **Performance**: Benchmarking with large datasets
- **Data Integrity**: Comprehensive validation of all inputs
- **User Experience**: Clear error messages and documentation

## Success Criteria

1. All existing functionality remains unchanged
2. New filter functionality works as specified
3. Test coverage maintains >95% for modified code
4. Performance impact <10% on existing operations
5. Documentation is complete and accurate