mod render;
use render::render;

use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::TermRead;
use std::io::{stdin, stdout};

pub type SimpleString = Vec<char>;
pub type Data = Vec<SimpleString>;

#[derive(Default)]
pub struct State {
    pub cursor: (usize, usize),
    pub mode: Mode,
    pub command_history: Vec<Key>
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Insert,
    Normal,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Insert
    }
}

fn main() {
    let mut data = vec![SimpleString::default()];
    let mut state = State::default();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.events().filter_map(|c| c.ok()) {
        let evt = c;

        // here acceptors and actions would need to be coded into

        render(&mut stdout, &data, &state);
    }
}

type Mapping = (Acceptor, Action);

/// A test wether a pattern of user inputs matches    
/// todo: this may as well be a trait, for a regex might often match these patterns 
type Acceptor = Fn(Key, State) -> bool;

/// An action to perform, yielding new state and input
type Action = Fn(Key, State, Data) -> (State, Data);