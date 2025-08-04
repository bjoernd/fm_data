use super::types::PlayerCategory;

/// Get all roles that belong to a specific player category
pub fn get_roles_for_category(category: &PlayerCategory) -> Vec<&'static str> {
    match category {
        PlayerCategory::Goal => vec!["GK", "SK(d)", "SK(s)", "SK(a)"],
        PlayerCategory::CentralDefender => vec![
            "CD(d)", "CD(s)", "CD(c)", "BPD(d)", "BPD(s)", "BPD(c)", "NCB(d)", "WCB(d)", "WCB(s)",
            "WCB(a)", "L(s)", "L(a)",
        ],
        PlayerCategory::WingBack => vec![
            "FB(d) R", "FB(s) R", "FB(a) R", "FB(d) L", "FB(s) L", "FB(a) L", "WB(d) R", "WB(s) R",
            "WB(a) R", "WB(d) L", "WB(s) L", "WB(a) L", "IFB(d) R", "IFB(d) L", "IWB(d) R",
            "IWB(s) R", "IWB(a) R", "IWB(d) L", "IWB(s) L", "IWB(a) L", "CWB(s) R", "CWB(a) R",
            "CWB(s) L", "CWB(a) L",
        ],
        PlayerCategory::DefensiveMidfielder => vec![
            "DM(d)", "DM(s)", "HB", "BWM(d)", "BWM(s)", "A", "CM(d)", "DLP(d)", "BBM", "SV(s)",
            "SV(a)",
        ],
        PlayerCategory::CentralMidfielder => vec![
            "CM(d)", "CM(s)", "CM(a)", "DLP(d)", "DLP(s)", "RPM", "BBM", "CAR", "MEZ(s)", "MEZ(a)",
        ],
        PlayerCategory::Winger => vec![
            "WM(d)", "WM(s)", "WM(a)", "WP(s)", "WP(a)", "W(s) R", "W(s) L", "W(a) R", "W(a) L",
            "IF(s)", "IF(a)", "IW(s)", "IW(a)", "WTM(s)", "WTM(a)", "TQ(a)", "RD(A)", "DW(d)",
            "DW(s)",
        ],
        PlayerCategory::AttackingMidfielder => vec![
            "SS", "EG", "AM(s)", "AM(a)", "AP(s)", "AP(a)", "CM(a)", "MEZ(a)", "IW(a)", "IW(s)",
        ],
        PlayerCategory::Playmaker => vec![
            "DLP(d)", "DLP(s)", "AP(s)", "AP(a)", "WP(s)", "WP(a)", "RGA", "RPM",
        ],
        PlayerCategory::Striker => vec![
            "AF", "P", "DLF(s)", "DLF(a)", "CF(s)", "CF(a)", "F9", "TM(s)", "TM(a)", "PF(d)",
            "PF(s)", "PF(a)", "IF(a)", "IF(s)",
        ],
    }
}

/// Check if a role belongs to a specific category
pub fn role_belongs_to_category(role_name: &str, category: &PlayerCategory) -> bool {
    get_roles_for_category(category).contains(&role_name)
}

/// Get all valid player categories
pub fn get_valid_categories() -> Vec<&'static str> {
    vec!["goal", "cd", "wb", "dm", "cm", "wing", "am", "pm", "str"]
}

/// Check if a category name is valid
pub fn is_valid_category(category: &str) -> bool {
    get_valid_categories().contains(&category.to_lowercase().as_str())
}
