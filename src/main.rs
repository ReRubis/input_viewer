use gilrs::ev::state;
use gilrs::{Gilrs, Event, EventType, Button};
use std::time::{Duration, Instant};
use std::thread::{self, Thread};
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex, RwLock};


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


fn render_positions(
    state: &CardinalDirectionStates,
) {
    let position = calculate_position(state);
    println!("Current position: {:?}", position);
}


fn main() {
    let thread_pool = ThreadPool::new(2); 

    let mut gilrs = Gilrs::new().unwrap();

    let mut current_state = Arc::new(RwLock::new(CardinalDirectionStates {
        up: ButtonState::Released,
        down: ButtonState::Released,
        left: ButtonState::Released,
        right: ButtonState::Released,
    }));

    let target_sequence = vec![
        Button::DPadRight,
        Button::DPadDown,
        Button::DPadRight,
        Button::RightThumb,
    ];

    let mut start_time: Option<Instant>= None;
    let mut current_step = 0;
    let mut current_position: NumericalNotation = NumericalNotation::Five;

    loop {
        while let Some(event) = gilrs.next_event() {

            let mut state = current_state.write().unwrap();
            parse_event(&event, &mut state);
            
            let state_clone = Arc::clone(&current_state);
            thread_pool.execute(move || {
                let state = state_clone.read().unwrap();
                render_positions(&state);
            });
            

        }
    }
}