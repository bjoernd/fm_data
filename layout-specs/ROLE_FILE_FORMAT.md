# Role File Format Documentation

This document describes the role file format required by the `fm_team_selector` tool.

## Overview

The role file specifies which 11 Football Manager roles you want to assign players to. The tool will find the optimal assignment of players to these roles based on their role ratings from the Google Sheets data.

The tool supports two file formats:
1. **Legacy Format**: Simple list of 11 roles (backward compatible)
2. **Sectioned Format**: Roles with optional player filters (NEW)

## File Format

- **File Type**: Plain text file (e.g., `.txt`)
- **Encoding**: UTF-8
- **Line Endings**: Any (Unix/Windows/Mac compatible)
- **Whitespace**: Leading/trailing whitespace is automatically trimmed
- **Comments**: Lines starting with `#` are treated as comments (sectioned format only)

### Legacy Format (Backward Compatible)

- **Content**: Exactly 11 lines, one role per line
- **No sections**: Simple list format
- **No player filters**: All players eligible for all roles

### Sectioned Format (NEW)

- **Sections**: `[roles]` and optional `[filters]` sections
- **Player Filters**: Restrict specific players to position categories
- **Backward Compatible**: Legacy files continue to work unchanged

## Role Names

Each role must be one of the 96 valid Football Manager roles. Here's the complete list:

### Attacking Roles
- **Strikers**: AF, P, DLF(s), DLF(a), CF(s), CF(a), F9, TM(s), TM(a), PF(d), PF(s), PF(a)
- **Attacking Midfielders**: SS, EG, AM(s), AM(a), AP(s), AP(a)
- **Wingers**: W(s) R, W(s) L, W(a) R, W(a) L, IF(s), IF(a), IW(s), IW(a), WTM(s), WTM(a), TQ(a), RD(A), DW(d), DW(s)

### Midfield Roles
- **Central Midfielders**: CM(d), CM(s), CM(a), DLP(d), DLP(s), RPM, BBM, CAR, MEZ(s), MEZ(a)
- **Defensive Midfielders**: DM(d), DM(s), HB, BWM(d), BWM(s), A
- **Wide Midfielders**: WM(d), WM(s), WM(a), WP(s), WP(a)

### Defensive Roles
- **Centre Backs**: CD(d), CD(s), CD(c), BPD(d), BPD(s), BPD(c), NCB(d), WCB(d), WCB(s), WCB(a), L(s), L(a)
- **Full Backs**: FB(d) R, FB(s) R, FB(a) R, FB(d) L, FB(s) L, FB(a) L
- **Wing Backs**: WB(d) R, WB(s) R, WB(a) R, WB(d) L, WB(s) L, WB(a) L
- **Inverted Backs**: IFB(d) R, IFB(d) L, IWB(d) R, IWB(s) R, IWB(a) R, IWB(d) L, IWB(s) L, IWB(a) L
- **Complete Wing Backs**: CWB(s) R, CWB(a) R, CWB(s) L, CWB(a) L

### Goalkeeper Roles
- **Goalkeepers**: GK, SK(d), SK(s), SK(a)

### Special Roles
- **Second Strikers**: SV(s), SV(a), RGA

## Player Filters (Sectioned Format Only)

Player filters allow you to restrict specific players to certain position categories. This gives you tactical control over player assignments while maintaining the optimal assignment algorithm.

### Player Categories

The filter system uses 9 positional categories that map to Football Manager roles:

#### 1. Goal (`goal`) - Goalkeeper Roles
- **GK** - Goalkeeper
- **SK(d)** - Sweeper Keeper (Defend)
- **SK(s)** - Sweeper Keeper (Support)  
- **SK(a)** - Sweeper Keeper (Attack)

#### 2. Central Defender (`cd`) - Centre-Back Roles
- **CD(d/s/c)** - Centre-Back (Defend/Support/Cover)
- **BPD(d/s/c)** - Ball Playing Defender (Defend/Support/Cover)
- **NCB(d)** - No-Nonsense Centre-Back (Defend)
- **WCB(d/s/a)** - Wide Centre-Back (Defend/Support/Attack)
- **L(s/a)** - Libero (Support/Attack)

