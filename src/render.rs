use crate::{State, Data};

pub fn render(display: &mut impl std::io::Write, data: &Data, state: &State) -> Result<(), std::io::Error> {
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

