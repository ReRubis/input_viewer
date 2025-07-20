use color_eyre::owo_colors::Style;
use gilrs::ev::state;
use gilrs::{Gilrs, Event, EventType, Button};
use ratatui::symbols::border;
use std::time::{Duration, Instant};
use std::thread::{self, Thread};
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex, RwLock, mpsc::{self, Receiver, Sender}};
use ratatui::{
    crossterm::{
        event::{self as ratEvent, Event as RatEvent},
        terminal,
    },
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Block, List, ListItem, Paragraph, Sparkline, Widget},
    DefaultTerminal, Frame,
    CompletedFrame,
};
use std::error::Error;

#[derive(Debug)]
enum PossibleCoordinates {
    MinusOne = -1,
    Zero = 0,
    One = 1,
}

#[derive(Debug)]
enum NumericalNotation {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(Debug)]
enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
struct CardinalDirectionStates {
    up: ButtonState,
    down: ButtonState,
    left: ButtonState,
    right: ButtonState,
}

// Converts buttons states to a numerical notation, neutral SOCD
fn calculate_position(
    buttons_state: &CardinalDirectionStates,
) -> NumericalNotation {
    let vertical_coordinate :PossibleCoordinates ;
    match (&buttons_state.up, &buttons_state.down) {
        (ButtonState::Released, ButtonState::Released) => vertical_coordinate = PossibleCoordinates::Zero,
        (ButtonState::Pressed, ButtonState::Released) => vertical_coordinate = PossibleCoordinates::One,
        (ButtonState::Released, ButtonState::Pressed) => vertical_coordinate = PossibleCoordinates::MinusOne,
        (ButtonState::Pressed, ButtonState::Pressed) => vertical_coordinate = PossibleCoordinates::Zero,
    };

    let horizontal_coordinate :PossibleCoordinates ;
    match (&buttons_state.left, &buttons_state.right) {
        (ButtonState::Released, ButtonState::Released) => horizontal_coordinate = PossibleCoordinates::Zero,
        (ButtonState::Pressed, ButtonState::Released) => horizontal_coordinate = PossibleCoordinates::MinusOne,
        (ButtonState::Released, ButtonState::Pressed) => horizontal_coordinate = PossibleCoordinates::One,
        (ButtonState::Pressed, ButtonState::Pressed) => horizontal_coordinate = PossibleCoordinates::Zero,
    };

    match (horizontal_coordinate, vertical_coordinate) {
        (PossibleCoordinates::Zero, PossibleCoordinates::Zero) => NumericalNotation::Five,
        (PossibleCoordinates::One, PossibleCoordinates::Zero) => NumericalNotation::Six,
        (PossibleCoordinates::One, PossibleCoordinates::One) => NumericalNotation::Nine,
        (PossibleCoordinates::Zero, PossibleCoordinates::One) => NumericalNotation::Eight,
        (PossibleCoordinates::MinusOne, PossibleCoordinates::One) => NumericalNotation::Seven,
        (PossibleCoordinates::MinusOne, PossibleCoordinates::Zero) => NumericalNotation::Four,
        (PossibleCoordinates::MinusOne, PossibleCoordinates::MinusOne) => NumericalNotation::One,
        (PossibleCoordinates::Zero, PossibleCoordinates::MinusOne) => NumericalNotation::Two,
        (PossibleCoordinates::One, PossibleCoordinates::MinusOne) => NumericalNotation::Three,
    }
}


// Event parser to update the current state of the cardinal directions
fn parse_event(event: &Event, current_state: &mut CardinalDirectionStates) {
    match event.event {
        EventType::ButtonPressed(button, _) => {
            match button {
                Button::DPadUp => current_state.up = ButtonState::Pressed,
                Button::DPadDown => current_state.down = ButtonState::Pressed,
                Button::DPadLeft => current_state.left = ButtonState::Pressed,
                Button::DPadRight => current_state.right = ButtonState::Pressed,
                _ => {}
            }
        }
        EventType::ButtonReleased(button, _) => {
            match button {
                Button::DPadUp => current_state.up = ButtonState::Released,
                Button::DPadDown => current_state.down = ButtonState::Released,
                Button::DPadLeft => current_state.left = ButtonState::Released,
                Button::DPadRight => current_state.right = ButtonState::Released,
                _ => {}
            }
        }
        _ => {}
    }
}




// Fully controls the terminal
fn render_grid(
    render_rx: Receiver<NumericalNotation>,
) -> Result<(), String> {

    if let Err(e) = color_eyre::install(){
        eprintln!("Failed to install color_eyre: {}", e);
        return Err(e.to_string());
    };
    let mut terminal = ratatui::init();
    let mut current_position = NumericalNotation::Five;

    loop{
        let frame_start = Instant::now();
        
        if let Ok(new_position) = render_rx.try_recv() {
            current_position = new_position;
        }

        if let Ok(CompletedFrame) = terminal.draw(|f| 
            run_drawing(f, &current_position)
        ) {

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
    ]).split(border_area);

    // Create 3 columns for each row
    let top_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ]).split(rows[0]);

    let middle_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ]).split(rows[1]);

    let bottom_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ]).split(rows[2]);

    // Create blocks for each grid position
    let grid_positions = [
        (top_cols[0], "7"), (top_cols[1], "8"), (top_cols[2], "9"),
        (middle_cols[0], "4"), (middle_cols[1], "5"), (middle_cols[2], "6"),
        (bottom_cols[0], "1"), (bottom_cols[1], "2"), (bottom_cols[2], "3"),
    ];

    // Render each grid cell
    for (area, number) in grid_positions {
        let is_current = match (number, position) {
            ("1", NumericalNotation::One) | ("2", NumericalNotation::Two) | 
            ("3", NumericalNotation::Three) | ("4", NumericalNotation::Four) |
            ("5", NumericalNotation::Five) | ("6", NumericalNotation::Six) |
            ("7", NumericalNotation::Seven) | ("8", NumericalNotation::Eight) |
            ("9", NumericalNotation::Nine) => true,
            _ => false,
        };

        let block = if is_current {
            Block::bordered()
                .title(number)
                .style(ratatui::style::Style::default().bg(Color::Yellow).fg(Color::Black))
        } else {
            Block::bordered()
                .title(number)
        };

        frame.render_widget(block, area);
    }
}

fn main() {

    let (render_tx, render_rx) = mpsc::channel::<NumericalNotation>();

    let mut gilrs = Gilrs::new().unwrap();

    let mut current_state = CardinalDirectionStates {
        up: ButtonState::Released,
        down: ButtonState::Released,
        left: ButtonState::Released,
        right: ButtonState::Released,
    };

    let target_sequence = vec![
        Button::DPadRight,
        Button::DPadDown,
        Button::DPadRight,
        Button::RightThumb,
    ];

    let mut start_time: Option<Instant>= None;
    let mut current_step = 0;

    let render_handle = thread::spawn(move || render_grid(render_rx));
    let mut current_position = NumericalNotation::Five;
    loop {
        while let Some(event) = gilrs.next_event() {

            parse_event(&event, &mut current_state);
            current_position = calculate_position(&current_state);

            match render_tx.send(current_position) {
                Ok(()) => {
                    // Successfully sent the current position
                }
                Err(e) => {
                    eprintln!("Failed to send current position: {}", e);
                }
            }
        }
    }
}