#### 3. Wing Back (`wb`) - Wide Defender Roles
- **FB(d/s/a) R/L** - Full-Back Right/Left (Defend/Support/Attack)
- **WB(d/s/a) R/L** - Wing-Back Right/Left (Defend/Support/Attack)
- **IFB(d) R/L** - Inverted Full-Back (Defend)
- **IWB(d/s/a) R/L** - Inverted Wing-Back (Defend/Support/Attack)
- **CWB(s/a) R/L** - Complete Wing-Back (Support/Attack)

#### 4. Defensive Midfielder (`dm`) - Defensive Midfield Roles
- **DM(d/s)** - Defensive Midfielder (Defend/Support)
- **HB** - Half Back
- **BWM(d/s)** - Ball Winning Midfielder (Defend/Support)
- **A** - Anchor Man
- **CM(d)** - Central Midfielder (Defend)
- **DLP(d)** - Deep Lying Playmaker (Defend)
- **BBM** - Box to Box Midfielder
- **SV(s/a)** - Segundo Volante (Support/Attack)

#### 5. Central Midfielder (`cm`) - Central Midfield Roles
- **CM(d/s/a)** - Central Midfielder (Defend/Support/Attack)
- **DLP(d/s)** - Deep Lying Playmaker (Defend/Support)
- **RPM** - Roaming Playmaker
- **BBM** - Box to Box Midfielder
- **CAR** - Carrilero
- **MEZ(s/a)** - Mezzala (Support/Attack)

#### 6. Winger (`wing`) - Wide Attacking Roles
- **WM(d/s/a)** - Wide Midfielder (Defend/Support/Attack)
- **WP(s/a)** - Wide Playmaker (Support/Attack)
- **W(s/a) R/L** - Winger Right/Left (Support/Attack)
- **IF(s/a)** - Inside Forward (Support/Attack)
- **IW(s/a)** - Inverted Winger (Support/Attack)
- **WTM(s/a)** - Wide Target Man (Support/Attack)
- **TQ(a)** - Trequartista (Attack)
- **RD(A)** - Raumdeuter (Attack)
- **DW(d/s)** - Defensive Winger (Defend/Support)

#### 7. Attacking Midfielder (`am`) - Attacking Midfield Roles
- **SS** - Shadow Striker
- **EG** - Enganche
- **AM(s/a)** - Attacking Midfielder (Support/Attack)
- **AP(s/a)** - Advanced Playmaker (Support/Attack)
- **CM(a)** - Central Midfielder (Attack)
- **MEZ(a)** - Mezzala (Attack)
- **IW(a/s)** - Inverted Winger (Attack/Support)

#### 8. Playmaker (`pm`) - Creative Midfield Roles
- **DLP(d/s)** - Deep Lying Playmaker (Defend/Support)
- **AP(s/a)** - Advanced Playmaker (Support/Attack)
- **WP(s/a)** - Wide Playmaker (Support/Attack)
- **RGA** - Regista
- **RPM** - Roaming Playmaker

#### 9. Striker (`str`) - Forward Roles
- **AF** - Advanced Forward
- **P** - Poacher
- **DLF(s/a)** - Deep Lying Forward (Support/Attack)
- **CF(s/a)** - Complete Forward (Support/Attack)
- **F9** - False 9
- **TM(s/a)** - Target Man (Support/Attack)
- **PF(d/s/a)** - Pressing Forward (Defend/Support/Attack)
- **IF(a/s)** - Inside Forward (Attack/Support)

### Filter Format

In the `[filters]` section, use this format:
```
PlayerName: category1, category2, category3
```

- **Player Name**: Exact name as it appears in your Google Sheets data
- **Categories**: One or more category short names separated by commas
- **Case Insensitive**: Category names can be uppercase, lowercase, or mixed case
- **Multiple Categories**: Players can be eligible for multiple position types

