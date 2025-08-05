## Team Selection Assistant

This is a specification for a new tool, the team selection assistant. This new
project builds on the player data uploaded with the fm_google_up tool and
shares its configuration.

This time we are going to consume data from the remote Google spreadsheet and
use it to generate a proposed assignment of players to positions in a team.

**Implementation Status**: ✅ **COMPLETED** - All 8 implementation steps completed with comprehensive testing and documentation.

## Inputs

The program will rely on two inputs: a list of roles to assign for and a table of players and their strengths.

### Role List

The role list will be read from a local file. The user will provide a set of 11 player roles to assign.

Roles are one of "W(s) R	W(s) L	W(a) R	W(a) L	IF(s)	IF(a)	AP(s)	AP(a)	WTM(s)	WTM(a)	TQ(a)	RD(A)	IW(s)	IW(a)	DW(d)	DW(s)	WM(d)	WM(s)	WM(a)	WP(s)	WP(a)	MEZ(s)	MEZ(a)	BWM(d)	BWM(s)	BBM	CAR	CM(d)	CM(s)	CM(a)	DLP(d)	DLP(s)	RPM	HB	DM(d)	DM(s)	A	SV(s)	SV(a)	RGA	CD(d)	CD(s)	CD(c)	NCB(d)	WCB(d)	WCB(s)	WCB(a)	BPD(d)	BPD(s)	BPD(c)	L(s)	L(a)	FB(d) R	FB(s) R	FB(a) R	FB(d) L	FB(s) L	FB(a) L	IFB(d) R	IFB(d) L	WB(d) R	WB(s) R	WB(a) R	WB(d) L	WB(s) L	WB(a) L	IWB(d) R	IWB(s) R	IWB(a) R	IWB(d) L	IWB(s) L	IWB(a) L	CWB(s) R	CWB(a) R	CWB(s) L	CWB(a) L	PF(d)	PF(s)	PF(a)	TM(s)	TM(a)	AF	P	DLF(s)	DLF(a)	CF(s)	CF(a)	F9	SS	EG	AM(s)	AM(a)	SK(d)	SK(s)	SK(a)	GK"

### Player Strenghts

Player strengths are read from the remote Google spreadsheet. We will be reading the sheet named 'Squad'. We will read the cells in range A2:EQ58. The resulting table is structured as follows:
- each line represents an individual _player_
- each column represents a data element about this player:
  - column A is the player name
  - column B is the player age
  - column C is the player's footedness (R, L, or RL)
  - columns D to AX represent scores for player ability in the following order: "Cor Cro	Dri	Fin	Fir	Fre	Hea	Lon	L Th	Mar	Pas	Pen	Tck	Tec	Agg	Ant	Bra	Cmp	Cnt	Dec	Det	Fla	Ldr	OtB	Pos	Tea	Vis	Wor	Acc	Agi	Bal	Jum	Nat	Pac	Sta	Str	Aer	Cmd	Com	Ecc	Han	Kic	1v1	Pun	Ref	Rus	Thr". Abilities _can_ be empty.
  - column AY is a "player DNA" rating, representing how well a player scores on specific important abilities
  - columns AZ to EQ represent a players' ability when playing a specific role in the following order of the roles list specified in the previous section
  - we ignore lines that do not have the name field set. these do not represent actual players.

## Program Flow

The program will first validate that the role list exists and contains 11 valid role requirements. Note that duplicate roles are allowed in the role list (e.g., you can have two "GK" roles if you want two goalkeepers).

The program will then download data from the Google spreadsheet.

The program will then assign players from the spreadsheet to roles from the role list. An assignment of 11 players to the 11 roles can be scored by adding up the players' abilities in their assigned roles. Our goal is to find the player-role assignment with the highest total score.

The following constraints exist:
- An individual player can only be assigned to a single role.
- Multiple roles of the same type are allowed (e.g., two "GK" positions).
