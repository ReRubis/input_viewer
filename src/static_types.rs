#[derive(Debug)]
pub enum PossibleCoordinates {
    MinusOne = -1,
    Zero = 0,
    One = 1,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct CardinalDirectionStates {
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
}
