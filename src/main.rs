use gilrs::{Button, Gilrs};
use std::thread;
use std::time::Instant;

mod input_reader;
mod rendering;
mod static_types;
use input_reader::{calculate_position, parse_event};
use rendering::render_grid;
use static_types::{ButtonState, CardinalDirectionStates, NumericalNotation};
use std::sync::mpsc;

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

    let mut start_time: Option<Instant> = None;
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
