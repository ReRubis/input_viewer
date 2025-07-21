use crate::static_types::NumericalNotation;
use ratatui::{
    Frame,
    crossterm::event::{self as ratEvent, Event as RatEvent},
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Block, Paragraph},
};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};

// Fully controls the terminal
pub fn render_grid(render_rx: Receiver<NumericalNotation>) -> Result<(), String> {
    if let Err(e) = color_eyre::install() {
        eprintln!("Failed to install color_eyre: {}", e);
        return Err(e.to_string());
    };
    let mut terminal = ratatui::init();
    let mut current_position = NumericalNotation::Five;

    loop {
        let frame_start = Instant::now();

        if let Ok(new_position) = render_rx.try_recv() {
            current_position = new_position;
        }

        if let Ok(CompletedFrame) = terminal.draw(|f| run_drawing(f, &current_position)) {
        } else {
            eprintln!("Failed to draw frame");
            return Err("Failed to draw frame".to_string());
        }

        if ratEvent::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(RatEvent::Key(key)) = ratEvent::read() {
                match key.code {
                    ratEvent::KeyCode::Esc => {
                        println!("Exiting...");
                        break;
                    }
                    _ => {}
                }
            }
        }

        let frame_time = frame_start.elapsed();
        let target_frame_time = Duration::from_millis(16); // ~60 FPS
        if frame_time < target_frame_time {
            thread::sleep(target_frame_time - frame_time);
        }
    }

    ratatui::restore();
    Ok(())
}

fn run_drawing(frame: &mut Frame, position: &NumericalNotation) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let position_text = format!("Current Position: {:?}", position);
    let paragraph = Paragraph::new(position_text);
    frame.render_widget(paragraph, border_area);

    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    // Create 3 horizontal rows
    let rows = Layout::vertical([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(border_area);

    // Create 3 columns for each row
    let top_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(rows[0]);

    let middle_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(rows[1]);

    let bottom_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(rows[2]);

    // Create blocks for each grid position
    let grid_positions = [
        (top_cols[0], "7"),
        (top_cols[1], "8"),
        (top_cols[2], "9"),
        (middle_cols[0], "4"),
        (middle_cols[1], "5"),
        (middle_cols[2], "6"),
        (bottom_cols[0], "1"),
        (bottom_cols[1], "2"),
        (bottom_cols[2], "3"),
    ];

    // Render each grid cell
    for (area, number) in grid_positions {
        let is_current = match (number, position) {
            ("1", NumericalNotation::One)
            | ("2", NumericalNotation::Two)
            | ("3", NumericalNotation::Three)
            | ("4", NumericalNotation::Four)
            | ("5", NumericalNotation::Five)
            | ("6", NumericalNotation::Six)
            | ("7", NumericalNotation::Seven)
            | ("8", NumericalNotation::Eight)
            | ("9", NumericalNotation::Nine) => true,
            _ => false,
        };

        let block = if is_current {
            Block::bordered().title(number).style(
                ratatui::style::Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black),
            )
        } else {
            Block::bordered().title(number)
        };

        frame.render_widget(block, area);
    }
}
