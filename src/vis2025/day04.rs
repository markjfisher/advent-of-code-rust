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

use crate::util::grid::Grid;
use crate::util::point::{Point, DIAGONAL};

pub struct App {
    /// All states from initial to final stable 4-core
    history: Vec<Grid<u8>>,
    /// Number of live '@' cells at each step (same length as history)
    alive_counts: Vec<usize>,
    /// Current index into `history`
    current_index: usize,
    /// Whether we are auto-playing
    playing: bool,
    /// Delay between frames when playing (ms)
    frame_delay_ms: u64,
    /// Whether we show heatmap (blocks) or numeric counts
    heat_mode: bool,
    /// Viewport top-left in grid coordinates
    viewport_x: i32,
    viewport_y: i32,
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn viz(grid: Grid<u8>) -> Result<()> {
    // setup
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let mut terminal = ratatui::init();
    terminal.clear().context("Failed to clear terminal")?;

    let mut app = App::new(grid);
    let app_result = app.run(terminal).context("Failed to run application");

    ratatui::restore();
    app_result
}

impl App {
    pub fn new(initial_grid: Grid<u8>) -> Self {
        let (history, alive_counts) = compute_full_history(&initial_grid);
        Self {
            history,
            alive_counts,
            current_index: 0,
            playing: false,
            frame_delay_ms: 200,
            heat_mode: false,
            viewport_x: 0,
            viewport_y: 0,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Initial draw, paused
        terminal
            .draw(|frame| self.draw(frame))
            .context("Failed to draw frame")?;

        let mut last_advance = Instant::now();

        loop {
            if self.playing {
                // When playing, poll quickly for key events (so panning & toggles are responsive)
                if event::poll(Duration::from_millis(10)).context("Failed to poll events")? {
                    if let Event::Key(key) = event::read().context("Failed to read event")? {
                        if self.handle_key(key.code, &terminal) {
                            break;
                        }
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
                    if self.handle_key(key.code, &terminal) {
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

    fn handle_key(&mut self, code: KeyCode, terminal: &DefaultTerminal) -> bool {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                return true; // quit
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.reset();
            }
            // Time navigation
            KeyCode::Char('.') => {
                self.step_forward();
            }
            KeyCode::Char(',') => {
                self.step_backward();
            }
            KeyCode::Char(' ') => {
                self.step_forward();
            }
            // Play/pause
            KeyCode::Char('p') | KeyCode::Char('P') => {
                self.playing = !self.playing;
            }
            // Speed control
            KeyCode::Char('+') => {
                self.speed_up();
            }
            KeyCode::Char('-') => {
                self.slow_down();
            }
            // Heatmap toggle
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.heat_mode = !self.heat_mode;
            }
            // Viewport panning
            KeyCode::Up => {
                self.viewport_y -= 1;
            }
            KeyCode::Down => {
                self.viewport_y += 1;
            }
            KeyCode::Left => {
                self.viewport_x -= 1;
            }
            KeyCode::Right => {
                self.viewport_x += 1;
            }
            _ => {}
        }

        // Clamp viewport immediately using current terminal size
        if let Ok(size) = terminal.size() {
            self.clamp_viewport(size.width as i32, size.height as i32);
        }

        false
    }

    fn reset(&mut self) {
        self.current_index = 0;
        self.playing = false;
        self.viewport_x = 0;
        self.viewport_y = 0;
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
        if self.frame_delay_ms > 20 {
            self.frame_delay_ms = (self.frame_delay_ms as f32 * 0.8).round() as u64;
            if self.frame_delay_ms < 20 {
                self.frame_delay_ms = 20;
            }
        }
    }

    fn slow_down(&mut self) {
        if self.frame_delay_ms < 2000 {
            self.frame_delay_ms = (self.frame_delay_ms as f32 * 1.25).round() as u64;
            if self.frame_delay_ms > 2000 {
                self.frame_delay_ms = 2000;
            }
        }
    }

    fn clamp_viewport(&mut self, term_w: i32, term_h: i32) {
        let grid = &self.history[self.current_index];

        // Approximate usable area inside margins and borders
        let usable_h = (term_h - 4).max(1); // -2 margin, -2 borders
        let usable_w = (term_w - 4).max(1);

        let max_offset_x = (grid.width - usable_w).max(0);
        let max_offset_y = (grid.height - usable_h).max(0);

        if grid.width <= usable_w {
            self.viewport_x = 0;
        } else {
            self.viewport_x = self.viewport_x.clamp(0, max_offset_x);
        }

        if grid.height <= usable_h {
            self.viewport_y = 0;
        } else {
            self.viewport_y = self.viewport_y.clamp(0, max_offset_y);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let grid = &self.history[self.current_index];
        let neighbour_counts = compute_full_neighbour_counts(grid);

        // "Queue size" in the efficient algorithm sense:
        // how many '@' cells currently have neighbour_count < 4
        let mut queue_size = 0usize;
        for p in grid.points() {
            if grid[p] == b'@' && neighbour_counts[p] < 4 {
                queue_size += 1;
            }
        }

        let at_end = self.current_index + 1 >= self.history.len();
        let mode_label = if self.playing { "PLAY" } else { "PAUSE" };
        let heat_label = if self.heat_mode { "HEAT" } else { "DIGITS" };

        let alive_current = self.alive_counts[self.current_index];
        let removed_this_step = if self.current_index == 0 {
            0
        } else {
            self.alive_counts[self.current_index - 1] - alive_current
        };

        // Vertical layout: big area for grid, small area for status
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(3), Constraint::Length(2)])
            .split(area);

        let grid_area = chunks[0];

        // Visible content area inside the bordered block
        let content_w = grid_area.width.saturating_sub(2) as i32;
        let content_h = grid_area.height.saturating_sub(2) as i32;

        if content_w <= 0 || content_h <= 0 {
            return;
        }

        // Clamp viewport based on actual content area
        let max_offset_x = (grid.width - content_w).max(0);
        let max_offset_y = (grid.height - content_h).max(0);

        if grid.width <= content_w {
            self.viewport_x = 0;
        } else {
            self.viewport_x = self.viewport_x.clamp(0, max_offset_x);
        }

        if grid.height <= content_h {
            self.viewport_y = 0;
        } else {
            self.viewport_y = self.viewport_y.clamp(0, max_offset_y);
        }

        let visible_cols = content_w.min(grid.width);
        let visible_rows = content_h.min(grid.height);

        let mut lines: Vec<Line> = Vec::with_capacity(visible_rows as usize);

        for row in 0..visible_rows {
            let gy = self.viewport_y + row;
            let mut spans: Vec<Span> = Vec::with_capacity(visible_cols as usize);

            for col in 0..visible_cols {
                let gx = self.viewport_x + col;
                let p = Point::new(gx, gy);
                let value = neighbour_counts[p] as u8;
                let alive = grid[p] == b'@';

                if self.heat_mode {
                    // Heatmap mode: block characters + colour spectrum
                    let block_char = match value {
                        0 => ' ',
                        1 | 2 => '░',
                        3 | 4 => '▒',
                        5 | 6 => '▓',
                        _ => '█',
                    };

                    let base_color = match value {
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

                    // Emphasize live '@' cells a bit by brightening
                    let color = if alive {
                        match base_color {
                            Color::Rgb(r, g, b) => {
                                Color::Rgb(r.saturating_add(40), g.saturating_add(40), b.saturating_add(40))
                            }
                            other => other,
                        }
                    } else {
                        base_color
                    };

                    spans.push(Span::styled(block_char.to_string(), Style::default().fg(color)));
                } else {
                    // DIGITS mode: show the digit itself, grayscale-ish
                    let ch = match value {
                        0 => ' ',
                        n => char::from(b'0' + n),
                    };

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

                    let style = if alive {
                        Style::default().fg(color)
                    } else {
                        Style::default().fg(color)
                    };

                    spans.push(Span::styled(ch.to_string(), style));
                }
            }

            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Neighbour counts – step {} / {}  removed={}  alive={}  queue={}  [{} | {}]  view=({}, {})",
                    self.current_index,
                    self.history.len() - 1,
                    removed_this_step,
                    alive_current,
                    queue_size,
                    mode_label,
                    heat_label,
                    self.viewport_x,
                    self.viewport_y,
                )),
        );

        frame.render_widget(paragraph, grid_area);

        // Status line with controls and speed
        let status_text = if at_end {
            format!(
                "[{} | {}] final state  |  [,] prev  [.] next(no-op)  [Arrows] pan  [P] play/pause  [+/-] speed={}ms  [H] heatmap  [R] reset  [Q] quit",
                mode_label, heat_label, self.frame_delay_ms
            )
        } else {
            format!(
                "[{} | {}]  |  [,] prev  [.] next  [Arrows] pan  [P] play/pause  [+/-] speed={}ms  [H] heatmap  [R] reset  [Q] quit",
                mode_label, heat_label, self.frame_delay_ms
            )
        };

        let status = Paragraph::new(status_text);
        frame.render_widget(status, chunks[1]);
    }
}

/// Precompute the entire sequence of erosion states from the initial grid
/// until no more '@' cells have < 4 neighbours.
/// Also track how many '@' are alive at each step.
fn compute_full_history(initial: &Grid<u8>) -> (Vec<Grid<u8>>, Vec<usize>) {
    let mut history = Vec::new();
    let mut alive_counts = Vec::new();

    history.push(initial.clone());
    alive_counts.push(count_alive(initial));

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

        alive_counts.push(count_alive(&next));
        history.push(next);
    }

    (history, alive_counts)
}

/// Count how many '@' cells are in the given grid.
fn count_alive(grid: &Grid<u8>) -> usize {
    grid.points().filter(|&p| grid[p] == b'@').count()
}

/// Recompute the true neighbour counts for every cell in the grid.
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
