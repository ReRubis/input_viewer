use gilrs::{Button, Gilrs};
use std::thread;
use std::time::Instant;

mod input_reader;
mod rendering;
mod static_types;
use input_reader::{calculate_position, is_attack_pressed, parse_event};
use rendering::render_grid;
use static_types::{ButtonState, ButtonsStates, GlobalState, NumericalNotation};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (render_tx, render_rx) = mpsc::channel::<GlobalState>();

    let mut gilrs = Gilrs::new().unwrap();

    let mut current_state = ButtonsStates {
        up: ButtonState::Released,
        down: ButtonState::Released,
        left: ButtonState::Released,
        right: ButtonState::Released,
        attack_north: ButtonState::Released,
        attack_south: ButtonState::Released,
        attack_east: ButtonState::Released,
        attack_west: ButtonState::Released,
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

    let mut data_state = GlobalState {
        current_position: current_position,
        attack_pressed: false,
        position_history: Vec::new(),
        close_requested: false,
    };
    loop {
        let frame_start = Instant::now();

        // Process all pending events
        while let Some(event) = gilrs.next_event() {
            parse_event(&event, &mut current_state);
        }

        // Update state once per frame
        data_state.current_position = calculate_position(&current_state);
        data_state
            .position_history
            .push(data_state.current_position.clone());
        if data_state.position_history.len() > 16 {
            data_state.position_history.remove(0);
        }

        data_state.attack_pressed = is_attack_pressed(&current_state);

        match render_tx.send(data_state.clone()) {
            Ok(()) => {
                // Successfully sent the current position
            }
            Err(e) => {
                eprintln!("Failed to send current position: {}", e);
            }
        }

        // Sleep to maintain 60 FPS
        let frame_time = frame_start.elapsed();
        let target_frame_time = Duration::from_nanos(16_666_667); // 1/60 second
        if frame_time < target_frame_time {
            thread::sleep(target_frame_time - frame_time);
        }
    }
}
