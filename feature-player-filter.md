## Feature Overview

We are going to extend the fm_team_selector program with the ability to
restrict players to specific areas of the field. We are going to define
categories of player roles. And we will add a facility for the user to restrict
specific players to one or more of these categories.

## Player Role Categories

Football players are often trained to play in specific areas of the pitch. We define
such areas below:

**Goalkeepers** (short: goal) can play the roles GK, SK(d), SK(s), SK(a)

**Central Defenders** (short: cd) can play CD(d), CD(s), CD(c), BPD(d), BPD(s), BPD(c), NCB(d), WCB(d), WCB(s), WCB(a), L(s), L(a)

**Wing Backs** (short: wb) can play FB(d) R, FB(s) R, FB(a) R, FB(d) L, FB(s) L, FB(a) L, WB(d) R, WB(s) R, WB(a) R, WB(d) L, WB(s) L, WB(a) L, IFB(d) R, IFB(d) L, IWB(d) R, IWB(s) R, IWB(a) R, IWB(d) L, IWB(s) L, IWB(a) L, CWB(s) R, CWB(a) R, CWB(s) L, CWB(a) L

**Defensive Midfielders** (short: dm) can play DM(d), DM(s), HB, BWM(d), BWM(s), A, CM(d), DLP(d), BBM, SV

**Central Midfielders** (short: cm) can play CM(d), CM(s), CM(a), DLP(d), DLP(s), RPM, BBM, CAR, MEZ(s), MEZ(a)

**Wingers** (short: wing) can play WM(d), WM(s), WM(a), WP(s), WP(a), W(s) R, W(s) L, W(a) R, W(a) L, IF(s), IF(a), IW(s), IW(a), WTM(s), WTM(a), TQ(a), RD(A), DW(d), DW(s)

**Attacking Midfielders** (short: am) can play SS, EG, AM(s), AM(a), AP(s), AP(a), CM(a), MEZ(a), IW(a), IW(s)

**Playmakers** (short: pm) can play DLP(d), DLP(s), AP(s), AP(a), WP(s), WP(a), RGA, RPM

**Strikers** (short: str) can play AF, P, DLF(s), DLF(a), CF(s), CF(a), F9, TM(s), TM(a), PF(d), PF(s), PF(a), IF(a), IF(s)

## Assigning players to role categories

We will expand the role file facility that already exists. The user will provide two sections: roles and filters.

The roles section will start with the string "[roles]" and a linebreak, followed by 11 lines containing role specifications. This will cover the already existing functionality.

The newly introduced filters section will start with the string "[filters]" and a linebreak. It will be followed by individual lines of the format: "PLAYER_NAME: CATEGORY_LIST".

PLAYER_NAME will match the existing name attribute in the player data structure.

CATEGORY_LIST will be a comma-separated list of categories as defined in the previous section. Users can use the short form of the role categories.

## Effect of role category filtering

When trying to assign players to roles from the [roles] section, the tool will only consider a player if there is either NO category filter for this player or if there is a category filter and the respective role is included in the CATEGORY_LIST.

## Implementation Requirements

1. **Player name matching**: Use exact string matching (case-sensitive) between PLAYER_NAME in filters and the name attribute in player data.

2. **Backward compatibility**: The `[filters]` section is optional. If missing, print a warning message but continue with normal operation.

3. **Category validation**: Validate that all categories in CATEGORY_LIST are valid (goal, cd, wb, dm, cm, wing, am, pm, str). Invalid categories should cause an error.

4. **Unique player filters**: Each player can appear only once in the filters section. Duplicate player entries should cause an error.

5. **Overlapping categories**: Role categories are allowed to overlap (e.g., CM(a) appears in both cm and am categories).

6. **Unassignable players**: If a player cannot be assigned to any role due to their category restrictions, ignore them for assignments and print a warning at the end listing all such players.
