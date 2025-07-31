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

### Step 4: Update Configuration and CLI

**Starting Assumption**: Current CLI in `src/bin/fm_team_selector.rs` accepts role file path

**Implementation Details**:
1. No CLI changes needed - role file path remains the same
2. Update help text to mention new sectioned format
3. Ensure error messages clearly indicate file format issues
4. Update example role files in documentation

**Testing Requirements**:
- Integration tests with new role file format
- Test CLI error handling for malformed files
- Verify help text accuracy

**Validation**:
- Run `cargo test` - all tests pass
- Manual CLI testing with various file formats
- Help text review

**Definition of Done**:
- CLI seamlessly handles new format
- Clear error messages guide users
- Help documentation updated

### Step 5: Extend Integration Tests

**Starting Assumption**: Current integration tests in `tests/integration_tests.rs` test basic team selection

**Implementation Details**:
1. Add integration test scenarios:
   - Role file with filters that allow assignments
   - Role file with filters that block some players
   - Mixed scenario with filtered and unfiltered players
   - Backward compatibility test with old format
2. Create sample role files for each scenario
3. Add mock player data matching filter requirements
4. Verify correct assignment outputs and warning messages

**Testing Requirements**:
- 5+ new integration tests covering all filter scenarios
- Performance test: filtered assignment completes in <1 second
- Error handling test: invalid filter format

**Validation**:
- Run `cargo test` - all tests pass including new integration tests
- Run `cargo test --test integration_tests` specifically
- Performance benchmarking

**Definition of Done**:
- Comprehensive integration test coverage
- All scenarios produce expected outputs
- Performance requirements met

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