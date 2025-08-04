use super::types::Team;

/// Format team output for clean stdout display
pub fn format_team_output(team: &Team) -> String {
    let mut output = String::new();

    // Sort assignments by role name for consistent output
    let sorted_assignments = team.sorted_by_role();

    // Format each assignment as "$ROLE -> $PLAYER_NAME (score: $SCORE)"
    for assignment in sorted_assignments {
        output.push_str(&format!(
            "{} -> {} (score: {:.1})\n",
            assignment.role.name, assignment.player.name, assignment.score
        ));
    }

    // Include total team score
    output.push_str(&format!("Total Score: {:.1}\n", team.total_score()));

    output
}

/// Format assignment summary with additional team statistics
pub fn format_assignment_summary(team: &Team) -> String {
    format!(
        "Team of {} players with total score: {:.1}",
        team.assignments.len(),
        team.total_score()
    )
}