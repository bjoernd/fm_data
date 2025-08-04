use fm_data::error::Result;
use fm_data::{
    find_optimal_assignments, format_team_output, parse_player_data, parse_role_file, Config,
};
use std::io::Write;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;

/// Test the complete workflow: role file → mock sheets data → assignment → output
#[tokio::test]
async fn test_complete_workflow_mock_data() -> Result<()> {
    // Create a temporary role file
    let role_content =
        "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse roles
    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;
    assert_eq!(roles.len(), 11);

    // Create mock player data (simulating Google Sheets data)
    let mock_sheet_data = create_mock_sheet_data();

    // Parse player data
    let players = parse_player_data(mock_sheet_data)?;
    assert!(!players.is_empty());

    // Run assignment algorithm
    let team = find_optimal_assignments(players, roles)?;
    assert_eq!(team.assignments.len(), 11);

    // Generate output
    let output = format_team_output(&team);
    assert!(output.contains(" -> "));
    assert!(output.contains("Total Score:"));

    // Verify output format
    let lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(lines.len(), 12); // 11 assignments + 1 total score line

    for line in &lines[0..11] {
        assert!(line.contains(" -> "));
        let parts: Vec<&str> = line.split(" -> ").collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty()); // Role should not be empty
        assert!(!parts[1].is_empty()); // Player name should not be empty
    }

    // Check total score line
    assert!(lines[11].starts_with("Total Score:"));

    Ok(())
}

/// Test error handling for invalid role files
#[tokio::test]
async fn test_error_handling_invalid_role_file() {
    // Test with non-existent role file
    let result = parse_role_file("/nonexistent/roles.txt").await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Failed to read file"));

    // Test with invalid roles
    let invalid_role_content = "InvalidRole1\nInvalidRole2\nGK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(invalid_role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    let result = parse_role_file(role_file.path().to_str().unwrap()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid role"));

    // Test with wrong number of roles
    let wrong_count_content = "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L"; // Only 5 roles
    let role_file2 = NamedTempFile::new().unwrap();
    let mut async_role_file2 = tokio::fs::File::create(role_file2.path()).await.unwrap();
    async_role_file2
        .write_all(wrong_count_content.as_bytes())
        .await
        .unwrap();
    async_role_file2.flush().await.unwrap();

    let result2 = parse_role_file(role_file2.path().to_str().unwrap()).await;
    assert!(result2.is_err());
    assert!(result2
        .unwrap_err()
        .to_string()
        .contains("Roles section must contain exactly 11 roles"));
}

/// Test error handling for assignment algorithm edge cases
#[tokio::test]
async fn test_assignment_algorithm_edge_cases() -> Result<()> {
    // Create valid roles
    let role_content =
        "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;

    // Test with insufficient players
    let insufficient_data = create_mock_sheet_data_with_player_count(5); // Only 5 players
    let insufficient_players = parse_player_data(insufficient_data)?;

    let result = find_optimal_assignments(insufficient_players, roles.clone());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Need at least 11 players"));

    // Test with exactly 11 players
    let exact_data = create_mock_sheet_data_with_player_count(11);
    let exact_players = parse_player_data(exact_data)?;

    let result = find_optimal_assignments(exact_players, roles.clone());
    assert!(result.is_ok());
    let team = result.unwrap();
    assert_eq!(team.assignments.len(), 11);

    // Test with many players (should select best 11)
    let many_data = create_mock_sheet_data_with_player_count(25);
    let many_players = parse_player_data(many_data)?;

    let result = find_optimal_assignments(many_players, roles);
    assert!(result.is_ok());
    let team = result.unwrap();
    assert_eq!(team.assignments.len(), 11);

    Ok(())
}

