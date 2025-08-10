use gilrs::{Button, Gilrs};
use std::thread;
use std::time::Instant;

mod input_explainer;
mod input_reader;
mod rendering;
mod static_types;
use input_explainer::check_move_sequence;
use input_reader::{calculate_position, is_attack_pressed, parse_event};
use rendering::render_grid;
use static_types::{ButtonState, ButtonsStates, GlobalState, NumericalNotation, create_move_map};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (render_tx, render_rx) = mpsc::channel::<GlobalState>();

    let mut gilrs = Gilrs::new().unwrap();

    let move_map = create_move_map();

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

    let render_handle = thread::spawn(move || render_grid(render_rx));
    let mut current_position = NumericalNotation::Five;

    let mut data_state = GlobalState {
        current_position: current_position,
        attack_pressed: false,
        position_history: Vec::new(),
        close_requested: false,
        last_successful_move: vec![],
    };
    loop {
        let frame_start = Instant::now();

        while let Some(event) = gilrs.next_event() {
            parse_event(&event, &mut current_state);
        }

        data_state.current_position = calculate_position(&current_state);
        data_state
            .position_history
            .push(data_state.current_position.clone());
        if data_state.position_history.len() > 30 {
            data_state.position_history.remove(0);
        }

        data_state.attack_pressed = is_attack_pressed(&current_state);
        if data_state.attack_pressed {
            if let Some(last_successful_move) =
                check_move_sequence(&data_state.position_history, &move_map)
            {
                data_state.last_successful_move.push(last_successful_move);
            }
        };
        match render_tx.send(data_state.clone()) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to send current position: {}", e);
            }
        }

        let frame_time = frame_start.elapsed();
        let target_frame_time = Duration::from_nanos(16_666_667);
        if frame_time < target_frame_time {
            thread::sleep(target_frame_time - frame_time);
        }
    }
}
