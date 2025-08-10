use crate::static_types::{
    ButtonState, ButtonsStates, GlobalState, Moves, NumericalNotation, create_move_map,
};
use std::collections::HashMap;

pub fn is_valid_sequence(
    position_history: &[NumericalNotation],
    move_sequence: &[NumericalNotation],
) -> bool {
    if move_sequence.is_empty() {
        return true;
    }

    if position_history.is_empty() {
        return false;
    }

    let mut history_idx = position_history.len() - 1;
    let mut sequence_idx = move_sequence.len() - 1;
    let mut first_occurrences = Vec::new();

    loop {
        if position_history[history_idx] == move_sequence[sequence_idx] {
            first_occurrences.push(history_idx);

            if sequence_idx == 0 {
                first_occurrences.reverse();
                for i in 1..first_occurrences.len() {
                    let distance = first_occurrences[i] - first_occurrences[i - 1];
                    if distance > 7 {
                        return false;
                    }
                }
                return true;
            }
            sequence_idx -= 1;
        }

        if history_idx == 0 {
            return false;
        }

        // Move backwards in history
        history_idx -= 1;
    }
}

pub fn check_move_sequence(
    position_history: &[NumericalNotation],
    move_map: &HashMap<Moves, Vec<NumericalNotation>>,
) -> Option<Moves> {
    for (move_name, move_sequence) in move_map {
        if move_sequence.last() == position_history.last() {
            if is_valid_sequence(position_history, move_sequence) {
                return Some(*move_name);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sequence() {
        let position_history = vec![
            NumericalNotation::Six,
            NumericalNotation::Two,
            NumericalNotation::Three,
        ];
        let move_sequence = vec![
            NumericalNotation::Six,
            NumericalNotation::Two,
            NumericalNotation::Three,
        ];

        assert!(is_valid_sequence(&position_history, &move_sequence));

        let position_history = vec![
            NumericalNotation::Six,
            NumericalNotation::Two,
            NumericalNotation::Two,
            NumericalNotation::Two,
            NumericalNotation::Two,
            NumericalNotation::Two,
            NumericalNotation::Two,
            NumericalNotation::Two,
            NumericalNotation::Three,
        ];
        assert!(is_valid_sequence(&position_history, &move_sequence));
    }

    #[test]
    fn test_is_invalid_sequence() {
        let position_history = vec![
            NumericalNotation::One,
            NumericalNotation::Two,
            NumericalNotation::Three,
            NumericalNotation::Four,
            NumericalNotation::Five,
            NumericalNotation::Six,
            NumericalNotation::Seven,
        ];
        let move_sequence = vec![
            NumericalNotation::Six,
            NumericalNotation::Two,
            NumericalNotation::Three,
        ];

        assert!(!is_valid_sequence(&position_history, &move_sequence));
    }
}
