use crate::domain::RoleId;
use serde::{Deserialize, Serialize};

/// Column headers for the browser data in exact spreadsheet order (A-EQ, 145 columns total)
/// This matches the exact structure used in Google Sheets with A2:EQ58 range
pub const COLUMN_HEADERS: [&str; 145] = [
    // Player Metadata (Columns A-C) - 3 fields
    "Name", "Age", "Foot", // Technical Attributes (Columns D-Q) - 14 attributes
    "Cor", "Cro", "Dri", "Fin", "Fir", "Fre", "Hea", "Lon", "L Th", "Mar", "Pas", "Pen", "Tck",
    "Tec", // Mental Attributes (Columns R-AE) - 14 attributes
    "Agg", "Ant", "Bra", "Cmp", "Cnt", "Dec", "Det", "Fla", "Ldr", "OtB", "Pos", "Tea", "Vis",
    "Wor", // Physical Attributes (Columns AF-AM) - 8 attributes
    "Acc", "Agi", "Bal", "Jum", "Nat", "Pac", "Sta", "Str",
    // Goalkeeping Attributes (Columns AN-AX) - 11 attributes
    "Aer", "Cmd", "Com", "Ecc", "Han", "Kic", "1v1", "Pun", "Ref", "Rus", "Thr",
    // Additional Data (Column AY) - 1 field
    "DNA",
    // Position Ratings (Columns AZ-EQ) - 94 ratings matching RoleId::VALID_ROLES exactly
    "W(s) R", "W(s) L", "W(a) R", "W(a) L", "IF(s)", "IF(a)", "AP(s)", "AP(a)", "WTM(s)", "WTM(a)",
    "TQ(a)", "RD(A)", "IW(s)", "IW(a)", "DW(d)", "DW(s)", "WM(d)", "WM(s)", "WM(a)", "WP(s)",
    "WP(a)", "MEZ(s)", "MEZ(a)", "BWM(d)", "BWM(s)", "BBM", "CAR", "CM(d)", "CM(s)", "CM(a)",
    "DLP(d)", "DLP(s)", "RPM", "HB", "DM(d)", "DM(s)", "A", "SV(s)", "SV(a)", "RGA", "CD(d)",
    "CD(s)", "CD(c)", "NCB(d)", "WCB(d)", "WCB(s)", "WCB(a)", "BPD(d)", "BPD(s)", "BPD(c)", "L(s)",
    "L(a)", "FB(d) R", "FB(s) R", "FB(a) R", "FB(d) L", "FB(s) L", "FB(a) L", "IFB(d) R",
    "IFB(d) L", "WB(d) R", "WB(s) R", "WB(a) R", "WB(d) L", "WB(s) L", "WB(a) L", "IWB(d) R",
    "IWB(s) R", "IWB(a) R", "IWB(d) L", "IWB(s) L", "IWB(a) L", "CWB(s) R", "CWB(a) R", "CWB(s) L",
    "CWB(a) L", "PF(d)", "PF(s)", "PF(a)", "TM(s)", "TM(a)", "AF", "P", "DLF(s)", "DLF(a)",
    "CF(s)", "CF(a)", "F9", "SS", "EG", "SK(d)", "SK(s)", "SK(a)", "GK",
];

