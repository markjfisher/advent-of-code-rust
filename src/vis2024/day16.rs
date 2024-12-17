use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use crate::aoc2024::day16::*;
use pathfinding::prelude::astar;

use color_eyre::{eyre::Context, Result};
use std::time::Duration;
use ratatui::{
    crossterm::event::{self, Event, KeyCode}, 
    style::Color, 
    widgets::{Paragraph, Block, Borders},
    text::{Line, Span},
    style::Style,
    DefaultTerminal, 
    Frame
};

pub struct App {
    grid: Grid<u8>,
    visited: FastSet<Point>,
    best_path: Vec<Point>,
    to_visit: Vec<Point>,
    end_point: Point,
    end_point_reached: bool,
    best_distance: i32,
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
    // capture the state
    pub fn new(grid: Grid<u8>) -> Self {
        let start = grid.find(b'S').unwrap();
        let end = Point::new(grid.width - 2, grid.height - 2); // Assuming end is near top-right
        Self {
            grid,
            visited: FastSet::default(),
            best_path: Vec::new(),
            to_visit: vec![start],
            end_point: end,
            end_point_reached: false,
            best_distance: i32::MAX,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let frame_duration = Duration::from_millis(250);
        let mut last_frame = std::time::Instant::now();

        loop {
            // Only draw if enough time has passed since last frame
            let now = std::time::Instant::now();
            if now.duration_since(last_frame) >= frame_duration {
                terminal.draw(|frame| self.draw(frame))?;
                last_frame = now;

                // Wait for space to continue
                loop {
                    if event::poll(Duration::from_millis(10)).context("event poll failed")? {
                        if let Event::Key(key) = event::read().context("event read failed")? {
                            match key.code {
                                KeyCode::Char('q') => return Ok(()),
                                KeyCode::Char(' ') => break,
                                _ => {}
                            }
                        }
                    }
                }
            }

            // Check if visualization is complete
            if self.to_visit.is_empty() {
                // Optional: wait for user to quit after completion
                while !self.should_quit()? {}
                break;
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, frame: &mut Frame) {
        let mut all_lines = Vec::new();
        
        // First add the grid visualization
        for y in 0..self.grid.height {
            let mut line_spans = Vec::new();
            for x in 0..self.grid.width {
                let point = Point::new(x, y);
                let char = match self.grid[point] {
                    b'#' => Span::styled("█", Style::default().fg(Color::White)),
                    b'S' => Span::styled("S", Style::default().fg(Color::Yellow)),
                    b'E' => Span::styled("E", Style::default().fg(Color::Yellow)),
                    _ => {
                        if self.best_path.contains(&point) {
                            Span::styled(" ", Style::default().bg(Color::Blue))
                        } else if self.visited.contains(&point) {
                            Span::styled(" ", Style::default().bg(Color::Green))
                        } else {
                            Span::styled(" ", Style::default().fg(Color::DarkGray))
                        }
                    }
                };
                line_spans.push(char);
            }
            all_lines.push(Line::from(line_spans));
        }

        // Add empty line as separator
        all_lines.push(Line::from(""));

        // Add status information
        all_lines.push(Line::from(vec![
            Span::raw(format!("Visited Points: {}", self.visited.len())),
            Span::raw(" | "),
            Span::raw(format!("Best Distance: {}", self.best_distance)),
            Span::raw(" | "),
            Span::styled(
                format!("End Point Reached: {}", self.end_point_reached),
                Style::default().fg(if self.end_point_reached { Color::Green } else { Color::Red })
            ),
        ]));

        // Add best path visualization
        if !self.best_path.is_empty() {
            let path_str = self.best_path.iter()
                .map(|p| format!("({},{})", p.x, p.y))
                .collect::<Vec<_>>()
                .join(" → ");
            all_lines.push(Line::from(vec![
                Span::raw("Best Path: "),
                Span::styled(path_str, Style::default().fg(Color::Blue))
            ]));
            all_lines.push(Line::from(vec![
                Span::raw("Path Length: "),
                Span::styled(
                    format!("{}", self.best_path.len() - 1),
                    Style::default().fg(Color::Blue)
                )
            ]));
        }

        let paragraph = Paragraph::new(all_lines)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(paragraph, frame.area());

        // Process next step in visualization
        self.step();
    }
    
    fn should_quit(&self) -> Result<bool> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                return Ok(KeyCode::Char('q') == key.code);
            }
        }
        Ok(false)
    }

    fn step(&mut self) {
        if let Some(current) = self.to_visit.pop() {
            if self.visited.contains(&current) {
                return;
            }
            self.visited.insert(current);

            // Check if we've reached the end point
            if current == self.end_point {
                self.end_point_reached = true;
            }

            // Check if this point is closer to the end
            let current_distance = current.manhattan(self.end_point);
            if current_distance < self.best_distance {
                self.best_distance = current_distance;
                
                // Update best path only when we find a closer point
                let start = self.grid.find(b'S').unwrap();
                let reindeer = Reindeer { pos: start, dir: RIGHT };
                if let Some((path, _)) = astar(
                    &reindeer,
                    |r| get_successors(r, &self.grid),
                    |r| r.pos.manhattan(current) as u32,
                    |r| r.pos == current
                ) {
                    self.best_path = path.into_iter().map(|r| r.pos).collect();
                }
            }

            // Continue flood fill even after reaching end
            for dir in [UP, DOWN, LEFT, RIGHT] {
                let next = current + dir;
                if self.grid.contains(next) && self.grid[next] != b'#' && !self.visited.contains(&next) {
                    self.to_visit.push(next);
                }
            }
        }
    }
}
