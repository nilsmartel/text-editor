use colored::*;

#[derive(Default)]
struct State {
    cursor: (usize, usize),
    mode: Mode,
}

enum Mode {
    Insert,
    Normal,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Insert
    }
}

fn main() {
    let mut data = vec![String::from("")];
    let mut state = State::default();
}
