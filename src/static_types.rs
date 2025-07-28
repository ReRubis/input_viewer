#[derive(Debug)]
pub enum PossibleCoordinates {
    MinusOne = -1,
    Zero = 0,
    One = 1,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum ButtonState {
    Pressed,
    Released,
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
}