/// Represents a player in the browser with all 145 fields from the spreadsheet
/// Total: 3 metadata + 14 technical + 14 mental + 8 physical + 11 goalkeeping + 1 DNA + 94 role ratings = 145 fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrowserPlayer {
    // Player Metadata (Columns A-C) - 3 fields
    pub name: String,
    pub age: f64,
    pub foot: String,

    // Technical Attributes (Columns D-Q) - 14 attributes
    pub corners: f64,
    pub crossing: f64,
    pub dribbling: f64,
    pub finishing: f64,
    pub first_touch: f64,
    pub free_kick_taking: f64,
    pub heading: f64,
    pub long_shots: f64,
    pub long_throws: f64,
    pub marking: f64,
    pub passing: f64,
    pub penalty_taking: f64,
    pub tackling: f64,
    pub technique: f64,

    // Mental Attributes (Columns R-AE) - 14 attributes
    pub aggression: f64,
    pub anticipation: f64,
    pub bravery: f64,
    pub composure: f64,
    pub concentration: f64,
    pub decisions: f64,
    pub determination: f64,
    pub flair: f64,
    pub leadership: f64,
    pub off_the_ball: f64,
    pub positioning: f64,
    pub teamwork: f64,
    pub vision: f64,
    pub work_rate: f64,

    // Physical Attributes (Columns AF-AM) - 8 attributes
    pub acceleration: f64,
    pub agility: f64,
    pub balance: f64,
    pub jumping_reach: f64,
    pub natural_fitness: f64,
    pub pace: f64,
    pub stamina: f64,
    pub strength: f64,

    // Goalkeeping Attributes (Columns AN-AX) - 11 attributes
    pub aerial_reach: f64,
    pub command_of_area: f64,
    pub communication: f64,
    pub eccentricity: f64,
    pub handling: f64,
    pub kicking: f64,
    pub one_on_ones: f64,
    pub punching: f64,
    pub reflexes: f64,
    pub rushing_out: f64,
    pub throwing: f64,

    // Additional Data (Column AY) - 1 field
    pub dna: f64,

    // Position Ratings (Columns AZ-EQ) - 94 ratings matching RoleId::VALID_ROLES exactly
    pub w_s_r: f64,
    pub w_s_l: f64,
    pub w_a_r: f64,
    pub w_a_l: f64,
    pub if_s: f64,
    pub if_a: f64,
    pub ap_s: f64,
    pub ap_a: f64,
    pub wtm_s: f64,
    pub wtm_a: f64,
    pub tq_a: f64,
    pub rd_a: f64,
    pub iw_s: f64,
    pub iw_a: f64,
    pub dw_d: f64,
    pub dw_s: f64,
    pub wm_d: f64,
    pub wm_s: f64,
    pub wm_a: f64,
    pub wp_s: f64,
    pub wp_a: f64,
    pub mez_s: f64,
    pub mez_a: f64,
    pub bwm_d: f64,
    pub bwm_s: f64,
    pub bbm: f64,
    pub car: f64,
    pub cm_d: f64,
    pub cm_s: f64,
    pub cm_a: f64,
    pub dlp_d: f64,
    pub dlp_s: f64,
    pub rpm: f64,
    pub hb: f64,
    pub dm_d: f64,
    pub dm_s: f64,
    pub a: f64,
    pub sv_s: f64,
    pub sv_a: f64,
    pub rga: f64,
    pub cd_d: f64,
    pub cd_s: f64,
    pub cd_c: f64,
    pub ncb_d: f64,
    pub wcb_d: f64,
    pub wcb_s: f64,
    pub wcb_a: f64,
    pub bpd_d: f64,
    pub bpd_s: f64,
    pub bpd_c: f64,
    pub l_s: f64,
    pub l_a: f64,
    pub fb_d_r: f64,
    pub fb_s_r: f64,
    pub fb_a_r: f64,
    pub fb_d_l: f64,
    pub fb_s_l: f64,
    pub fb_a_l: f64,
    pub ifb_d_r: f64,
    pub ifb_d_l: f64,
    pub wb_d_r: f64,
    pub wb_s_r: f64,
    pub wb_a_r: f64,
    pub wb_d_l: f64,
    pub wb_s_l: f64,
    pub wb_a_l: f64,
    pub iwb_d_r: f64,
    pub iwb_s_r: f64,
    pub iwb_a_r: f64,
    pub iwb_d_l: f64,
    pub iwb_s_l: f64,
    pub iwb_a_l: f64,
    pub cwb_s_r: f64,
    pub cwb_a_r: f64,
    pub cwb_s_l: f64,
    pub cwb_a_l: f64,
    pub pf_d: f64,
    pub pf_s: f64,
    pub pf_a: f64,
    pub tm_s: f64,
    pub tm_a: f64,
    pub af: f64,
    pub p: f64,
    pub dlf_s: f64,
    pub dlf_a: f64,
    pub cf_s: f64,
    pub cf_a: f64,
    pub f9: f64,
    pub ss: f64,
    pub eg: f64,
    pub sk_d: f64,
    pub sk_s: f64,
    pub sk_a: f64,
    pub gk: f64,
}

