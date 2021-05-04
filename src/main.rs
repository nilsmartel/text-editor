type Data = Vec<String>;

#[derive(Default)]
struct State {
    cursor: (usize, usize),
    mode: Mode,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Insert,
    Normal,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Insert
    }
}

fn render(display: &mut impl std::io::Write, data: &Data, state: &State) -> Result<(), std::io::Error> {
    // width and height of terminal
    let (width, height) = (120, 60);
    let info_bar_height = 2;

    let text_heigth = (height-info_bar_height).min(data.len());

    // print text
    for (line_number, content) in data.iter().take(text_heigth).enumerate() {
        std::write!(display, "{:4}", line_number)?;

        let cursor_on_line = line_number == state.cursor.1;

        // print chars
        for (col,c) in content.chars().take(width-4).enumerate() {
            if cursor_on_line {
                use colored::*;
                let s = String::from(c);
                if state.cursor.0 == col {
                    std::write!(display, "{}", s.on_black())?;
                } else {
                    std::write!(display, "{}", s.on_blue())?;
                }
            }
            std::write!(display, "{}", c)?;
        }

    }

    for _ in 0..(( height -info_bar_height)-text_heigth) {
        std::write!(display, "\n")?;
    }

    for _ in 0..width {
        std::write!(display, "_")?;
    }

    std::write!(display, "{:?}", state.mode)?;

    display.flush()
}

fn main() {
    let mut data = vec![String::from("")];
    let mut state = State::default();

    use termion::event::{Key, Event, MouseEvent};
    use termion::input::{TermRead};
    use std::io::stdin;
    let stdin = stdin();
    let mut stdout = std::io::stdout();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char(ch)) => match ch {
                '\n' if state.mode == Mode::Insert => {
                    data.insert(state.cursor.1, String::new());
                    state.cursor.1 += 1;
                }
                n if state.mode == Mode::Insert => {
                    data[state.cursor.1].insert(state.cursor.0, n);
                    state.cursor.0 += 1;
                }
                '\t' if state.mode == Mode::Insert => {
                    state.mode = Mode::Normal;
                }
                'h' if state.mode == Mode::Normal => {
                    state.cursor.0 -= 1;
                }
                'l' if state.mode == Mode::Normal => {
                    state.cursor.0 += 1;
                }
                'j' if state.mode == Mode::Normal => {
                    state.cursor.1 -= 1;
                }
                'k' if state.mode == Mode::Normal => {
                    state.cursor.1 += 1;
                }
                'i' if state.mode == Mode::Normal => {
                    state.mode = Mode::Insert;
                }
                'a' if state.mode == Mode::Normal => {
                    state.cursor.0 += 1;
                    state.mode = Mode::Insert;
                }
                'q' if state.mode == Mode::Normal => {
                    break
                }
                _ => {},
            },
            _ => {}
        }

        render(&mut stdout, &data, &state);
    }
}

