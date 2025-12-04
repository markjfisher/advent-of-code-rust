use std::time::{Duration, Instant};

use color_eyre::{eyre::Context, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

use crate::aoc2025::day04::DIGIT_MAP;
use crate::util::grid::Grid;
use crate::util::point::{Point, DIAGONAL};

pub struct App {
    /// All states from initial to final stable 4-core
    history: Vec<Grid<u8>>,
    /// Current index into `history`
    current_index: usize,
    /// Whether we are auto-playing
    playing: bool,
    /// Delay between frames when playing (ms)
    frame_delay_ms: u64,
    /// Whether we show heatmap (blocks) or numeric counts
    heat_mode: bool,
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn viz(grid: Grid<u8>) -> Result<()> {
    // setup
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let mut terminal = ratatui::init();
    terminal.clear().context("Failed to clear terminal")?;

    // start an App to run the visualisation
    let mut app = App::new(grid);
    let app_result = app.run(terminal).context("Failed to run application");

    // restore the terminal
    ratatui::restore();
    app_result
}

impl App {
    pub fn new(initial_grid: Grid<u8>) -> Self {
        let history = compute_full_history(&initial_grid);
        Self {
            history,
            current_index: 0,
            playing: false,
            frame_delay_ms: 200, // 200ms default
            heat_mode: false,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Draw initial state and start paused
        terminal
            .draw(|frame| self.draw(frame))
            .context("Failed to draw frame")?;

        let mut last_advance = Instant::now();

        loop {
            if self.playing {
                // While playing, poll frequently so keys are responsive.
                if event::poll(Duration::from_millis(10)).context("Failed to poll events")? {
                    if let Event::Key(key) = event::read().context("Failed to read event")? {
                        if self.handle_key(key.code) {
                            break;
                        }
                        // Any manual navigation resets the advance timer to feel snappy
                        last_advance = Instant::now();
                    }
                }

                // Advance automatically according to frame_delay_ms
                if last_advance.elapsed() >= Duration::from_millis(self.frame_delay_ms) {
                    if self.current_index + 1 < self.history.len() {
                        self.current_index += 1;
                        last_advance = Instant::now();
                    } else {
                        // Reached final state: stop playing
                        self.playing = false;
                    }
                }
            } else {
                // Paused: block until a key event
                if let Event::Key(key) = event::read().context("Failed to read event")? {
                    if self.handle_key(key.code) {
                        break;
                    }
                    last_advance = Instant::now();
                }
            }

            terminal
                .draw(|frame| self.draw(frame))
                .context("Failed to draw frame")?;
        }

        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                return true; // quit
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.reset();
            }
            KeyCode::Char(' ') | KeyCode::Right => {
                self.step_forward();
            }
            KeyCode::Left => {
                self.step_backward();
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                self.playing = !self.playing;
            }
            KeyCode::Char('+') => {
                self.speed_up();
            }
            KeyCode::Char('-') => {
                self.slow_down();
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.heat_mode = !self.heat_mode;
            }
            _ => {}
        }
        false
    }

    fn reset(&mut self) {
        self.current_index = 0;
        self.playing = false;
    }

    fn step_forward(&mut self) {
        if self.current_index + 1 < self.history.len() {
            self.current_index += 1;
        }
    }

    fn step_backward(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    fn speed_up(&mut self) {
        // Decrease delay (faster), clamp to a sensible minimum
        if self.frame_delay_ms > 20 {
            self.frame_delay_ms = (self.frame_delay_ms as f32 * 0.8).round() as u64;
            if self.frame_delay_ms < 20 {
                self.frame_delay_ms = 20;
            }
        }
    }

    fn slow_down(&mut self) {
        // Increase delay (slower), clamp to a sensible maximum
        if self.frame_delay_ms < 2000 {
            self.frame_delay_ms = (self.frame_delay_ms as f32 * 1.25).round() as u64;
            if self.frame_delay_ms > 2000 {
                self.frame_delay_ms = 2000;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let current_grid = &self.history[self.current_index];
        let neighbour_counts = compute_full_neighbour_counts(current_grid);

        let at_end = self.current_index + 1 >= self.history.len();
        let mode_label = if self.playing { "PLAY" } else { "PAUSE" };
        let heat_label = if self.heat_mode { "HEAT" } else { "DIGITS" };

        // First chunk: exact grid height (+2 for borders)
        // Second chunk: status line
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length((current_grid.height as u16).saturating_add(2)),
                Constraint::Min(1),
            ])
            .split(area);

        // Turn neighbour_counts into a string using your DIGIT_MAP (for DIGITS mode)
        let grid_str = neighbour_counts.to_grid_string_with_map(Some(&DIGIT_MAP));

        // Build coloured lines, either DIGITS or HEATMAP style
        let lines: Vec<Line> = grid_str
            .lines()
            .enumerate()
            .map(|(y, row)| {
                let spans: Vec<Span> = row
                    .chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        let value = match ch {
                            '1' => 1,
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            _ => 0,
                        };

                        if self.heat_mode {
                            // Heatmap mode: block characters + colour spectrum
                            let block_char = match value {
                                0 => ' ',
                                1 | 2 => '░',
                                3 | 4 => '▒',
                                5 | 6 => '▓',
                                _ => '█',
                            };

                            let color = match value {
                                0 => Color::Black,
                                1 => Color::Rgb(0, 0, 128),     // dark blue
                                2 => Color::Rgb(0, 128, 255),   // cyan-ish
                                3 => Color::Rgb(0, 200, 0),     // green
                                4 => Color::Rgb(180, 180, 0),   // yellow-ish
                                5 => Color::Rgb(255, 165, 0),   // orange
                                6 => Color::Rgb(255, 0, 0),     // red
                                7 => Color::Rgb(180, 0, 0),     // dark red
                                _ => Color::Rgb(128, 0, 0),     // very dark red
                            };

                            // Optional: you could also emphasize live '@' cells differently by
                            // looking into current_grid[Point::new(x as i32, y as i32)] here.
                            let _p = Point::new(x as i32, y as i32);
                            Span::styled(block_char.to_string(), Style::default().fg(color))
                        } else {
                            // DIGITS mode: show the digit itself, grayscale-ish
                            let color = match value {
                                0 => Color::Black,
                                1 => Color::DarkGray,
                                2 => Color::Gray,
                                3 => Color::Rgb(180, 180, 180),
                                4 => Color::White,
                                5 => Color::Rgb(200, 200, 255),
                                6 => Color::Rgb(180, 180, 255),
                                7 => Color::Rgb(160, 160, 255),
                                _ => Color::Rgb(140, 140, 255),
                            };

                            Span::styled(ch.to_string(), Style::default().fg(color))
                        }
                    })
                    .collect();

                Line::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Neighbour counts – step {} / {}  [{} | {}]",
                    self.current_index,
                    self.history.len() - 1,
                    mode_label,
                    heat_label
                )),
        );

        frame.render_widget(paragraph, chunks[0]);

        // Status line with controls and speed
        let status_text = if at_end {
            format!(
                "[{} | {}] final state  |  [Space/→] no-op  [←] back  [P] play/pause  [+/-] speed={}ms  [H] heatmap  [R] reset  [Q] quit",
                mode_label, heat_label, self.frame_delay_ms
            )
        } else {
            format!(
                "[{} | {}]  |  [Space/→] step forward  [←] step back  [P] play/pause  [+/-] speed={}ms  [H] heatmap  [R] reset  [Q] quit",
                mode_label, heat_label, self.frame_delay_ms
            )
        };

        let status = Paragraph::new(status_text);
        frame.render_widget(status, chunks[1]);
    }
}