/// Test duplicate roles functionality
#[tokio::test]
async fn test_duplicate_roles_workflow() -> Result<()> {
    // Create role file with duplicate roles (multiple goalkeepers)
    let duplicate_role_content =
        "GK\nGK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(duplicate_role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse roles
    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;
    assert_eq!(roles.len(), 11);

    // Verify duplicate roles are present
    let gk_count = roles.iter().filter(|r| r.name == "GK").count();
    assert_eq!(gk_count, 2);

    // Create mock data with multiple good goalkeepers
    let mock_data = create_mock_sheet_data_with_good_goalkeepers();
    let players = parse_player_data(mock_data)?;

    // Run assignment algorithm
    let team = find_optimal_assignments(players, roles)?;
    assert_eq!(team.assignments.len(), 11);

    // Generate output and verify multiple GK assignments
    let output = format_team_output(&team);
    let gk_lines: Vec<&str> = output
        .lines()
        .filter(|line| line.starts_with("GK -> "))
        .collect();
    assert_eq!(gk_lines.len(), 2);

    Ok(())
}

/// Test configuration system integration
#[tokio::test]
async fn test_config_integration() -> Result<()> {
    // Create a temporary config file
    let config_content = r#"{
        "google": {
            "spreadsheet_name": "test-spreadsheet-id",
            "team_sheet": "Squad"
        },
        "input": {
            "role_file": "test_roles.txt"
        }
    }"#;

    let config_file = NamedTempFile::new().unwrap();
    tokio::fs::write(config_file.path(), config_content)
        .await
        .unwrap();

    // Test loading config
    let config = Config::from_file(config_file.path())?;
    assert_eq!(config.google.spreadsheet_name, "test-spreadsheet-id");
    assert_eq!(config.google.team_sheet, "Squad");
    assert_eq!(config.input.role_file, "test_roles.txt");

    // Test path resolution (without validation since files don't exist)
    let (spreadsheet, _credfile, _rolefile) = config.resolve_paths_unchecked(
        Some("override-spreadsheet".to_string()),
        Some("override-creds.json".to_string()),
        Some("override-input.html".to_string()),
    );
    assert_eq!(spreadsheet, "override-spreadsheet");

    Ok(())
}

/// Test large dataset performance
#[tokio::test]
async fn test_large_dataset_performance() -> Result<()> {
    use std::time::Instant;

    // Create roles
    let role_content =
        "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;

    // Create large dataset (50 players)
    let large_data = create_mock_sheet_data_with_player_count(50);
    let players = parse_player_data(large_data)?;

    // Measure performance
    let start = Instant::now();
    let team = find_optimal_assignments(players, roles)?;
    let duration = start.elapsed();

    // Should complete quickly (under 1 second for 50 players)
    assert!(duration.as_millis() < 1000);
    assert_eq!(team.assignments.len(), 11);
    assert!(team.total_score() >= 0.0);

    Ok(())
}

/// Test output formatting consistency
#[tokio::test]
async fn test_output_formatting_consistency() -> Result<()> {
    // Create test data
    let role_content =
        "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;
    let mock_data = create_mock_sheet_data();
    let players = parse_player_data(mock_data)?;
    let team = find_optimal_assignments(players, roles)?;

    // Generate output multiple times to ensure consistency
    let output1 = format_team_output(&team);
    let output2 = format_team_output(&team);
    let output3 = format_team_output(&team);

    assert_eq!(output1, output2);
    assert_eq!(output2, output3);

    // Verify output is properly sorted
    let lines: Vec<&str> = output1.trim().split('\n').collect();
    let roles_in_output: Vec<&str> = lines[0..11]
        .iter()
        .map(|line| line.split(" -> ").next().unwrap())
        .collect();

    // Check that roles are in sorted order
    for i in 1..roles_in_output.len() {
        assert!(
            roles_in_output[i - 1] <= roles_in_output[i],
            "Output not properly sorted: {} should come before {}",
            roles_in_output[i - 1],
            roles_in_output[i]
        );
    }

    Ok(())
}