### Filter Behavior

- **No Filter**: Players without filters can be assigned to any role
- **With Filter**: Players are only eligible for roles in their allowed categories
- **Multiple Categories**: Players can be assigned to roles from any of their allowed categories
- **Assignment Priority**: Algorithm still assigns players to their highest-rated eligible roles

## Example Role Files

### Legacy Format Examples

#### Standard 4-3-3 Formation (Legacy)
```
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)
```

#### 3-5-2 Formation with Wing Backs (Legacy)
```
GK
CD(d)
CD(s)
CD(c)
WB(a) R
WB(a) L
CM(d)
CM(s)
CM(a)
CF(s)
CF(a)
```

### Sectioned Format Examples

#### 4-3-3 Formation with Player Filters
```
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)

[filters]
# Restrict goalkeeper to only goal roles
Alisson: goal
# Van Dijk can play central defender or wing back roles
Van Dijk: cd, wb
# Robertson limited to wing back positions
Robertson: wb
# Salah can play wing or attacking midfielder
Salah: wing, am
# Henderson restricted to central and defensive midfield
Henderson: cm, dm
```

#### Complex Formation with Multiple Restrictions
```
[roles]
GK
GK
CD(d)
CD(s)
WB(a) R
WB(a) L
DM(d)
CM(s)
AM(a)
W(a) R
CF(a)

[filters]
# Multiple goalkeepers with goal restriction
Alisson: goal
Becker: goal
# Strict positional restrictions
Van Dijk: cd
Matip: cd
# Wing backs with flexibility
Alexander-Arnold: wb, dm, cm
Robertson: wb, wing
# Attacking players with multiple options
De Bruyne: cm, am, pm
Salah: wing, am, str
# Strikers with forward flexibility
Haaland: str
```

#### Sectioned Format Without Filters
```
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)

# No [filters] section - all players eligible for all roles
```

### Mixed Tactical Examples

#### Defensive 5-4-1 with Specialist Restrictions
```
[roles]
GK
CD(d)
CD(s)
CD(c)
FB(d) R
FB(d) L
CM(d)
CM(s)
WM(d) R
WM(d) L
CF(s)

[filters]
# Defensive specialists only
Goalkeeper1: goal
CentreBack1: cd
CentreBack2: cd
CentreBack3: cd
# Wing backs can also play midfield if needed
RightBack: wb, dm
LeftBack: wb, dm
# Midfield flexibility
MidfielderA: dm, cm
MidfielderB: cm, am
# Wide players restricted to wide roles
RightMid: wing, am
LeftMid: wing, am
# Striker with some flexibility
MainStriker: str, am
```

## Rules and Constraints

### Role Requirements (Both Formats)
1. **Exactly 11 Roles**: The `[roles]` section (or entire file for legacy) must contain exactly 11 valid roles
2. **Valid Role Names**: Each role must match one of the 96 predefined Football Manager roles exactly
3. **Case Sensitive**: Role names are case sensitive (use exact spelling and capitalization)
4. **Duplicate Roles Allowed**: You can use the same role multiple times (e.g., two goalkeepers)

### Format-Specific Rules

#### Legacy Format
- **No Comments**: Comments are not supported in legacy format
- **No Empty Lines**: Empty lines are ignored, but discouraged
- **Simple List**: Just 11 lines of roles, no sections

#### Sectioned Format
- **Required Sections**: `[roles]` section is mandatory
- **Optional Filters**: `[filters]` section is optional
- **Comments Supported**: Lines starting with `#` are treated as comments
- **Section Headers**: Must be exactly `[roles]` and `[filters]` (case-insensitive)
- **Empty Lines**: Ignored in all sections

