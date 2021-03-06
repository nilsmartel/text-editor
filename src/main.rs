mod render;
use render::render;

use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub type SimpleString = Vec<char>;
pub type Data = Vec<SimpleString>;

#[derive(Default)]
pub struct State {
    pub cursor: (usize, usize),
    pub mode: Mode,
    pub command_history: Vec<Key>,
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

    render(&mut stdout, &data, &state);

    for c in stdin.events().filter_map(|c| c.ok()) {
        // here acceptors and actions would need to be coded into
        match c {
            Event::Key(Key::Esc) => {
                state.mode = Mode::Normal;
            }
            Event::Key(Key::Backspace) if state.mode == Mode::Insert => {
                if state.cursor.0 == 0 {
                    if state.cursor.1 == 0 {
                        continue;
                    }

                    let mut row = data.remove(state.cursor.1);
                    let baserow = data.get_mut(state.cursor.1 - 1).unwrap();
                    let len = baserow.len();

                    state.cursor.0 = len;
                    state.cursor.1 -= 1;

                    baserow.append(&mut row);
                    continue
                }

                data[state.cursor.1].remove(state.cursor.0 - 1);
                state.cursor.0 -= 1;
            },
            Event::Key(Key::Char(ch)) => match ch {
                'x' if state.mode == Mode::Normal => {
                    data[state.cursor.1].remove(state.cursor.0);
                }
                '\n' if state.mode == Mode::Insert => {
                    data.insert(state.cursor.1+1, Vec::new());
                    state.cursor.0 = 0;
                    state.cursor.1 += 1;
                }
                'o' if state.mode == Mode::Normal => {
                    data.insert(state.cursor.1 + 1, Vec::new());
                    state.cursor.0 = 0;
                    state.cursor.1 += 1;
                }
                n if state.mode == Mode::Insert => {
                    data[state.cursor.1].insert(state.cursor.0, n);
                    state.cursor.0 += 1;
                }
                'h' if state.mode == Mode::Normal => {
                    state.cursor.0 -= 1;
                }
                'l' if state.mode == Mode::Normal => {
                    state.cursor.0 += 1;
                }
                'k' if state.mode == Mode::Normal => {
                    state.cursor.1 -= 1;
                }
                'j' if state.mode == Mode::Normal => {
                    state.cursor.1 += 1;
                }
                'i' if state.mode == Mode::Normal => {
                    state.mode = Mode::Insert;
                }
                'a' if state.mode == Mode::Normal => {
                    state.cursor.0 += 1;
                    state.mode = Mode::Insert;
                }
                'q' if state.mode == Mode::Normal => break,
                _ => {}
            },
            _ => {}
        }

        render(&mut stdout, &data, &state);
    }

    println!("bye");
}

type Mapping = (Acceptor, Action);

/// A test wether a pattern of user inputs matches    
/// todo: this may as well be a trait, for a regex might often match these patterns
type Acceptor = Fn(Key, State) -> bool;

/// An action to perform, yielding new state and input
type Action = Fn(Key, State, Data) -> (State, Data);