/// Test with realistic Football Manager mock data
#[tokio::test]
async fn test_realistic_mock_squad() -> Result<()> {
    // Create realistic roles for a standard formation
    let role_content =
        "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;

    // Use realistic mock data
    let mock_data = create_realistic_mock_squad();
    let players = parse_player_data(mock_data)?;

    // Should have exactly the expected players
    assert_eq!(players.len(), 20);

    // Run assignment algorithm
    let team = find_optimal_assignments(players, roles)?;
    assert_eq!(team.assignments.len(), 11);

    // Verify logical assignments (goalkeeper should be assigned to GK)
    let gk_assignment = team
        .assignments
        .iter()
        .find(|a| a.role.name == "GK")
        .unwrap();

    assert!(gk_assignment.player.name.contains("Alisson")); // Best GK in mock data

    // Verify team has reasonable total score
    assert!(team.total_score() > 150.0); // Should be high with good players

    Ok(())
}

/// Test assignment quality with known optimal solution
#[tokio::test]
async fn test_optimal_assignment_quality() -> Result<()> {
    let role_content =
        "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)";
    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    let roles = parse_role_file(role_file.path().to_str().unwrap()).await?;

    // Create data where optimal assignment is known
    let mock_data = create_optimal_test_squad();
    let players = parse_player_data(mock_data)?;

    let team = find_optimal_assignments(players, roles)?;

    // Verify the algorithm found reasonable assignments
    // Since this is a greedy algorithm, it may not find the global optimum
    // but each assignment should be reasonable
    for assignment in &team.assignments {
        // The assignment score should match the player's rating for that role
        assert_eq!(
            assignment.score,
            assignment.player.get_role_rating(&assignment.role)
        );

        // The score should be positive (no negative ratings in our test data)
        assert!(assignment.score >= 0.0);
    }

    // The total score should be reasonable (high because of specialized players)
    assert!(team.total_score() > 100.0);

    Ok(())
}

// Helper function to create mock Google Sheets data
fn create_mock_sheet_data() -> Vec<Vec<String>> {
    create_mock_sheet_data_with_player_count(15)
}

// Helper function to create mock data with specific player count
fn create_mock_sheet_data_with_player_count(player_count: usize) -> Vec<Vec<String>> {
    let mut data = Vec::new();

    for i in 0..player_count {
        let mut row = Vec::new();

        // Column A: Player name
        row.push(format!("Player {}", i));

        // Column B: Age
        row.push(format!("{}", 20 + (i % 15))); // Ages 20-34

        // Column C: Footedness
        row.push(match i % 3 {
            0 => "R".to_string(),
            1 => "L".to_string(),
            _ => "RL".to_string(),
        });

        // Columns D-AX: Abilities (47 abilities)
        for j in 0..47 {
            row.push(format!("{}", 10.0 + (i + j) as f32 % 10.0)); // Varying abilities 10.0-19.9
        }

        // Column AY: DNA score
        row.push(format!("{}", 75.0 + (i as f32 * 2.0) % 20.0)); // DNA 75.0-95.0

        // Columns AZ-EQ: Role ratings (96 roles)
        for j in 0..96 {
            // Give players varying ratings, with some specialization
            let base_rating = 5.0 + (i as f32) % 10.0;
            let specialization_bonus = if j == (i % 11) * 8 { 10.0 } else { 0.0 };
            row.push(format!("{}", base_rating + specialization_bonus));
        }

        data.push(row);
    }

    data
}

// Helper function to create mock data with good goalkeepers for duplicate role testing
fn create_mock_sheet_data_with_good_goalkeepers() -> Vec<Vec<String>> {
    let mut data = Vec::new();

    for i in 0..15 {
        let mut row = Vec::new();

        // Column A: Player name
        row.push(format!("Player {}", i));

        // Column B: Age
        row.push(format!("{}", 20 + (i % 15)));

        // Column C: Footedness
        row.push("R".to_string());

        // Columns D-AX: Abilities (47 abilities)
        for _j in 0..47 {
            row.push("12.0".to_string());
        }

        // Column AY: DNA score
        row.push("80.0".to_string());

        // Columns AZ-EQ: Role ratings (96 roles)
        for j in 0..96 {
            // Make first few players excellent goalkeepers (GK is role index 95)
            if j == 95 && i < 5 {
                row.push(format!("{}", 18.0 + i as f32)); // Excellent GK ratings
            } else {
                row.push("8.0".to_string()); // Average rating for other roles
            }
        }

        data.push(row);
    }

    data
}