### Player Filter Rules (Sectioned Format Only)
1. **Unique Player Names**: Each player can only appear once in the `[filters]` section
2. **Valid Categories**: Category names must be one of: `goal`, `cd`, `wb`, `dm`, `cm`, `wing`, `am`, `pm`, `str`
3. **Filter Format**: Must follow `PlayerName: category1, category2` format
4. **Case Insensitive Categories**: Category names can be uppercase, lowercase, or mixed case
5. **Multiple Categories**: Players can have multiple allowed categories separated by commas
6. **Exact Player Names**: Player names must match exactly as they appear in your Google Sheets data

## Common Errors

### Role-Related Errors (Both Formats)

#### Invalid Role Names
```
# ❌ WRONG - Invalid role name
goalkeeper

# ✅ CORRECT - Valid role name  
GK
```

#### Wrong Number of Roles
```
# ❌ WRONG - Only 10 roles (missing 1)
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L

# ✅ CORRECT - Exactly 11 roles
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L  
CF(s)
```

#### Case Sensitivity
```
# ❌ WRONG - Incorrect capitalization
gk
cd(d)

# ✅ CORRECT - Proper capitalization
GK
CD(d)
```

### Filter-Related Errors (Sectioned Format)

#### Invalid Filter Format
```
# ❌ WRONG - Missing colon
[filters]
Alisson goal

# ❌ WRONG - Missing player name
[filters]
: goal

# ✅ CORRECT - Proper format
[filters]
Alisson: goal
```

#### Invalid Category Names
```
# ❌ WRONG - Invalid category
[filters]
Salah: striker
Van Dijk: defender

# ✅ CORRECT - Valid categories
[filters]
Salah: str
Van Dijk: cd
```

#### Duplicate Player Filters
```
# ❌ WRONG - Same player appears twice
[filters]
Salah: wing
Salah: am

# ✅ CORRECT - Combine categories for same player
[filters]
Salah: wing, am
```

#### Missing Section Headers
```
# ❌ WRONG - Missing [roles] section header
GK
CD(d)
CD(s)
...

[filters]
Alisson: goal

# ✅ CORRECT - Proper section headers
[roles]
GK
CD(d)
CD(s)
...

[filters]
Alisson: goal
```

## Usage Examples

### Basic Usage
```bash
# Use legacy format role file
cargo run --bin fm_team_selector -- -r examples/formation_legacy.txt

# Use sectioned format with filters
cargo run --bin fm_team_selector -- -r examples/formation_with_filters.txt

# Complex filtering scenario
cargo run --bin fm_team_selector -- -r examples/formation_mixed_restrictions.txt

# Use with config file
cargo run --bin fm_team_selector -- -c config.json

# Verbose mode to see filter processing
cargo run --bin fm_team_selector -- -r formation_with_filters.txt -v
```

### Creating Role Files

#### Create Legacy Format File
```bash
echo -e "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)" > my_formation.txt
cargo run --bin fm_team_selector -- -r my_formation.txt
```

#### Create Sectioned Format File with Filters
```bash
cat > my_filtered_formation.txt << 'EOF'
[roles]
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)

[filters]
# Your player filters here
YourGoalkeeper: goal
YourCenterBack: cd
YourWinger: wing, am
EOF
cargo run --bin fm_team_selector -- -r my_filtered_formation.txt
```

## Configuration File Integration

You can specify the role file path in your `config.json`:

```json
{
  "google": {
    "spreadsheet_name": "YOUR_SPREADSHEET_ID",
    "creds_file": "credentials.json",
    "team_sheet": "Squad"
  },
  "input": {
    "role_file": "examples/formation_with_filters.txt"
  }
}
```

## Troubleshooting

### Role File Validation Errors

#### "Invalid role on line X" Error
- Check that the role name exactly matches one of the 96 valid roles
- Verify capitalization and spacing (e.g., "W(s) R" not "W(s)R" or "w(s) r")
- Make sure there are no extra characters or typos
- See the complete role list in this document for reference

#### "Role file must contain exactly 11 roles, found X" Error
- Ensure the `[roles]` section contains exactly 11 Football Manager roles
- Count the number of non-empty, non-comment lines
- Check for empty lines or comments within the roles section
- Add roles if you have fewer than 11, remove if you have more

