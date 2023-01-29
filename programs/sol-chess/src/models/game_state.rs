use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum GameState {
    Waiting,
    White,
    Black,
    WhiteWon,
    BlackWon,
    Draw,
}

impl GameState {
    pub fn get_current_player_turn(&self) -> Color {
        match self {
            GameState::White => Color::White,
            GameState::Black => Color::Black,
            _ => panic!("Invalid Game State"),
        }
    }

    pub fn is_white_turn(&self) -> bool {
        self == &GameState::White
    }

    pub fn is_black_turn(&self) -> bool {
        self == &GameState::Black
    }

    pub fn next_turn(&self) -> Self {
        match self {
            GameState::White => GameState::Black,
            GameState::Black => GameState::White,
            _ => panic!("Invalid Game State"),
        }
    }
}