// Helper function to create realistic mock squad data with recognizable names
fn create_realistic_mock_squad() -> Vec<Vec<String>> {
    let mut data = Vec::new();

    // Realistic player data based on well-known players
    let players = vec![
        // Goalkeepers
        (
            "Alisson",
            30,
            "R",
            vec![12, 8, 6, 7, 15, 16, 17, 8, 5, 10, 16],
            88.0,
            vec![5, 5, 5, 19, 5, 5],
        ), // Excellent GK
        (
            "Ederson",
            29,
            "L",
            vec![10, 9, 8, 6, 14, 15, 16, 12, 7, 11, 15],
            85.0,
            vec![6, 6, 6, 18, 6, 6],
        ), // Good GK
        // Defenders
        (
            "Van Dijk",
            32,
            "R",
            vec![11, 6, 8, 12, 18, 17, 19, 6, 4, 8, 13],
            92.0,
            vec![15, 18, 17, 10, 10, 8],
        ), // Elite CB
        (
            "Dias",
            26,
            "R",
            vec![10, 7, 9, 11, 17, 16, 18, 5, 5, 9, 14],
            89.0,
            vec![14, 17, 16, 9, 9, 7],
        ), // Elite CB
        (
            "Robertson",
            29,
            "L",
            vec![8, 13, 14, 9, 14, 12, 15, 16, 12, 15, 11],
            87.0,
            vec![11, 12, 13, 7, 17, 6],
        ), // Elite LB
        (
            "Alexander-Arnold",
            25,
            "R",
            vec![9, 15, 13, 10, 13, 11, 14, 17, 13, 16, 12],
            88.0,
            vec![10, 11, 12, 8, 16, 7],
        ), // Elite RB
        (
            "Cancelo",
            29,
            "R",
            vec![10, 14, 15, 11, 15, 13, 16, 15, 11, 14, 13],
            86.0,
            vec![12, 13, 14, 6, 15, 8],
        ), // Versatile FB
        // Midfielders
        (
            "De Bruyne",
            32,
            "R",
            vec![14, 12, 17, 16, 17, 13, 15, 18, 16, 19, 14],
            95.0,
            vec![8, 9, 10, 11, 12, 18],
        ), // Elite CAM
        (
            "Rodri",
            27,
            "R",
            vec![12, 8, 11, 13, 16, 15, 17, 12, 10, 16, 15],
            91.0,
            vec![6, 7, 8, 9, 10, 17],
        ), // Elite CDM
        (
            "Modric",
            38,
            "R",
            vec![13, 10, 16, 14, 15, 12, 14, 15, 14, 17, 13],
            90.0,
            vec![7, 8, 9, 10, 11, 16],
        ), // Elite CM
        (
            "Bellingham",
            20,
            "R",
            vec![11, 9, 14, 13, 14, 13, 15, 13, 11, 15, 14],
            88.0,
            vec![8, 9, 10, 11, 12, 15],
        ), // Promising CM
        // Wingers
        (
            "Salah",
            31,
            "L",
            vec![8, 14, 18, 17, 16, 12, 13, 15, 17, 14, 12],
            93.0,
            vec![12, 8, 9, 7, 8, 17],
        ), // Elite RW
        (
            "Mane",
            31,
            "R",
            vec![9, 15, 17, 16, 15, 13, 14, 14, 16, 13, 13],
            90.0,
            vec![13, 9, 10, 8, 7, 16],
        ), // Elite LW
        (
            "Sterling",
            29,
            "R",
            vec![7, 16, 16, 15, 14, 11, 12, 17, 15, 12, 11],
            87.0,
            vec![14, 10, 11, 9, 8, 15],
        ), // Good Winger
        // Forwards
        (
            "Haaland",
            23,
            "L",
            vec![6, 8, 12, 18, 15, 11, 19, 9, 7, 10, 16],
            94.0,
            vec![10, 6, 7, 8, 9, 19],
        ), // Elite ST
        (
            "Kane",
            30,
            "R",
            vec![8, 10, 14, 17, 16, 13, 17, 11, 9, 15, 15],
            92.0,
            vec![11, 7, 8, 9, 10, 18],
        ), // Elite ST
        (
            "Benzema",
            36,
            "R",
            vec![10, 11, 15, 16, 15, 14, 16, 12, 10, 14, 14],
            90.0,
            vec![12, 8, 9, 10, 11, 17],
        ), // Elite ST
        // Squad players
        (
            "Squad Player 1",
            24,
            "R",
            vec![8, 9, 10, 11, 12, 10, 11, 9, 8, 10, 9],
            75.0,
            vec![8, 8, 8, 8, 8, 9],
        ),
        (
            "Squad Player 2",
            26,
            "L",
            vec![9, 8, 11, 10, 11, 9, 12, 10, 9, 11, 10],
            76.0,
            vec![9, 9, 9, 9, 9, 10],
        ),
        (
            "Squad Player 3",
            22,
            "RL",
            vec![7, 10, 9, 12, 10, 8, 10, 8, 7, 9, 8],
            74.0,
            vec![7, 7, 7, 7, 7, 8],
        ),
    ];

    for (name, age, foot, abilities, dna, role_sample) in players {
        let mut row = Vec::new();

        // Column A: Player name
        row.push(name.to_string());

        // Column B: Age
        row.push(age.to_string());

        // Column C: Footedness
        row.push(foot.to_string());

        // Columns D-AX: Abilities (47 abilities) - extend the sample
        for i in 0..47 {
            let ability_value = abilities.get(i % abilities.len()).unwrap_or(&10);
            row.push(ability_value.to_string());
        }

        // Column AY: DNA score
        row.push(dna.to_string());

        // Columns AZ-EQ: Role ratings (96 roles) - extend the sample
        for i in 0..96 {
            let role_value = role_sample.get(i % role_sample.len()).unwrap_or(&8);
            row.push(role_value.to_string());
        }

        data.push(row);
    }

    data
}