#### "Failed to read role file" Error
- Check that the file path is correct and the file exists
- Verify the file is readable (check file permissions)
- Ensure the file is not locked by another application
- Try using an absolute path instead of a relative path

### Player Filter Validation Errors

#### "Invalid category 'CATEGORY' for player 'PLAYER_NAME' on line X" Error
- Use only valid category short names: `goal`, `cd`, `wb`, `dm`, `cm`, `wing`, `am`, `pm`, `str`
- Check for typos in category names
- Categories are case-insensitive but must match exactly (no extra characters)

#### "Duplicate player filter for 'PLAYER_NAME' on line X" Error
- Each player can only have one filter entry in the `[filters]` section
- Combine multiple categories for a player into a single line: `Player: cat1, cat2, cat3`
- Remove duplicate entries for the same player

#### "Invalid filter format on line X: expected 'PLAYER_NAME: CATEGORIES'" Error
- Ensure filter lines follow the format: `PlayerName: category1, category2`
- Use a colon (`:`) to separate player name from categories
- Use commas (`,`) to separate multiple categories
- Make sure player name is not empty

#### "Missing [roles] section in role file" Error  
- When using sectioned format, the `[roles]` section is required
- Ensure section headers are enclosed in square brackets: `[roles]`, `[filters]`
- Check that section names are spelled correctly

### Assignment Warnings

#### "Warning: Player 'PLAYER_NAME' could not be assigned due to filter restrictions"
- The player's allowed categories don't include any roles needed for the formation
- Consider expanding the player's allowed categories or adjusting the formation
- Check if the player's natural position matches their filter categories

#### "Warning: X players could not be assigned due to filter restrictions"
- Multiple players are filtered out of all available roles
- Review filter settings and formation requirements
- Some unfiltered players may be assigned instead

### Format Detection Issues

#### "No filters section found - using roles-only mode"
- This is informational when using sectioned format without filters
- The `[filters]` section is optional in sectioned format
- Add a `[filters]` section if you want to restrict specific players

### General Tips

1. **Backward Compatibility**: Old role files (11 lines without sections) continue to work unchanged
2. **Section Headers**: Use `[roles]` and `[filters]` exactly (case-insensitive)
3. **Comments**: Use `#` at the start of lines for comments in sectioned format
4. **Whitespace**: Leading/trailing spaces are automatically trimmed
5. **Case Sensitivity**: Role names are case-sensitive, but category names are case-insensitive
6. **Player Names**: Must match exactly as they appear in your Google Sheets data

## Performance Notes

- **Role file parsing**: Very fast (microseconds) for both formats
- **File size**: Minimal (typically <200 bytes for sectioned format with filters)
- **Validation**: All roles and filters are validated before starting assignment
- **Error detection**: Invalid roles and filters are caught early with clear error messages
- **Assignment performance**: Player filtering adds negligible overhead (<10% impact)
- **Large datasets**: Filtering works efficiently with 50+ players in <1 second
- **Memory usage**: Filter data structures are lightweight and memory-efficient

## Format Migration

### Upgrading from Legacy to Sectioned Format

To convert a legacy role file to sectioned format:

1. **Keep existing roles**: Copy your 11 roles to a `[roles]` section
2. **Add filters optionally**: Create a `[filters]` section if you want player restrictions
3. **Test compatibility**: Both formats continue to work

#### Example Migration
```bash
# Original legacy file (my_old_formation.txt)
GK
CD(d)
CD(s)
...

# New sectioned file (my_new_formation.txt)
[roles]
GK
CD(d)
CD(s)
...

[filters]  
# Add your player restrictions here
YourPlayer: goal
```

### Backward Compatibility Guarantee

- Legacy format files will continue to work indefinitely
- No changes required to existing role files
- Tool automatically detects and handles both formats
- Warning messages inform about format detection