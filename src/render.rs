use crate::{State, Data};
use termion::clear;

pub fn render(display: &mut impl std::io::Write, data: &Data, state: &State) -> Result<(), std::io::Error> {
    // clear the entire screen
    write!(display, "{}", clear::All)?;

    // width and height of terminal
    let (width, height) = (120, 48);
    let info_bar_height = 2;

    let text_heigth = (height-info_bar_height).min(data.len());

    // print text
    for (line_number, content) in data.iter().take(text_heigth).enumerate() {
        std::write!(display, "\r{:3} ", line_number)?;

        let cursor_on_line = line_number == state.cursor.1;

        // print chars
        for (col,c) in content.iter().take(width-4).enumerate() {
            if cursor_on_line {
                use colored::*;
                let s = String::from(*c);
                if state.cursor.0 == col {
                    // TODO remove colored deps
                    std::write!(display, "{}", s.on_black())?;
                } else {
                    std::write!(display, "{}", s.on_blue())?;
                }
                continue;
            }
            std::write!(display, "{}", c)?;
        }

        std::write!(display, "\n")?;

    }

    for _ in 0..(( height - info_bar_height)-text_heigth) {
        std::write!(display, "\n\r")?;
    }

    for _ in 0..width {
        std::write!(display, "_")?;
    }

    std::write!(display, "\n\r{:?}", state.mode)?;
    // std::write!(display, "\n\r")?;
    display.flush()
}