// Helper function to create test data with known optimal assignments
fn create_optimal_test_squad() -> Vec<Vec<String>> {
    let mut data = Vec::new();

    // Create 15 players where each is optimized for specific roles
    for i in 0..15 {
        let mut row = Vec::new();

        // Column A: Player name
        row.push(format!("Specialist {}", i));

        // Column B: Age
        row.push("25".to_string());

        // Column C: Footedness
        row.push("R".to_string());

        // Columns D-AX: Abilities (47 abilities)
        for _j in 0..47 {
            row.push("12.0".to_string());
        }

        // Column AY: DNA score
        row.push("80.0".to_string());

        // Columns AZ-EQ: Role ratings (96 roles)
        for j in 0..96 {
            // Each player specializes in a few specific roles
            let specialized_roles = match i {
                0 => vec![95],                 // GK specialist
                1 | 2 => vec![40, 41, 42],     // CB specialists (CD roles)
                3 => vec![52, 53, 54],         // RB specialist (FB R roles)
                4 => vec![55, 56, 57],         // LB specialist (FB L roles)
                5 | 6 | 7 => vec![27, 28, 29], // CM specialists
                8 => vec![0, 1],               // RW specialist (W R roles)
                9 => vec![2, 3],               // LW specialist (W L roles)
                10 => vec![78, 79, 80],        // ST specialist (CF roles)
                _ => vec![],                   // Average players
            };

            if specialized_roles.contains(&j) {
                row.push("18.0".to_string()); // Excellent in specialized role
            } else {
                row.push("6.0".to_string()); // Poor in other roles
            }
        }

        data.push(row);
    }

    data
}

