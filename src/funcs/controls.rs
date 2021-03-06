use quicksilver::{input::Keyboard, prelude::Key};

pub fn check_multiple(board: &Keyboard, to_check: &[Key]) -> bool {
    to_check
        .iter()
        .map(|v| board[*v])
        .map(|v| v.is_down())
        .any(|v| v)
}
