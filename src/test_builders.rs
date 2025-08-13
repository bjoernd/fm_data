//! Test data builders for reducing test code duplication and improving test maintainability.
//!
//! This module provides fluent builder patterns for creating test data structures
//! that are commonly used across the test suite. This eliminates the repetitive
//! mock data creation patterns identified in the refactoring proposal.

use crate::Config;
use std::collections::HashMap;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;

/// Builder for creating Google Sheets player data for testing.
/// Reduces the 60% test code duplication identified in integration tests.
#[derive(Debug, Clone)]
pub struct PlayerDataBuilder {
    name: String,
    age: u8,
    footedness: String,
    abilities: Vec<String>,
    dna_score: f32,
    role_ratings: Vec<String>,
}

impl PlayerDataBuilder {
    /// Create a new player data builder with sensible defaults.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            age: 25,
            footedness: "R".to_string(),
            abilities: vec!["12.0".to_string(); 47], // Default abilities (47 total)
            dna_score: 80.0,
            role_ratings: vec!["10.0".to_string(); 96], // Default role ratings (96 total)
        }
    }

    /// Set the player's age.
    pub fn age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    /// Set the player's footedness (L, R, or RL).
    pub fn footedness(mut self, footedness: impl Into<String>) -> Self {
        self.footedness = footedness.into();
        self
    }

    /// Set a specific ability value by index (0-46).
    pub fn with_ability(mut self, index: usize, value: f32) -> Self {
        if index < self.abilities.len() {
            self.abilities[index] = value.to_string();
        }
        self
    }

    /// Set multiple ability values from a slice.
    pub fn with_abilities(mut self, abilities: &[f32]) -> Self {
        for (i, &value) in abilities.iter().enumerate() {
            if i < self.abilities.len() {
                self.abilities[i] = value.to_string();
            }
        }
        self
    }

    /// Set the DNA score.
    pub fn dna_score(mut self, score: f32) -> Self {
        self.dna_score = score;
        self
    }

    /// Set a specific role rating by index (0-95).
    pub fn with_role_rating(mut self, role_index: usize, rating: f32) -> Self {
        if role_index < self.role_ratings.len() {
            self.role_ratings[role_index] = rating.to_string();
        }
        self
    }

    /// Set multiple role ratings from a slice.
    pub fn with_role_ratings(mut self, ratings: &[f32]) -> Self {
        for (i, &rating) in ratings.iter().enumerate() {
            if i < self.role_ratings.len() {
                self.role_ratings[i] = rating.to_string();
            }
        }
        self
    }

    /// Make this player excellent at goalkeeper roles (role indices 95, 92-94).
    pub fn excellent_goalkeeper(mut self, rating: f32) -> Self {
        // GK is typically role index 95
        self.role_ratings[95] = rating.to_string();
        // Other goalkeeper variants
        for gk_role in [92, 93, 94] {
            self.role_ratings[gk_role] = rating.to_string();
        }
        self
    }

    /// Make this player excellent at centre-back roles.
    pub fn excellent_centre_back(mut self, rating: f32) -> Self {
        // CD roles are typically indices 40-42
        for cb_role in [40, 41, 42] {
            if cb_role < self.role_ratings.len() {
                self.role_ratings[cb_role] = rating.to_string();
            }
        }
        self
    }

    /// Make this player excellent at midfielder roles.
    pub fn excellent_midfielder(mut self, rating: f32) -> Self {
        // CM roles are typically indices 27-29
        for cm_role in [27, 28, 29] {
            if cm_role < self.role_ratings.len() {
                self.role_ratings[cm_role] = rating.to_string();
            }
        }
        self
    }

    /// Make this player excellent at striker roles.
    pub fn excellent_striker(mut self, rating: f32) -> Self {
        // CF/ST roles are typically indices 78-80
        for st_role in [78, 79, 80] {
            if st_role < self.role_ratings.len() {
                self.role_ratings[st_role] = rating.to_string();
            }
        }
        self
    }

    /// Build the player data as a Google Sheets row (Vec<String>).
    pub fn build(self) -> Vec<String> {
        let mut row = Vec::with_capacity(145); // Total expected columns

        // Column A: Player name
        row.push(self.name);

        // Column B: Age
        row.push(self.age.to_string());

        // Column C: Footedness
        row.push(self.footedness);

        // Columns D-AX: Abilities (47 abilities)
        row.extend(self.abilities);

        // Column AY: DNA score
        row.push(self.dna_score.to_string());

        // Columns AZ-EQ: Role ratings (96 roles)
        row.extend(self.role_ratings);

        row
    }
}

