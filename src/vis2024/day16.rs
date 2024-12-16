use crate::util::grid::*;
// use crate::util::hash::*;
// use crate::util::point::*;
// use crate::aoc2024::day16::*;
// use pathfinding::prelude::astar;

use color_eyre::{eyre::Context, Result};
use std::time::Duration;
use ratatui::{
    crossterm::event::{self, Event, KeyCode}, style::{Stylize, Color}, widgets::Paragraph, DefaultTerminal, Frame
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn viz(grid: &Grid<u8>) -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let mut terminal = ratatui::init();
    terminal.clear().context("Failed to clear terminal")?;
    let app_result = run(terminal, grid).context("Failed to run application");
    ratatui::restore();
    app_result
}

pub fn run(mut terminal: DefaultTerminal, grid: &Grid<u8>) -> Result<()> {
    let start = grid.find(b'S').unwrap();
    let _end = grid.find(b'E').unwrap();

    let mut _to_visit = vec![start];
    // let mut visited = FastSet::default();

    // while let Some(current) = to_visit.pop() {
    //     if visited.contains(&current) {
    //         continue;
    //     }
    //     visited.insert(current);

    //     let reindeer = Reindeer { pos: start, dir: RIGHT };
    //     let (path, _) = astar(
    //         &reindeer,
    //         |r| get_successors(r, &grid),
    //         |r| r.pos.manhattan(current) as u32,
    //         |r| r.pos == current
    //     ).unwrap();

    //     println!("path: {:?}", path);

    //     for dir in [UP, DOWN, LEFT, RIGHT] {
    //         let next = current + dir;
    //         if grid[next] != b'#' && !visited.contains(&next) {
    //             to_visit.push(next);
    //         }
    //     }
    // }

    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)").fg(Color::White).bg(Color::Blue);
    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}