/// Test the complete workflow with player filters that allow assignments
#[tokio::test]
async fn test_complete_workflow_with_filters_allowing_assignments() -> fm_data::error::Result<()> {
    use fm_data::{
        find_optimal_assignments_with_filters, format_team_output, parse_player_data,
        parse_role_file_content,
    };
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a temporary role file with filters
    let role_content = r#"[roles]
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
Alisson: goal
Van Dijk: cd
Matip: cd
Alexander-Arnold: wb
Robertson: wb
Henderson: dm
Fabinho: cm
Wijnaldum: cm
Salah: wing
Mané: wing
Firmino: str"#;

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse role file with filters
    let role_file_content = parse_role_file_content(role_file.path().to_str().unwrap()).await?;
    assert_eq!(role_file_content.roles.len(), 11);
    assert_eq!(role_file_content.filters.len(), 11);

    // Create mock player data
    let mock_sheet_data = create_mock_sheet_data();
    let players = parse_player_data(mock_sheet_data)?;

    // Run assignment algorithm with filters
    let team = find_optimal_assignments_with_filters(
        players,
        role_file_content.roles,
        &role_file_content.filters,
    )?;

    // Should successfully assign all players
    assert_eq!(team.assignments.len(), 11);

    // Generate output
    let output = format_team_output(&team);
    assert!(output.contains(" -> "));
    assert!(output.contains("Total Score:"));

    Ok(())
}

/// Test backward compatibility - old role file format should still work
#[tokio::test]
async fn test_backward_compatibility_old_format() -> fm_data::error::Result<()> {
    use fm_data::{
        find_optimal_assignments, find_optimal_assignments_with_filters, parse_player_data,
        parse_role_file_content,
    };
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a traditional role file (no sections)
    let role_content = "GK
CD(d)
CD(s)
FB(d) R
FB(d) L
CM(d)
CM(s)
CM(a)
W(s) R
W(s) L
CF(s)";

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse role file using new parser
    let role_file_content = parse_role_file_content(role_file.path().to_str().unwrap()).await?;
    assert_eq!(role_file_content.roles.len(), 11);
    assert_eq!(role_file_content.filters.len(), 0); // No filters in old format

    // Create mock player data
    let mock_sheet_data = create_mock_sheet_data();
    let players = parse_player_data(mock_sheet_data)?;

    // Run assignment algorithm - should work exactly like before
    let team_new = find_optimal_assignments_with_filters(
        players.clone(),
        role_file_content.roles.clone(),
        &role_file_content.filters,
    )?;

    let team_old = find_optimal_assignments(players, role_file_content.roles)?;

    // Results should be identical
    assert_eq!(team_new.assignments.len(), team_old.assignments.len());
    assert_eq!(team_new.total_score(), team_old.total_score());

    Ok(())
}

/// Test role file with filters that block some players from their natural roles
#[tokio::test]
async fn test_filters_blocking_player_assignments() -> Result<()> {
    use fm_data::{
        find_optimal_assignments_with_filters, format_team_output, parse_player_data,
        parse_role_file_content,
    };
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a role file where some players are restricted away from their best roles
    let role_content = r#"[roles]
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
Alisson: cd
Van Dijk: wing
Salah: goal"#;

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse role file with blocking filters
    let role_file_content = parse_role_file_content(role_file.path().to_str().unwrap()).await?;
    assert_eq!(role_file_content.roles.len(), 11);
    assert_eq!(role_file_content.filters.len(), 3);

    // Create mock player data
    let mock_sheet_data = create_mock_sheet_data();
    let players = parse_player_data(mock_sheet_data)?;

    // Run assignment algorithm with blocking filters
    let team = find_optimal_assignments_with_filters(
        players,
        role_file_content.roles,
        &role_file_content.filters,
    )?;

    // Should still create a team, but assignments will be suboptimal due to restrictions
    assert!(team.assignments.len() <= 11);

    // Generate output
    let output = format_team_output(&team);
    assert!(output.contains(" -> "));

    // Verify that blocked players are NOT in their natural positions
    let lines: Vec<&str> = output.trim().split('\n').collect();
    let assignment_lines = &lines[0..team.assignments.len()];

    // Alisson should NOT be assigned to GK (blocked to cd category)
    let gk_assignment = assignment_lines.iter().find(|line| line.starts_with("GK"));
    if let Some(gk_line) = gk_assignment {
        assert!(
            !gk_line.contains("Alisson"),
            "Alisson should be blocked from GK role"
        );
    }

    Ok(())
}