/// Builder for creating collections of player data for testing.
#[derive(Debug)]
pub struct PlayersDataBuilder {
    players: Vec<PlayerDataBuilder>,
}

impl PlayersDataBuilder {
    /// Create a new players data builder.
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    /// Add a player using a builder.
    pub fn add_player(mut self, player: PlayerDataBuilder) -> Self {
        self.players.push(player);
        self
    }

    /// Add a basic player with just a name.
    pub fn add_basic_player(mut self, name: impl Into<String>) -> Self {
        self.players.push(PlayerDataBuilder::new(name));
        self
    }

    /// Add a realistic squad with well-known player names.
    pub fn add_realistic_squad(mut self) -> Self {
        // Goalkeepers
        self.players.push(
            PlayerDataBuilder::new("Alisson")
                .age(30)
                .footedness("R")
                .excellent_goalkeeper(19.0)
                .dna_score(88.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Ederson")
                .age(29)
                .footedness("L")
                .excellent_goalkeeper(18.0)
                .dna_score(85.0),
        );

        // Defenders
        self.players.push(
            PlayerDataBuilder::new("Van Dijk")
                .age(32)
                .footedness("R")
                .excellent_centre_back(18.0)
                .excellent_midfielder(15.0) // Also good at midfield
                .dna_score(92.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Dias")
                .age(26)
                .footedness("R")
                .excellent_centre_back(17.0)
                .excellent_midfielder(14.0) // Also good at midfield
                .dna_score(89.0),
        );

        // Midfielders
        self.players.push(
            PlayerDataBuilder::new("De Bruyne")
                .age(32)
                .footedness("R")
                .excellent_midfielder(18.0)
                .excellent_striker(16.0) // Also good as attacking player
                .dna_score(95.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Rodri")
                .age(27)
                .footedness("R")
                .excellent_midfielder(17.0)
                .excellent_centre_back(15.0) // Also good at defense
                .dna_score(91.0),
        );

        // Forwards
        self.players.push(
            PlayerDataBuilder::new("Haaland")
                .age(23)
                .footedness("L")
                .excellent_striker(19.0)
                .dna_score(94.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Kane")
                .age(30)
                .footedness("R")
                .excellent_striker(18.0)
                .dna_score(92.0),
        );

        // Additional squad players to reach 20 total
        self.players.push(
            PlayerDataBuilder::new("Robertson")
                .age(29)
                .footedness("L")
                .dna_score(87.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Alexander-Arnold")
                .age(25)
                .footedness("R")
                .dna_score(88.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Cancelo")
                .age(29)
                .footedness("R")
                .dna_score(86.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Modric")
                .age(38)
                .footedness("R")
                .dna_score(90.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Bellingham")
                .age(20)
                .footedness("R")
                .dna_score(88.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Salah")
                .age(31)
                .footedness("L")
                .dna_score(93.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Mane")
                .age(31)
                .footedness("R")
                .dna_score(90.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Sterling")
                .age(29)
                .footedness("R")
                .dna_score(87.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Benzema")
                .age(36)
                .footedness("R")
                .dna_score(90.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Squad Player 1")
                .age(24)
                .footedness("R")
                .dna_score(75.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Squad Player 2")
                .age(26)
                .footedness("L")
                .dna_score(76.0),
        );
        self.players.push(
            PlayerDataBuilder::new("Squad Player 3")
                .age(22)
                .footedness("RL")
                .dna_score(74.0),
        );

        self
    }

    /// Generate players with a specific count and numbering.
    pub fn generate_players(mut self, count: usize) -> Self {
        for i in 0..count {
            let player = PlayerDataBuilder::new(format!("Player {i}"))
                .age(20 + (i % 15) as u8) // Ages 20-34
                .footedness(match i % 3 {
                    0 => "R",
                    1 => "L",
                    _ => "RL",
                })
                .dna_score(75.0 + (i as f32 * 2.0) % 20.0); // DNA 75.0-95.0

            self.players.push(player);
        }
        self
    }

    /// Build all players as Google Sheets data.
    pub fn build(self) -> Vec<Vec<String>> {
        self.players.into_iter().map(|p| p.build()).collect()
    }

    /// Build with specific player count (truncate or extend).
    pub fn build_with_count(self, count: usize) -> Vec<Vec<String>> {
        let mut data = self.build();
        data.truncate(count);

        // Extend if needed
        while data.len() < count {
            let i = data.len();
            let player = PlayerDataBuilder::new(format!("Generated Player {i}"))
                .age(20 + (i % 15) as u8)
                .build();
            data.push(player);
        }

        data
    }
}

impl Default for PlayersDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating test configuration files.
#[derive(Debug, Clone)]
pub struct ConfigDataBuilder {
    google_config: GoogleConfigData,
    input_config: InputConfigData,
}

#[derive(Debug, Clone)]
struct GoogleConfigData {
    spreadsheet_name: Option<String>,
    creds_file: Option<String>,
    team_sheet: String,
    scouting_sheet: String,
}

#[derive(Debug, Clone)]
struct InputConfigData {
    data_html: String,
    role_file: String,
    image_file: String,
}

impl ConfigDataBuilder {
    /// Create a new config builder with sensible defaults.
    pub fn new() -> Self {
        Self {
            google_config: GoogleConfigData {
                spreadsheet_name: None,
                creds_file: None,
                team_sheet: "Squad".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input_config: InputConfigData {
                data_html: "".to_string(),
                role_file: "".to_string(),
                image_file: "".to_string(),
            },
        }
    }

    /// Set the spreadsheet ID.
    pub fn spreadsheet_id(mut self, id: impl Into<String>) -> Self {
        self.google_config.spreadsheet_name = Some(id.into());
        self
    }

    /// Set the credentials file path.
    pub fn credentials_file(mut self, path: impl Into<String>) -> Self {
        self.google_config.creds_file = Some(path.into());
        self
    }

    /// Set the team sheet name.
    pub fn team_sheet(mut self, sheet: impl Into<String>) -> Self {
        self.google_config.team_sheet = sheet.into();
        self
    }

    /// Set the scouting sheet name.
    pub fn scouting_sheet(mut self, sheet: impl Into<String>) -> Self {
        self.google_config.scouting_sheet = sheet.into();
        self
    }

    /// Set the input HTML file path.
    pub fn data_html(mut self, path: impl Into<String>) -> Self {
        self.input_config.data_html = path.into();
        self
    }

    /// Set the role file path.
    pub fn role_file(mut self, path: impl Into<String>) -> Self {
        self.input_config.role_file = path.into();
        self
    }

    /// Set the image file path.
    pub fn image_file(mut self, path: impl Into<String>) -> Self {
        self.input_config.image_file = path.into();
        self
    }

    /// Build the configuration as JSON string.
    pub fn build_json(self) -> String {
        let mut json_parts = Vec::new();

        // Google section
        let mut google_parts = Vec::new();
        if let Some(spreadsheet) = self.google_config.spreadsheet_name {
            google_parts.push(format!(r#"        "spreadsheet_name": "{spreadsheet}""#));
        }
        if let Some(creds) = self.google_config.creds_file {
            google_parts.push(format!(r#"        "creds_file": "{creds}""#));
        }
        google_parts.push(format!(
            r#"        "team_sheet": "{}""#,
            self.google_config.team_sheet
        ));
        google_parts.push(format!(
            r#"        "scouting_sheet": "{}""#,
            self.google_config.scouting_sheet
        ));

        json_parts.push(format!(
            "    \"google\": {{\n{}\n    }}",
            google_parts.join(",\n")
        ));

        // Input section
        let mut input_parts = Vec::new();
        if !self.input_config.data_html.is_empty() {
            input_parts.push(format!(
                r#"        "data_html": "{}""#,
                self.input_config.data_html
            ));
        }
        if !self.input_config.role_file.is_empty() {
            input_parts.push(format!(
                r#"        "role_file": "{}""#,
                self.input_config.role_file
            ));
        }
        if !self.input_config.image_file.is_empty() {
            input_parts.push(format!(
                r#"        "image_file": "{}""#,
                self.input_config.image_file
            ));
        }

        if !input_parts.is_empty() {
            json_parts.push(format!(
                "    \"input\": {{\n{}\n    }}",
                input_parts.join(",\n")
            ));
        }

        format!("{{\n{}\n}}", json_parts.join(",\n"))
    }

    /// Build and write to a temporary file.
    pub async fn build_temp_file(self) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
        let temp_file = NamedTempFile::new()?;
        let json_content = self.build_json();
        tokio::fs::write(temp_file.path(), json_content).await?;
        Ok(temp_file)
    }

    /// Build as a Config object (for direct testing).
    pub fn build_config(self) -> Config {
        // Note: This creates a Config using the default constructor
        // and would need to be loaded from JSON in practice
        Config::create_default()
    }
}

impl Default for ConfigDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating test role files.
#[derive(Debug, Clone)]
pub struct RoleFileBuilder {
    roles: Vec<String>,
    filters: HashMap<String, String>,
    use_sectioned_format: bool,
}

impl RoleFileBuilder {
    /// Create a new role file builder.
    pub fn new() -> Self {
        Self {
            roles: Vec::new(),
            filters: HashMap::new(),
            use_sectioned_format: false,
        }
    }

    /// Create a standard 11-role formation.
    pub fn standard_formation() -> Self {
        Self::new()
            .add_role("GK")
            .add_role("CD(d)")
            .add_role("CD(s)")
            .add_role("FB(d) R")
            .add_role("FB(d) L")
            .add_role("CM(d)")
            .add_role("CM(s)")
            .add_role("CM(a)")
            .add_role("W(s) R")
            .add_role("W(s) L")
            .add_role("CF(s)")
    }

    /// Add a role to the formation.
    pub fn add_role(mut self, role: impl Into<String>) -> Self {
        self.roles.push(role.into());
        self
    }

    /// Add multiple roles.
    pub fn add_roles(mut self, roles: &[&str]) -> Self {
        for role in roles {
            self.roles.push(role.to_string());
        }
        self
    }

    /// Add a player filter.
    pub fn add_filter(mut self, player: impl Into<String>, categories: impl Into<String>) -> Self {
        self.filters.insert(player.into(), categories.into());
        self.use_sectioned_format = true;
        self
    }

    /// Add multiple filters.
    pub fn add_filters(mut self, filters: &[(&str, &str)]) -> Self {
        for (player, categories) in filters {
            self.filters
                .insert(player.to_string(), categories.to_string());
        }
        self.use_sectioned_format = true;
        self
    }

    /// Create a formation with realistic player filters.
    pub fn with_realistic_filters() -> Self {
        Self::standard_formation()
            .add_filter("Alisson", "goal")
            .add_filter("Van Dijk", "cd")
            .add_filter("Dias", "cd")
            .add_filter("Robertson", "wb")
            .add_filter("Alexander-Arnold", "wb")
            .add_filter("Rodri", "dm")
            .add_filter("De Bruyne", "cm")
            .add_filter("Bellingham", "cm")
            .add_filter("Salah", "wing")
            .add_filter("Mane", "wing")
            .add_filter("Haaland", "str")
    }

    /// Build the role file content.
    pub fn build_content(self) -> String {
        if self.use_sectioned_format {
            let mut content = String::new();

            // Roles section
            content.push_str("[roles]\n");
            for role in &self.roles {
                content.push_str(role);
                content.push('\n');
            }

            // Filters section (if any)
            if !self.filters.is_empty() {
                content.push_str("\n[filters]\n");
                for (player, categories) in &self.filters {
                    content.push_str(&format!("{player}: {categories}\n"));
                }
            }

            content
        } else {
            // Legacy format - just roles
            self.roles.join("\n")
        }
    }

    /// Build and write to a temporary file.
    pub async fn build_temp_file(self) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
        let temp_file = NamedTempFile::new()?;
        let content = self.build_content();

        let mut file = tokio::fs::File::create(temp_file.path()).await?;
        file.write_all(content.as_bytes()).await?;
        file.flush().await?;

        Ok(temp_file)
    }
}

impl Default for RoleFileBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating test image files and related structures.
#[derive(Debug, Clone)]
pub struct ImageDataBuilder {
    format: ImageFormat,
    size: Option<(u32, u32)>,
    has_valid_header: bool,
}

#[derive(Debug, Clone)]
enum ImageFormat {
    Png,
    Jpeg,
    InvalidFormat,
}

impl ImageDataBuilder {
    /// Create a new image data builder.
    pub fn new() -> Self {
        Self {
            format: ImageFormat::Png,
            size: None,
            has_valid_header: true,
        }
    }

    /// Create a valid PNG image.
    pub fn valid_png() -> Self {
        Self::new().format_png().with_valid_header()
    }

    /// Create an invalid format image.
    pub fn invalid_format() -> Self {
        Self::new().format_invalid()
    }

    /// Set the image format to PNG.
    pub fn format_png(mut self) -> Self {
        self.format = ImageFormat::Png;
        self
    }

    /// Set the image format to JPEG.
    pub fn format_jpeg(mut self) -> Self {
        self.format = ImageFormat::Jpeg;
        self
    }

    /// Set the image format to invalid.
    pub fn format_invalid(mut self) -> Self {
        self.format = ImageFormat::InvalidFormat;
        self
    }

    /// Set the image size.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Enable valid file header.
    pub fn with_valid_header(mut self) -> Self {
        self.has_valid_header = true;
        self
    }

    /// Disable valid file header (create corrupted file).
    pub fn with_invalid_header(mut self) -> Self {
        self.has_valid_header = false;
        self
    }

    /// Build and write to a temporary file.
    pub fn build_temp_file(self) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
        let suffix = match self.format {
            ImageFormat::Png => ".png",
            ImageFormat::Jpeg => ".jpg",
            ImageFormat::InvalidFormat => ".txt",
        };

        let temp_file = NamedTempFile::with_suffix(suffix)?;

        let data = if self.has_valid_header {
            match self.format {
                ImageFormat::Png => {
                    // PNG magic number
                    vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
                }
                ImageFormat::Jpeg => {
                    // JPEG magic number
                    vec![0xFF, 0xD8, 0xFF, 0xE0]
                }
                ImageFormat::InvalidFormat => b"This is not an image file".to_vec(),
            }
        } else {
            // Invalid header
            vec![0x00, 0x00, 0x00, 0x00]
        };

        std::fs::write(temp_file.path(), data)?;
        Ok(temp_file)
    }

    /// Get the expected file extension.
    pub fn extension(&self) -> &str {
        match self.format {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::InvalidFormat => "txt",
        }
    }
}

impl Default for ImageDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_data_builder_basic() {
        let player = PlayerDataBuilder::new("Test Player")
            .age(30)
            .footedness("L")
            .dna_score(85.0)
            .build();

        assert_eq!(player[0], "Test Player"); // Name
        assert_eq!(player[1], "30"); // Age
        assert_eq!(player[2], "L"); // Footedness

        // Should have correct total length (name + age + foot + 47 abilities + dna + 96 roles)
        assert_eq!(player.len(), 1 + 1 + 1 + 47 + 1 + 96);
    }

    #[test]
    fn test_player_data_builder_abilities() {
        let player = PlayerDataBuilder::new("Test Player")
            .with_ability(0, 15.0)
            .with_ability(1, 16.0)
            .build();

        assert_eq!(player[3], "15"); // First ability (column D)
        assert_eq!(player[4], "16"); // Second ability (column E)
    }

    #[test]
    fn test_player_data_builder_roles() {
        let player = PlayerDataBuilder::new("Goalkeeper")
            .excellent_goalkeeper(18.0)
            .build();

        // Check GK role rating (should be at index 3 + 47 + 1 + 95 = 146)
        let gk_index = 3 + 47 + 1 + 95; // Name + age + foot + abilities + dna + role_95
        assert_eq!(player[gk_index], "18");
    }

    #[test]
    fn test_players_data_builder() {
        let data = PlayersDataBuilder::new()
            .add_basic_player("Player 1")
            .add_basic_player("Player 2")
            .build();

        assert_eq!(data.len(), 2);
        assert_eq!(data[0][0], "Player 1");
        assert_eq!(data[1][0], "Player 2");
    }

    #[test]
    fn test_players_data_builder_generate() {
        let data = PlayersDataBuilder::new().generate_players(5).build();

        assert_eq!(data.len(), 5);
        assert_eq!(data[0][0], "Player 0");
        assert_eq!(data[4][0], "Player 4");
    }

    #[test]
    fn test_config_data_builder() {
        let json = ConfigDataBuilder::new()
            .spreadsheet_id("test-spreadsheet-123")
            .credentials_file("/path/to/creds.json")
            .team_sheet("MySquad")
            .build_json();

        assert!(json.contains("test-spreadsheet-123"));
        assert!(json.contains("/path/to/creds.json"));
        assert!(json.contains("MySquad"));
    }

    #[test]
    fn test_role_file_builder_legacy() {
        let content = RoleFileBuilder::standard_formation().build_content();

        let lines: Vec<&str> = content.trim().split('\n').collect();
        assert_eq!(lines.len(), 11);
        assert_eq!(lines[0], "GK");
        assert_eq!(lines[10], "CF(s)");
        assert!(!content.contains("[roles]")); // Legacy format
    }

    #[test]
    fn test_role_file_builder_sectioned() {
        let content = RoleFileBuilder::standard_formation()
            .add_filter("Test Player", "goal")
            .build_content();

        assert!(content.contains("[roles]"));
        assert!(content.contains("[filters]"));
        assert!(content.contains("Test Player: goal"));
    }

    #[tokio::test]
    async fn test_image_data_builder() -> Result<(), Box<dyn std::error::Error>> {
        let png_file = ImageDataBuilder::valid_png().build_temp_file()?;

        // Check file extension
        let path = png_file.path();
        assert!(path.extension().unwrap().to_str().unwrap() == "png");

        // Check file content starts with PNG magic number
        let content = std::fs::read(path)?;
        assert_eq!(&content[0..4], &[0x89, 0x50, 0x4E, 0x47]);

        Ok(())
    }

    #[test]
    fn test_image_data_builder_invalid() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_file = ImageDataBuilder::invalid_format().build_temp_file()?;

        let path = invalid_file.path();
        assert!(path.extension().unwrap().to_str().unwrap() == "txt");

        let content = std::fs::read(path)?;
        assert!(content.starts_with(b"This is not an image file"));

        Ok(())
    }
}