/// Precompute the entire sequence of erosion states from the initial grid
/// until no more '@' cells have < 4 neighbours.
///
/// This uses the simple "full recompute neighbour counts each wave" logic,
/// purely for visualisation.
fn compute_full_history(initial: &Grid<u8>) -> Vec<Grid<u8>> {
    let mut history = Vec::new();
    history.push(initial.clone());

    loop {
        let last = history.last().unwrap();
        let neighbour_counts = compute_full_neighbour_counts(last);

        let mut next = last.clone();
        let mut changed = false;

        for p in last.points() {
            if last[p] == b'@' && neighbour_counts[p] < 4 {
                next[p] = b'.';
                changed = true;
            }
        }

        if !changed {
            break;
        }

        history.push(next);
    }

    history
}

/// Recompute the true neighbour counts for every cell in the grid.
/// This is *for visualisation*, so we favour clarity over speed.
fn compute_full_neighbour_counts(grid: &Grid<u8>) -> Grid<u8> {
    let mut out = grid.same_size_with(0u8);

    for p in grid.points() {
        if grid[p] == b'@' {
            let mut count = 0;
            for &dir in DIAGONAL.iter() {
                let n = p + dir;
                if grid.contains(n) && grid[n] == b'@' {
                    count += 1;
                }
            }
            out[p] = count;
        } else {
            out[p] = 0;
        }
    }

    out
}
