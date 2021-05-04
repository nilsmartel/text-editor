mod render;
use render::render;

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
    let mut data = vec![String::from("")];
    let mut state = State::default();

    let stdin = stdin();
    let mut stdout = std::io::stdout();
    for c in stdin.events() {
        if c.is_err() {
            continue
        }

        let evt = c.unwrap();
        eprintln!("{:#?}", evt);
        // render(&mut stdout, &data, &state);
    }
}


/// A test wether a pattern of user inputs matches    
type Acceptor = Fn(Key, State, Data) -> bool;

/// An action to perform, yielding new state and input
type Action = Fn(Key, State, Data) -> (State, Data);
