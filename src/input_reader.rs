use crate::static_types::{ButtonState, ButtonsStates, NumericalNotation, PossibleCoordinates};
use gilrs::{Button, Event, EventType};

// Converts buttons states to a numerical notation, neutral SOCD
pub fn calculate_position(buttons_state: &ButtonsStates) -> NumericalNotation {
    let vertical_coordinate: PossibleCoordinates;
    match (&buttons_state.up, &buttons_state.down) {
        (ButtonState::Released, ButtonState::Released) => {
            vertical_coordinate = PossibleCoordinates::Zero
        }
        (ButtonState::Pressed, ButtonState::Released) => {
            vertical_coordinate = PossibleCoordinates::One
        }
        (ButtonState::Released, ButtonState::Pressed) => {
            vertical_coordinate = PossibleCoordinates::MinusOne
        }
        (ButtonState::Pressed, ButtonState::Pressed) => {
            vertical_coordinate = PossibleCoordinates::Zero
        }
    };

    let horizontal_coordinate: PossibleCoordinates;
    match (&buttons_state.left, &buttons_state.right) {
        (ButtonState::Released, ButtonState::Released) => {
            horizontal_coordinate = PossibleCoordinates::Zero
        }
        (ButtonState::Pressed, ButtonState::Released) => {
            horizontal_coordinate = PossibleCoordinates::MinusOne
        }
        (ButtonState::Released, ButtonState::Pressed) => {
            horizontal_coordinate = PossibleCoordinates::One
        }
        (ButtonState::Pressed, ButtonState::Pressed) => {
            horizontal_coordinate = PossibleCoordinates::Zero
        }
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
pub fn parse_event(event: &Event, current_state: &mut ButtonsStates) {
    match event.event {
        EventType::ButtonPressed(button, _) => match button {
            Button::DPadUp => current_state.up = ButtonState::Pressed,
            Button::DPadDown => current_state.down = ButtonState::Pressed,
            Button::DPadLeft => current_state.left = ButtonState::Pressed,
            Button::DPadRight => current_state.right = ButtonState::Pressed,
            Button::North => current_state.attack_north = ButtonState::Pressed,
            Button::South => current_state.attack_south = ButtonState::Pressed,
            Button::East => current_state.attack_east = ButtonState::Pressed,
            Button::West => current_state.attack_west = ButtonState::Pressed,
            _ => {}
        },
        EventType::ButtonReleased(button, _) => match button {
            Button::DPadUp => current_state.up = ButtonState::Released,
            Button::DPadDown => current_state.down = ButtonState::Released,
            Button::DPadLeft => current_state.left = ButtonState::Released,
            Button::DPadRight => current_state.right = ButtonState::Released,
            Button::North => current_state.attack_north = ButtonState::Released,
            Button::South => current_state.attack_south = ButtonState::Released,
            Button::East => current_state.attack_east = ButtonState::Released,
            Button::West => current_state.attack_west = ButtonState::Released,
            _ => {}
        },
        _ => {}
    }
}
