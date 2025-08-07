use super::types::{PlayerCategory, PlayerFilter, Role, RoleFileContent};
use crate::error::{FMDataError, Result};
use crate::error_helpers::{role_file_format_error, ErrorContext};
use crate::validators::RoleValidator;
use std::collections::HashSet;
use tokio::fs;

/// Parse a role file containing 11 roles (one per line) - legacy format
pub async fn parse_role_file(file_path: &str) -> Result<Vec<Role>> {
    let content = parse_role_file_content(file_path).await?;
    Ok(content.roles)
}

/// Parse a role file with optional filters (new sectioned format)
pub async fn parse_role_file_content(file_path: &str) -> Result<RoleFileContent> {
    let content = fs::read_to_string(file_path)
        .await
        .with_file_context(file_path, "read")?;

    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            // Remove inline comments and trim whitespace
            let without_comment = if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            };
            without_comment.trim().to_string()
        })
        .filter(|line| !line.is_empty())
        .collect();

    if lines.is_empty() {
        return Err(role_file_format_error(
            0,
            "Role file is empty or contains no valid lines",
        ));
    }

    // Check if this is a sectioned file
    let has_sections = lines
        .iter()
        .any(|line| line.starts_with('[') && line.ends_with(']'));

    if has_sections {
        parse_sectioned_role_file(lines)
    } else {
        // Legacy format - treat entire file as roles
        let roles = parse_roles_section(lines)?;
        log::warn!("Role file does not contain [filters] section - using legacy format");
        Ok(RoleFileContent::new(roles, vec![]))
    }
}

/// Parse sectioned role file with [roles] and optional [filters] sections
fn parse_sectioned_role_file(lines: Vec<String>) -> Result<RoleFileContent> {
    let mut current_section = None;
    let mut roles_lines = Vec::new();
    let mut filters_lines = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        if line.starts_with('[') && line.ends_with(']') {
            // Section header
            match line.to_lowercase().as_str() {
                "[roles]" => current_section = Some("roles"),
                "[filters]" => current_section = Some("filters"),
                _ => {
                    return Err(FMDataError::selection(format!(
                        "Unknown section '{}' on line {}",
                        line,
                        line_num + 1
                    )));
                }
            }
        } else {
            // Section content
            match current_section {
                Some("roles") => roles_lines.push(line.clone()),
                Some("filters") => filters_lines.push(line.clone()),
                Some(_) => {
                    // This shouldn't happen since we validate sections above
                    return Err(FMDataError::selection(format!(
                        "Unknown section state on line {}: {}",
                        line_num + 1,
                        line
                    )));
                }
                None => {
                    return Err(FMDataError::selection(format!(
                        "Content found outside of section on line {}: {}",
                        line_num + 1,
                        line
                    )));
                }
            }
        }
    }

    if roles_lines.is_empty() {
        return Err(FMDataError::selection(
            "No [roles] section found in role file".to_string(),
        ));
    }

    let roles = parse_roles_section(roles_lines)?;

    if filters_lines.is_empty() {
        log::warn!("No [filters] section found in role file");
        Ok(RoleFileContent::new(roles, vec![]))
    } else {
        let filters = parse_filters_section(filters_lines)?;
        Ok(RoleFileContent::new(roles, filters))
    }
}

/// Parse roles section - expects exactly 11 valid roles
fn parse_roles_section(lines: Vec<String>) -> Result<Vec<Role>> {
    use crate::constants::team::REQUIRED_ROLE_COUNT;

    if lines.len() != REQUIRED_ROLE_COUNT {
        return Err(FMDataError::selection(format!(
            "Roles section must contain exactly {} roles, found {}",
            REQUIRED_ROLE_COUNT,
            lines.len()
        )));
    }

    let mut roles = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let role = Role::new(line)
            .map_err(|e| role_file_format_error(line_num + 1, &format!("Invalid role: {e}")))?;

        roles.push(role);
    }

    Ok(roles)
}

/// Parse filters section - expects "PLAYER_NAME: CATEGORY_LIST" format
fn parse_filters_section(lines: Vec<String>) -> Result<Vec<PlayerFilter>> {
    let mut filters = Vec::new();
    let mut seen_players = HashSet::new();

    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(FMDataError::selection(format!(
                "Invalid filter format on line {}: '{}'. Expected 'PLAYER_NAME: CATEGORY_LIST'",
                line_num + 1,
                line
            )));
        }

        let player_name = parts[0].trim().to_string();
        let categories_str = parts[1].trim();

        if player_name.is_empty() {
            return Err(FMDataError::selection(format!(
                "Empty player name on line {}",
                line_num + 1
            )));
        }

        if !seen_players.insert(player_name.clone()) {
            return Err(FMDataError::selection(format!(
                "Duplicate player filter for '{}' on line {}",
                player_name,
                line_num + 1
            )));
        }

        let categories = if categories_str.is_empty() {
            Vec::new()
        } else {
            categories_str
                .split(',')
                .map(|cat| cat.trim())
                .filter(|cat| !cat.is_empty())
                .map(|cat| {
                    PlayerCategory::from_short_name(cat).map_err(|e| {
                        FMDataError::selection(format!(
                            "Invalid category '{}' for player '{}' on line {}: {}",
                            cat,
                            player_name,
                            line_num + 1,
                            e
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?
        };

        if categories.is_empty() {
            return Err(FMDataError::selection(format!(
                "No valid categories specified for player '{}' on line {}",
                player_name,
                line_num + 1
            )));
        }

        filters.push(PlayerFilter::new(player_name, categories));
    }

    Ok(filters)
}

/// Validate that roles in a role file are valid
pub fn validate_roles(roles: &[String]) -> Result<()> {
    RoleValidator::validate_roles(roles)
}
