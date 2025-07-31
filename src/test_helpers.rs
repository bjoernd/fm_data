#[cfg(test)]
use crate::selection::{Footedness, Player};
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use tempfile::NamedTempFile;

#[cfg(test)]
pub fn create_test_credentials_file() -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        r#"{{
            "type": "service_account",
            "project_id": "test-project",
            "private_key_id": "test-key-id",
            "private_key": "-----BEGIN PRIVATE KEY-----\ntest-key\n-----END PRIVATE KEY-----\n",
            "client_email": "test@test-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }}"#
    )
    .unwrap();
    temp_file
}

#[cfg(test)]
pub fn create_test_html_file(content: &str) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "{}", content).unwrap();
    temp_file
}

#[cfg(test)]
pub fn create_test_role_file(roles: &[&str]) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    for role in roles {
        writeln!(temp_file, "{}", role).unwrap();
    }
    temp_file
}

#[cfg(test)]
pub fn create_test_player(name: &str, player_index: usize) -> Player {
    // Create test abilities (47 values) - using Option<f32>
    let abilities = (0..47)
        .map(|i| Some(((i + player_index) % 20) as f32))
        .collect();

    // Create test role ratings (96 values) - using Option<f32>
    let role_ratings = (0..96)
        .map(|i| Some(((i + player_index) % 10) as f32))
        .collect();

    Player {
        name: name.to_string(),
        age: ((player_index % 15) + 18) as u8, // Age between 18-32
        footedness: match player_index % 3 {
            0 => Footedness::Right,
            1 => Footedness::Left,
            _ => Footedness::Both,
        },
        abilities,
        dna_score: Some((player_index % 100) as f32),
        role_ratings,
    }
}

#[cfg(test)]
pub fn create_test_config_file(content: &str) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "{}", content).unwrap();
    temp_file
}