impl BrowserPlayer {
    /// Creates a BrowserPlayer from a row of string data
    /// Returns None if the player name is empty or row has insufficient columns
    pub fn from_row(row: &[String]) -> Option<Self> {
        if row.len() < 145 {
            return None;
        }

        let name = row[0].trim().to_string();
        if name.is_empty() {
            return None;
        }

        // Helper function to parse a value, treating "--" as 0.0
        let parse_value = |s: &str| -> f64 {
            let trimmed = s.trim();
            if trimmed == "--" || trimmed.is_empty() {
                0.0
            } else {
                trimmed.parse().unwrap_or(0.0)
            }
        };

        Some(BrowserPlayer {
            // Player Metadata (columns 0-2)
            name,
            age: parse_value(&row[1]),
            foot: row[2].trim().to_string(),

            // Technical Attributes (columns 3-16)
            corners: parse_value(&row[3]),
            crossing: parse_value(&row[4]),
            dribbling: parse_value(&row[5]),
            finishing: parse_value(&row[6]),
            first_touch: parse_value(&row[7]),
            free_kick_taking: parse_value(&row[8]),
            heading: parse_value(&row[9]),
            long_shots: parse_value(&row[10]),
            long_throws: parse_value(&row[11]),
            marking: parse_value(&row[12]),
            passing: parse_value(&row[13]),
            penalty_taking: parse_value(&row[14]),
            tackling: parse_value(&row[15]),
            technique: parse_value(&row[16]),

            // Mental Attributes (columns 17-30)
            aggression: parse_value(&row[17]),
            anticipation: parse_value(&row[18]),
            bravery: parse_value(&row[19]),
            composure: parse_value(&row[20]),
            concentration: parse_value(&row[21]),
            decisions: parse_value(&row[22]),
            determination: parse_value(&row[23]),
            flair: parse_value(&row[24]),
            leadership: parse_value(&row[25]),
            off_the_ball: parse_value(&row[26]),
            positioning: parse_value(&row[27]),
            teamwork: parse_value(&row[28]),
            vision: parse_value(&row[29]),
            work_rate: parse_value(&row[30]),

            // Physical Attributes (columns 31-38)
            acceleration: parse_value(&row[31]),
            agility: parse_value(&row[32]),
            balance: parse_value(&row[33]),
            jumping_reach: parse_value(&row[34]),
            natural_fitness: parse_value(&row[35]),
            pace: parse_value(&row[36]),
            stamina: parse_value(&row[37]),
            strength: parse_value(&row[38]),

            // Goalkeeping Attributes (columns 39-49)
            aerial_reach: parse_value(&row[39]),
            command_of_area: parse_value(&row[40]),
            communication: parse_value(&row[41]),
            eccentricity: parse_value(&row[42]),
            handling: parse_value(&row[43]),
            kicking: parse_value(&row[44]),
            one_on_ones: parse_value(&row[45]),
            punching: parse_value(&row[46]),
            reflexes: parse_value(&row[47]),
            rushing_out: parse_value(&row[48]),
            throwing: parse_value(&row[49]),

            // Additional Data (column 50)
            dna: parse_value(&row[50]),

            // Position Ratings (columns 51-144) - 94 ratings matching RoleId::VALID_ROLES order
            w_s_r: parse_value(&row[51]),
            w_s_l: parse_value(&row[52]),
            w_a_r: parse_value(&row[53]),
            w_a_l: parse_value(&row[54]),
            if_s: parse_value(&row[55]),
            if_a: parse_value(&row[56]),
            ap_s: parse_value(&row[57]),
            ap_a: parse_value(&row[58]),
            wtm_s: parse_value(&row[59]),
            wtm_a: parse_value(&row[60]),
            tq_a: parse_value(&row[61]),
            rd_a: parse_value(&row[62]),
            iw_s: parse_value(&row[63]),
            iw_a: parse_value(&row[64]),
            dw_d: parse_value(&row[65]),
            dw_s: parse_value(&row[66]),
            wm_d: parse_value(&row[67]),
            wm_s: parse_value(&row[68]),
            wm_a: parse_value(&row[69]),
            wp_s: parse_value(&row[70]),
            wp_a: parse_value(&row[71]),
            mez_s: parse_value(&row[72]),
            mez_a: parse_value(&row[73]),
            bwm_d: parse_value(&row[74]),
            bwm_s: parse_value(&row[75]),
            bbm: parse_value(&row[76]),
            car: parse_value(&row[77]),
            cm_d: parse_value(&row[78]),
            cm_s: parse_value(&row[79]),
            cm_a: parse_value(&row[80]),
            dlp_d: parse_value(&row[81]),
            dlp_s: parse_value(&row[82]),
            rpm: parse_value(&row[83]),
            hb: parse_value(&row[84]),
            dm_d: parse_value(&row[85]),
            dm_s: parse_value(&row[86]),
            a: parse_value(&row[87]),
            sv_s: parse_value(&row[88]),
            sv_a: parse_value(&row[89]),
            rga: parse_value(&row[90]),
            cd_d: parse_value(&row[91]),
            cd_s: parse_value(&row[92]),
            cd_c: parse_value(&row[93]),
            ncb_d: parse_value(&row[94]),
            wcb_d: parse_value(&row[95]),
            wcb_s: parse_value(&row[96]),
            wcb_a: parse_value(&row[97]),
            bpd_d: parse_value(&row[98]),
            bpd_s: parse_value(&row[99]),
            bpd_c: parse_value(&row[100]),
            l_s: parse_value(&row[101]),
            l_a: parse_value(&row[102]),
            fb_d_r: parse_value(&row[103]),
            fb_s_r: parse_value(&row[104]),
            fb_a_r: parse_value(&row[105]),
            fb_d_l: parse_value(&row[106]),
            fb_s_l: parse_value(&row[107]),
            fb_a_l: parse_value(&row[108]),
            ifb_d_r: parse_value(&row[109]),
            ifb_d_l: parse_value(&row[110]),
            wb_d_r: parse_value(&row[111]),
            wb_s_r: parse_value(&row[112]),
            wb_a_r: parse_value(&row[113]),
            wb_d_l: parse_value(&row[114]),
            wb_s_l: parse_value(&row[115]),
            wb_a_l: parse_value(&row[116]),
            iwb_d_r: parse_value(&row[117]),
            iwb_s_r: parse_value(&row[118]),
            iwb_a_r: parse_value(&row[119]),
            iwb_d_l: parse_value(&row[120]),
            iwb_s_l: parse_value(&row[121]),
            iwb_a_l: parse_value(&row[122]),
            cwb_s_r: parse_value(&row[123]),
            cwb_a_r: parse_value(&row[124]),
            cwb_s_l: parse_value(&row[125]),
            cwb_a_l: parse_value(&row[126]),
            pf_d: parse_value(&row[127]),
            pf_s: parse_value(&row[128]),
            pf_a: parse_value(&row[129]),
            tm_s: parse_value(&row[130]),
            tm_a: parse_value(&row[131]),
            af: parse_value(&row[132]),
            p: parse_value(&row[133]),
            dlf_s: parse_value(&row[134]),
            dlf_a: parse_value(&row[135]),
            cf_s: parse_value(&row[136]),
            cf_a: parse_value(&row[137]),
            f9: parse_value(&row[138]),
            ss: parse_value(&row[139]),
            eg: parse_value(&row[140]),
            sk_d: parse_value(&row[141]),
            sk_s: parse_value(&row[142]),
            sk_a: parse_value(&row[143]),
            gk: parse_value(&row[144]),
        })
    }