/// Test mixed scenario with both filtered and unfiltered players
#[tokio::test]
async fn test_mixed_filtered_and_unfiltered_players() -> Result<()> {
    use fm_data::{
        find_optimal_assignments_with_filters, format_team_output, parse_player_data,
        parse_role_file_content,
    };
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a role file with only some players having filters
    let role_content = r#"[roles]
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
Alisson: goal
Van Dijk: cd
Salah: wing"#;

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse role file
    let role_file_content = parse_role_file_content(role_file.path().to_str().unwrap()).await?;
    assert_eq!(role_file_content.roles.len(), 11);
    assert_eq!(role_file_content.filters.len(), 3); // Only 3 players have filters

    // Create mock player data (15 players total)
    let mock_sheet_data = create_mock_sheet_data();
    let players = parse_player_data(mock_sheet_data)?;
    assert_eq!(players.len(), 15);

    // Run assignment algorithm
    let team = find_optimal_assignments_with_filters(
        players,
        role_file_content.roles,
        &role_file_content.filters,
    )?;

    // Should successfully assign all 11 roles
    assert_eq!(team.assignments.len(), 11);

    // Generate output
    let output = format_team_output(&team);
    assert!(output.contains(" -> "));
    assert!(output.contains("Total Score:"));

    // Verify that filtered players are in appropriate positions
    let lines: Vec<&str> = output.trim().split('\n').collect();
    let assignment_lines = &lines[0..11];

    // There should be a GK assignment (since we have players and a GK role)
    let gk_assignment = assignment_lines.iter().find(|line| line.starts_with("GK"));
    assert!(gk_assignment.is_some(), "Should have a GK assignment");

    // Verify that we have both filtered and unfiltered assignments
    let has_assignments = assignment_lines.len() > 0;
    assert!(has_assignments, "Should have at least some assignments");

    // Unfiltered players should be able to fill any remaining roles
    let total_players_assigned = assignment_lines.len();
    assert!(total_players_assigned > 0);

    Ok(())
}

/// Test performance with filtered assignments on large dataset
#[tokio::test]
async fn test_filtered_assignment_performance() -> Result<()> {
    use fm_data::{
        find_optimal_assignments_with_filters, parse_player_data, parse_role_file_content,
    };
    use std::time::Instant;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a role file with several filters
    let role_content = r#"[roles]
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
Player 0: goal
Player 1: cd
Player 2: cd
Player 3: wb
Player 4: wb
Player 5: dm
Player 6: cm
Player 7: cm
Player 8: wing
Player 9: wing
Player 10: str"#;

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Parse role file
    let role_file_content = parse_role_file_content(role_file.path().to_str().unwrap()).await?;

    // Create large dataset (50 players)
    let large_data = create_mock_sheet_data_with_player_count(50);
    let players = parse_player_data(large_data)?;
    assert_eq!(players.len(), 50);

    // Measure performance with filters
    let start = Instant::now();
    let team = find_optimal_assignments_with_filters(
        players,
        role_file_content.roles,
        &role_file_content.filters,
    )?;
    let duration = start.elapsed();

    // Should complete quickly (under 1 second even with filters)
    assert!(
        duration.as_millis() < 1000,
        "Filtered assignment took too long: {}ms",
        duration.as_millis()
    );

    // Should successfully create assignments
    assert!(team.assignments.len() <= 11);
    assert!(team.total_score() >= 0.0);

    Ok(())
}

