# Role File Format Documentation

This document describes the role file format required by the `fm_team_selector` tool.

## Overview

The role file specifies which 11 Football Manager roles you want to assign players to. The tool will find the optimal assignment of players to these roles based on their role ratings from the Google Sheets data.

## File Format

- **File Type**: Plain text file (e.g., `.txt`)
- **Content**: Exactly 11 lines, one role per line
- **Encoding**: UTF-8
- **Line Endings**: Any (Unix/Windows/Mac compatible)
- **Whitespace**: Leading/trailing whitespace is automatically trimmed

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

## Example Role Files

### Standard 4-3-3 Formation
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

### 3-5-2 Formation with Wing Backs
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

### Defensive 5-4-1 Formation
```
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
```

### Formation with Duplicate Roles
```
GK
GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
CF(s)
```

## Rules and Constraints

1. **Exactly 11 Roles**: The file must contain exactly 11 lines with valid roles
2. **Valid Role Names**: Each role must match one of the 96 predefined Football Manager roles exactly
3. **Case Sensitive**: Role names are case sensitive (use exact spelling and capitalization)
4. **Duplicate Roles Allowed**: You can use the same role multiple times (e.g., two goalkeepers)
5. **No Empty Lines**: Empty lines or lines with only whitespace are ignored
6. **No Comments**: The file format doesn't support comments

## Common Errors

### Invalid Role Names
```
# ❌ WRONG - Invalid role name
goalkeeper

# ✅ CORRECT - Valid role name  
GK
```

### Wrong Number of Roles
```
# ❌ WRONG - Only 10 roles (missing 1)
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
```

### Case Sensitivity
```
# ❌ WRONG - Incorrect capitalization
gk
cd(d)

# ✅ CORRECT - Proper capitalization
GK
CD(d)
```

## Usage Examples

```bash
# Use the example role file
cargo run --bin fm_team_selector -- -r test_roles.txt

# Create your own formation file
echo -e "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)" > my_formation.txt
cargo run --bin fm_team_selector -- -r my_formation.txt

# Use with config file
cargo run --bin fm_team_selector -- -c config.json
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
    "role_file": "my_formation.txt"
  }
}
```

## Troubleshooting

### "Invalid role on line X" Error
- Check that the role name exactly matches one of the 96 valid roles
- Verify capitalization and spacing (e.g., "W(s) R" not "W(s)R" or "w(s) r")
- Make sure there are no extra characters or typos

### "Role file must contain exactly 11 roles" Error
- Count the number of non-empty lines in your file
- Remove any empty lines or comments
- Add roles if you have fewer than 11, remove if you have more

### "Failed to read role file" Error
- Check that the file path is correct
- Verify the file exists and is readable
- Ensure the file is not locked by another application

## Performance Notes

- Role file parsing is very fast (microseconds)
- File size should be minimal (typically <100 bytes)
- The tool validates all roles before starting the assignment algorithm
- Invalid roles are caught early with clear error messages