    /// Checks if this player has valid data (non-empty name)
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
    }

    /// Gets a field value by column index (0-144)
    /// Returns formatted string representation appropriate for display
    pub fn get_field_by_index(&self, index: usize) -> String {
        match index {
            0 => self.name.clone(),
            1 => format!("{:.2}", self.age),
            2 => self.foot.clone(),
            3 => format!("{:.2}", self.corners),
            4 => format!("{:.2}", self.crossing),
            5 => format!("{:.2}", self.dribbling),
            6 => format!("{:.2}", self.finishing),
            7 => format!("{:.2}", self.first_touch),
            8 => format!("{:.2}", self.free_kick_taking),
            9 => format!("{:.2}", self.heading),
            10 => format!("{:.2}", self.long_shots),
            11 => format!("{:.2}", self.long_throws),
            12 => format!("{:.2}", self.marking),
            13 => format!("{:.2}", self.passing),
            14 => format!("{:.2}", self.penalty_taking),
            15 => format!("{:.2}", self.tackling),
            16 => format!("{:.2}", self.technique),
            17 => format!("{:.2}", self.aggression),
            18 => format!("{:.2}", self.anticipation),
            19 => format!("{:.2}", self.bravery),
            20 => format!("{:.2}", self.composure),
            21 => format!("{:.2}", self.concentration),
            22 => format!("{:.2}", self.decisions),
            23 => format!("{:.2}", self.determination),
            24 => format!("{:.2}", self.flair),
            25 => format!("{:.2}", self.leadership),
            26 => format!("{:.2}", self.off_the_ball),
            27 => format!("{:.2}", self.positioning),
            28 => format!("{:.2}", self.teamwork),
            29 => format!("{:.2}", self.vision),
            30 => format!("{:.2}", self.work_rate),
            31 => format!("{:.2}", self.acceleration),
            32 => format!("{:.2}", self.agility),
            33 => format!("{:.2}", self.balance),
            34 => format!("{:.2}", self.jumping_reach),
            35 => format!("{:.2}", self.natural_fitness),
            36 => format!("{:.2}", self.pace),
            37 => format!("{:.2}", self.stamina),
            38 => format!("{:.2}", self.strength),
            39 => format!("{:.2}", self.aerial_reach),
            40 => format!("{:.2}", self.command_of_area),
            41 => format!("{:.2}", self.communication),
            42 => format!("{:.2}", self.eccentricity),
            43 => format!("{:.2}", self.handling),
            44 => format!("{:.2}", self.kicking),
            45 => format!("{:.2}", self.one_on_ones),
            46 => format!("{:.2}", self.punching),
            47 => format!("{:.2}", self.reflexes),
            48 => format!("{:.2}", self.rushing_out),
            49 => format!("{:.2}", self.throwing),
            50 => format!("{:.2}", self.dna),
            // Position ratings (51-144)
            51 => format!("{:.2}", self.w_s_r),
            52 => format!("{:.2}", self.w_s_l),
            53 => format!("{:.2}", self.w_a_r),
            54 => format!("{:.2}", self.w_a_l),
            55 => format!("{:.2}", self.if_s),
            56 => format!("{:.2}", self.if_a),
            57 => format!("{:.2}", self.ap_s),
            58 => format!("{:.2}", self.ap_a),
            59 => format!("{:.2}", self.wtm_s),
            60 => format!("{:.2}", self.wtm_a),
            61 => format!("{:.2}", self.tq_a),
            62 => format!("{:.2}", self.rd_a),
            63 => format!("{:.2}", self.iw_s),
            64 => format!("{:.2}", self.iw_a),
            65 => format!("{:.2}", self.dw_d),
            66 => format!("{:.2}", self.dw_s),
            67 => format!("{:.2}", self.wm_d),
            68 => format!("{:.2}", self.wm_s),
            69 => format!("{:.2}", self.wm_a),
            70 => format!("{:.2}", self.wp_s),
            71 => format!("{:.2}", self.wp_a),
            72 => format!("{:.2}", self.mez_s),
            73 => format!("{:.2}", self.mez_a),
            74 => format!("{:.2}", self.bwm_d),
            75 => format!("{:.2}", self.bwm_s),
            76 => format!("{:.2}", self.bbm),
            77 => format!("{:.2}", self.car),
            78 => format!("{:.2}", self.cm_d),
            79 => format!("{:.2}", self.cm_s),
            80 => format!("{:.2}", self.cm_a),
            81 => format!("{:.2}", self.dlp_d),
            82 => format!("{:.2}", self.dlp_s),
            83 => format!("{:.2}", self.rpm),
            84 => format!("{:.2}", self.hb),
            85 => format!("{:.2}", self.dm_d),
            86 => format!("{:.2}", self.dm_s),
            87 => format!("{:.2}", self.a),
            88 => format!("{:.2}", self.sv_s),
            89 => format!("{:.2}", self.sv_a),
            90 => format!("{:.2}", self.rga),
            91 => format!("{:.2}", self.cd_d),
            92 => format!("{:.2}", self.cd_s),
            93 => format!("{:.2}", self.cd_c),
            94 => format!("{:.2}", self.ncb_d),
            95 => format!("{:.2}", self.wcb_d),
            96 => format!("{:.2}", self.wcb_s),
            97 => format!("{:.2}", self.wcb_a),
            98 => format!("{:.2}", self.bpd_d),
            99 => format!("{:.2}", self.bpd_s),
            100 => format!("{:.2}", self.bpd_c),
            101 => format!("{:.2}", self.l_s),
            102 => format!("{:.2}", self.l_a),
            103 => format!("{:.2}", self.fb_d_r),
            104 => format!("{:.2}", self.fb_s_r),
            105 => format!("{:.2}", self.fb_a_r),
            106 => format!("{:.2}", self.fb_d_l),
            107 => format!("{:.2}", self.fb_s_l),
            108 => format!("{:.2}", self.fb_a_l),
            109 => format!("{:.2}", self.ifb_d_r),
            110 => format!("{:.2}", self.ifb_d_l),
            111 => format!("{:.2}", self.wb_d_r),
            112 => format!("{:.2}", self.wb_s_r),
            113 => format!("{:.2}", self.wb_a_r),
            114 => format!("{:.2}", self.wb_d_l),
            115 => format!("{:.2}", self.wb_s_l),
            116 => format!("{:.2}", self.wb_a_l),
            117 => format!("{:.2}", self.iwb_d_r),
            118 => format!("{:.2}", self.iwb_s_r),
            119 => format!("{:.2}", self.iwb_a_r),
            120 => format!("{:.2}", self.iwb_d_l),
            121 => format!("{:.2}", self.iwb_s_l),
            122 => format!("{:.2}", self.iwb_a_l),
            123 => format!("{:.2}", self.cwb_s_r),
            124 => format!("{:.2}", self.cwb_a_r),
            125 => format!("{:.2}", self.cwb_s_l),
            126 => format!("{:.2}", self.cwb_a_l),
            127 => format!("{:.2}", self.pf_d),
            128 => format!("{:.2}", self.pf_s),
            129 => format!("{:.2}", self.pf_a),
            130 => format!("{:.2}", self.tm_s),
            131 => format!("{:.2}", self.tm_a),
            132 => format!("{:.2}", self.af),
            133 => format!("{:.2}", self.p),
            134 => format!("{:.2}", self.dlf_s),
            135 => format!("{:.2}", self.dlf_a),
            136 => format!("{:.2}", self.cf_s),
            137 => format!("{:.2}", self.cf_a),
            138 => format!("{:.2}", self.f9),
            139 => format!("{:.2}", self.ss),
            140 => format!("{:.2}", self.eg),
            141 => format!("{:.2}", self.sk_d),
            142 => format!("{:.2}", self.sk_s),
            143 => format!("{:.2}", self.sk_a),
            144 => format!("{:.2}", self.gk),
            _ => "0.00".to_string(), // Handle out of bounds gracefully
        }
    }

    /// Get the role rating for a specific role by name
    /// Returns None if the role is not found or invalid
    pub fn get_role_rating(&self, role_name: &str) -> Option<f64> {
        RoleId::VALID_ROLES
            .iter()
            .position(|&r| r == role_name)
            .and_then(|index| {
                // Position ratings start at column 51, so add 51 to the role index
                let column_index = 51 + index;
                if column_index <= 144 {
                    Some(match column_index {
                        51 => self.w_s_r,
                        52 => self.w_s_l,
                        53 => self.w_a_r,
                        54 => self.w_a_l,
                        55 => self.if_s,
                        56 => self.if_a,
                        57 => self.ap_s,
                        58 => self.ap_a,
                        59 => self.wtm_s,
                        60 => self.wtm_a,
                        61 => self.tq_a,
                        62 => self.rd_a,
                        63 => self.iw_s,
                        64 => self.iw_a,
                        65 => self.dw_d,
                        66 => self.dw_s,
                        67 => self.wm_d,
                        68 => self.wm_s,
                        69 => self.wm_a,
                        70 => self.wp_s,
                        71 => self.wp_a,
                        72 => self.mez_s,
                        73 => self.mez_a,
                        74 => self.bwm_d,
                        75 => self.bwm_s,
                        76 => self.bbm,
                        77 => self.car,
                        78 => self.cm_d,
                        79 => self.cm_s,
                        80 => self.cm_a,
                        81 => self.dlp_d,
                        82 => self.dlp_s,
                        83 => self.rpm,
                        84 => self.hb,
                        85 => self.dm_d,
                        86 => self.dm_s,
                        87 => self.a,
                        88 => self.sv_s,
                        89 => self.sv_a,
                        90 => self.rga,
                        91 => self.cd_d,
                        92 => self.cd_s,
                        93 => self.cd_c,
                        94 => self.ncb_d,
                        95 => self.wcb_d,
                        96 => self.wcb_s,
                        97 => self.wcb_a,
                        98 => self.bpd_d,
                        99 => self.bpd_s,
                        100 => self.bpd_c,
                        101 => self.l_s,
                        102 => self.l_a,
                        103 => self.fb_d_r,
                        104 => self.fb_s_r,
                        105 => self.fb_a_r,
                        106 => self.fb_d_l,
                        107 => self.fb_s_l,
                        108 => self.fb_a_l,
                        109 => self.ifb_d_r,
                        110 => self.ifb_d_l,
                        111 => self.wb_d_r,
                        112 => self.wb_s_r,
                        113 => self.wb_a_r,
                        114 => self.wb_d_l,
                        115 => self.wb_s_l,
                        116 => self.wb_a_l,
                        117 => self.iwb_d_r,
                        118 => self.iwb_s_r,
                        119 => self.iwb_a_r,
                        120 => self.iwb_d_l,
                        121 => self.iwb_s_l,
                        122 => self.iwb_a_l,
                        123 => self.cwb_s_r,
                        124 => self.cwb_a_r,
                        125 => self.cwb_s_l,
                        126 => self.cwb_a_l,
                        127 => self.pf_d,
                        128 => self.pf_s,
                        129 => self.pf_a,
                        130 => self.tm_s,
                        131 => self.tm_a,
                        132 => self.af,
                        133 => self.p,
                        134 => self.dlf_s,
                        135 => self.dlf_a,
                        136 => self.cf_s,
                        137 => self.cf_a,
                        138 => self.f9,
                        139 => self.ss,
                        140 => self.eg,
                        141 => self.sk_d,
                        142 => self.sk_s,
                        143 => self.sk_a,
                        144 => self.gk,
                        _ => 0.0,
                    })
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_test_row() -> Vec<String> {
        let mut row = vec![
            "Test Player".to_string(),
            "25.5".to_string(),
            "Right".to_string(),
        ];
        // Add 142 more values (145 total) - mix of numbers and "--"
        for i in 3..145 {
            if i % 5 == 0 {
                row.push("--".to_string());
            } else {
                row.push(format!("{}.5", i % 20));
            }
        }
        row
    }

    #[test]
    fn test_column_headers_count() {
        assert_eq!(COLUMN_HEADERS.len(), 145);
    }

    #[test]
    fn test_column_headers_match_valid_roles() {
        // Check that position rating headers match RoleId::VALID_ROLES
        let position_headers = &COLUMN_HEADERS[51..145]; // 94 position ratings
        assert_eq!(position_headers.len(), RoleId::VALID_ROLES.len());

        for (i, &role) in RoleId::VALID_ROLES.iter().enumerate() {
            assert_eq!(position_headers[i], role, "Mismatch at index {i}");
        }
    }

    #[test]
    fn test_from_row_valid_player() {
        let row = create_valid_test_row();
        let player = BrowserPlayer::from_row(&row).expect("Should parse valid player");

        assert_eq!(player.name, "Test Player");
        assert_eq!(player.age, 25.5);
        assert_eq!(player.foot, "Right");
        assert!(player.is_valid());
    }

    #[test]
    fn test_from_row_empty_name() {
        let mut row = create_valid_test_row();
        row[0] = "".to_string();

        let player = BrowserPlayer::from_row(&row);
        assert!(player.is_none());
    }

    #[test]
    fn test_from_row_whitespace_name() {
        let mut row = create_valid_test_row();
        row[0] = "   ".to_string();

        let player = BrowserPlayer::from_row(&row);
        assert!(player.is_none());
    }

    #[test]
    fn test_from_row_insufficient_columns() {
        let row = vec!["Test".to_string(), "25".to_string()]; // Only 2 columns
        let player = BrowserPlayer::from_row(&row);
        assert!(player.is_none());
    }

    #[test]
    fn test_dash_values_converted_to_zero() {
        let row = create_valid_test_row();
        let player = BrowserPlayer::from_row(&row).expect("Should parse valid player");

        // Index 15 should be "--" based on our test data generation (15 % 5 == 0)
        assert_eq!(player.get_field_by_index(15), "0.00");
    }

    #[test]
    fn test_get_field_by_index() {
        let row = create_valid_test_row();
        let player = BrowserPlayer::from_row(&row).expect("Should parse valid player");

        assert_eq!(player.get_field_by_index(0), "Test Player");
        assert_eq!(player.get_field_by_index(1), "25.50");
        assert_eq!(player.get_field_by_index(2), "Right");

        // Test out of bounds
        assert_eq!(player.get_field_by_index(200), "0.00");
    }

    #[test]
    fn test_get_role_rating() {
        let row = create_valid_test_row();
        let player = BrowserPlayer::from_row(&row).expect("Should parse valid player");

        // Test first role (W(s) R at index 0, column 51)
        let rating = player.get_role_rating("W(s) R");
        assert!(rating.is_some());

        // Test invalid role
        let rating = player.get_role_rating("INVALID_ROLE");
        assert!(rating.is_none());

        // Test last role (GK)
        let rating = player.get_role_rating("GK");
        assert!(rating.is_some());
    }

    #[test]
    fn test_invalid_numeric_values_default_to_zero() {
        let mut row = create_valid_test_row();
        row[1] = "invalid_age".to_string(); // Invalid age
        row[3] = "not_a_number".to_string(); // Invalid corners value

        let player = BrowserPlayer::from_row(&row).expect("Should parse with invalid numbers");
        assert_eq!(player.age, 0.0);
        assert_eq!(player.corners, 0.0);
    }

    #[test]
    fn test_is_valid_method() {
        let row = create_valid_test_row();
        let player = BrowserPlayer::from_row(&row).expect("Should parse valid player");
        assert!(player.is_valid());

        let mut invalid_row = create_valid_test_row();
        invalid_row[0] = "".to_string();
        let invalid_player = BrowserPlayer::from_row(&invalid_row);
        assert!(invalid_player.is_none());
    }

    #[test]
    fn test_role_consistency_with_domain() {
        // Verify we have exactly the same number of columns as expected
        assert_eq!(COLUMN_HEADERS.len(), 145);

        // Position ratings should start at column 51 and cover exactly 94 roles
        let position_headers = &COLUMN_HEADERS[51..145];
        assert_eq!(position_headers.len(), 94);
        assert_eq!(RoleId::VALID_ROLES.len(), 94);

        // Every role in domain should have a corresponding header
        for role in RoleId::VALID_ROLES {
            assert!(
                position_headers.contains(role),
                "Missing header for role: {role}"
            );
        }
    }
}
