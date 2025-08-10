use std::collections::HashMap;

#[derive(Debug)]
pub enum PossibleCoordinates {
    MinusOne = -1,
    Zero = 0,
    One = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumericalNotation {
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

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Moves {
    DP,
    QCF,
    QCB,
}

#[derive(Debug, Clone)]
pub struct ButtonsStates {
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
    pub attack_north: ButtonState,
    pub attack_south: ButtonState,
    pub attack_east: ButtonState,
    pub attack_west: ButtonState,
}

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub current_position: NumericalNotation,
    pub attack_pressed: bool,
    pub position_history: Vec<NumericalNotation>,
    pub close_requested: bool,
    pub last_successful_move: Vec<Moves>,
}

pub fn create_move_map() -> HashMap<Moves, Vec<NumericalNotation>> {
    let mut move_map = HashMap::new();
    move_map.insert(
        Moves::DP,
        vec![
            NumericalNotation::Six,
            NumericalNotation::Two,
            NumericalNotation::Three,
        ],
    );
    move_map
}