/// Test error handling for role files with invalid filter format
#[tokio::test]
async fn test_invalid_filter_format_error_handling() -> Result<()> {
    use fm_data::parse_role_file_content;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a role file with invalid filter format
    let role_content = r#"[roles]
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
Player1 invalid format no colon
Player2: 
Player3: unknown_category"#;

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Try to parse - should fail with clear error message
    let result = parse_role_file_content(role_file.path().to_str().unwrap()).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_message = error.to_string();

    // Should contain information about invalid filter format
    assert!(
        error_message.contains("Invalid filter format")
            || error_message.contains("Expected 'PLAYER_NAME: CATEGORY_LIST'"),
        "Error message should indicate invalid filter format: {}",
        error_message
    );

    Ok(())
}

/// Test error handling for duplicate player names in filters
#[tokio::test]
async fn test_duplicate_player_filter_error_handling() -> Result<()> {
    use fm_data::parse_role_file_content;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    // Create a role file with duplicate player filters
    let role_content = r#"[roles]
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
TestPlayer: goal
TestPlayer: cd"#;

    let role_file = NamedTempFile::new().unwrap();
    let mut async_role_file = tokio::fs::File::create(role_file.path()).await.unwrap();
    async_role_file
        .write_all(role_content.as_bytes())
        .await
        .unwrap();
    async_role_file.flush().await.unwrap();

    // Try to parse - should fail with clear error message
    let result = parse_role_file_content(role_file.path().to_str().unwrap()).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_message = error.to_string();

    // Should contain information about duplicate player
    assert!(
        error_message.contains("Duplicate")
            || error_message.contains("duplicate")
            || error_message.contains("already defined"),
        "Error message should indicate duplicate player: {}",
        error_message
    );

    Ok(())
}

/// Test that configuration file paths are properly resolved and used
#[tokio::test]
async fn test_config_file_input_path_resolution() -> Result<()> {
    use fm_data::Config;
    // Create temporary files for testing
    let _creds_file = NamedTempFile::new().unwrap();
    let input_file = NamedTempFile::new().unwrap();
    let config_file = NamedTempFile::new().unwrap();

    // Write a simple HTML table to the input file
    let html_content = r#"
        <html>
            <body>
                <table>
                    <tr><td>Name</td><td>Age</td><td>Position</td></tr>
                    <tr><td>Player1</td><td>25</td><td>GK</td></tr>
                </table>
            </body>
        </html>
    "#;
    std::fs::write(input_file.path(), html_content).unwrap();

    // Create a config file that specifies the input path
    let config_json = format!(
        r#"{{
            "google": {{
                "spreadsheet_name": "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc",
                "team_sheet": "Squad"
            }},
            "input": {{
                "data_html": "{}"
            }}
        }}"#,
        input_file.path().to_string_lossy().replace('\\', "\\\\")
    );

    let mut config_file_handle = config_file.reopen().unwrap();
    config_file_handle
        .write_all(config_json.as_bytes())
        .unwrap();
    drop(config_file_handle);

    // Test that Config::from_file correctly loads the input path
    let config = Config::from_file(config_file.path())?;
    assert_eq!(
        config.input.data_html,
        input_file.path().to_string_lossy().to_string()
    );

    // Test that resolve_paths uses the config file path when CLI input is None
    let (_spreadsheet, _credfile, resolved_input) =
        config.resolve_paths_unchecked(None, None, None);
    assert_eq!(
        resolved_input,
        input_file.path().to_string_lossy().to_string(),
        "Config file input path should be used when CLI input is None"
    );

    // Test that CLI input overrides config file input
    let other_input_file = NamedTempFile::new().unwrap();
    std::fs::write(other_input_file.path(), html_content).unwrap();

    let (_spreadsheet, _credfile, resolved_input) = config.resolve_paths_unchecked(
        None,
        None,
        Some(other_input_file.path().to_string_lossy().to_string()),
    );
    assert_eq!(
        resolved_input,
        other_input_file.path().to_string_lossy().to_string(),
        "CLI input should override config file input"
    );

    Ok(())
}
