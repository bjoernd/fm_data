use crate::browser::{BrowserPlayer, COLUMN_HEADERS};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
use std::io;

/// Main browser application state that manages the terminal UI
pub struct BrowserApp {
    /// List of players to display in the table
    players: Vec<BrowserPlayer>,
    /// Currently selected row (player index)
    selected_row: usize,
    /// Currently selected column index
    selected_col: usize,
    /// Flag to indicate when the application should quit
    should_quit: bool,
    /// Table state for managing selection
    table_state: TableState,
}

impl BrowserApp {
    /// Creates a new BrowserApp with the given players
    pub fn new(players: Vec<BrowserPlayer>) -> Self {
        let mut table_state = TableState::default();
        if !players.is_empty() {
            table_state.select(Some(0));
        }

        Self {
            players,
            selected_row: 0,
            selected_col: 0,
            should_quit: false,
            table_state,
        }
    }

    /// Main run loop for the application
    /// Sets up terminal, handles events, and manages cleanup
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            // Draw the current state
            terminal.draw(|f| self.draw(f))?;

            // Handle input events
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.should_quit = true;
                    }
                    KeyCode::Up => self.move_up(),
                    KeyCode::Down => self.move_down(),
                    KeyCode::Left => self.move_left(),
                    KeyCode::Right => self.move_right(),
                    _ => {}
                }
            }

            // Check if we should quit
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    /// Draw the UI frame with the player table
    fn draw(&mut self, f: &mut Frame) {
        // Create the main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0)].as_ref())
            .split(f.size());

        // Create table headers
        let header_cells = COLUMN_HEADERS
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells)
            .style(Style::default().bg(Color::DarkGray))
            .height(1);

        // Create table rows from player data
        let rows: Vec<Row> = self
            .players
            .iter()
            .enumerate()
            .map(|(i, player)| {
                let cells: Vec<Cell> = (0..COLUMN_HEADERS.len())
                    .map(|col_idx| {
                        let content = player.get_field_by_index(col_idx);
                        let style = if i == self.selected_row && col_idx == self.selected_col {
                            Style::default()
                                .bg(Color::Blue)
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD)
                        } else if i == self.selected_row {
                            Style::default().bg(Color::DarkGray)
                        } else {
                            Style::default()
                        };
                        Cell::from(content).style(style)
                    })
                    .collect();
                Row::new(cells).height(1)
            })
            .collect();

        // Create column constraints (all equal width for now)
        let constraints: Vec<Constraint> = (0..COLUMN_HEADERS.len())
            .map(|_| Constraint::Min(8))
            .collect();

        // Create the table widget
        let table = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Football Manager Player Browser - Press 'q' or ESC to quit"),
            )
            .widths(&constraints)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        // Render the table
        f.render_stateful_widget(table, chunks[0], &mut self.table_state);
    }

    /// Move selection up one row
    fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            self.table_state.select(Some(self.selected_row));
        }
    }

    /// Move selection down one row
    fn move_down(&mut self) {
        if self.selected_row < self.players.len().saturating_sub(1) {
            self.selected_row += 1;
            self.table_state.select(Some(self.selected_row));
        }
    }

    /// Move selection left one column
    fn move_left(&mut self) {
        if self.selected_col > 0 {
            self.selected_col -= 1;
        }
    }

    /// Move selection right one column
    fn move_right(&mut self) {
        if self.selected_col < COLUMN_HEADERS.len().saturating_sub(1) {
            self.selected_col += 1;
        }
    }
}

/// Setup the terminal for TUI mode
pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to normal mode
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_players() -> Vec<BrowserPlayer> {
        let mut row = vec![
            "Test Player 1".to_string(),
            "25.0".to_string(),
            "Right".to_string(),
        ];
        // Add remaining 142 fields
        for i in 3..145 {
            row.push(format!("{}.0", i % 20));
        }

        let player1 = BrowserPlayer::from_row(&row).unwrap();

        let mut row2 = vec![
            "Test Player 2".to_string(),
            "27.5".to_string(),
            "Left".to_string(),
        ];
        // Add remaining 142 fields
        for i in 3..145 {
            row2.push(format!("{}.5", i % 15));
        }

        let player2 = BrowserPlayer::from_row(&row2).unwrap();

        vec![player1, player2]
    }

    #[test]
    fn test_browser_app_creation() {
        let players = create_test_players();
        let app = BrowserApp::new(players.clone());

        assert_eq!(app.players.len(), 2);
        assert_eq!(app.selected_row, 0);
        assert_eq!(app.selected_col, 0);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_browser_app_empty_players() {
        let app = BrowserApp::new(vec![]);

        assert_eq!(app.players.len(), 0);
        assert_eq!(app.selected_row, 0);
        assert_eq!(app.selected_col, 0);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_navigation_up_down() {
        let players = create_test_players();
        let mut app = BrowserApp::new(players);

        // Start at row 0
        assert_eq!(app.selected_row, 0);

        // Move down
        app.move_down();
        assert_eq!(app.selected_row, 1);

        // Try to move down past last row
        app.move_down();
        assert_eq!(app.selected_row, 1); // Should stay at 1

        // Move up
        app.move_up();
        assert_eq!(app.selected_row, 0);

        // Try to move up past first row
        app.move_up();
        assert_eq!(app.selected_row, 0); // Should stay at 0
    }

    #[test]
    fn test_navigation_left_right() {
        let players = create_test_players();
        let mut app = BrowserApp::new(players);

        // Start at column 0
        assert_eq!(app.selected_col, 0);

        // Move right
        app.move_right();
        assert_eq!(app.selected_col, 1);

        // Move right again
        app.move_right();
        assert_eq!(app.selected_col, 2);

        // Move left
        app.move_left();
        assert_eq!(app.selected_col, 1);

        // Move left again
        app.move_left();
        assert_eq!(app.selected_col, 0);

        // Try to move left past first column
        app.move_left();
        assert_eq!(app.selected_col, 0); // Should stay at 0
    }

    #[test]
    fn test_navigation_right_boundary() {
        let players = create_test_players();
        let mut app = BrowserApp::new(players);

        let max_col = COLUMN_HEADERS.len() - 1;

        // Move to last column
        app.selected_col = max_col;

        // Try to move right past last column
        app.move_right();
        assert_eq!(app.selected_col, max_col); // Should stay at max
    }

    #[test]
    fn test_column_headers_consistency() {
        // Verify our UI is consistent with the browser module
        assert_eq!(COLUMN_HEADERS.len(), 145);
        assert_eq!(COLUMN_HEADERS[0], "Name");
        assert_eq!(COLUMN_HEADERS[1], "Age");
        assert_eq!(COLUMN_HEADERS[2], "Foot");
    }
}
