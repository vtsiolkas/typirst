// use crossterm::{cursor::MoveTo, execute, style::Print, terminal};
// use std::io::stdout;

// pub fn get_coords_for_centered_text(text: &str, y_offset: isize) -> (u16, u16) {
//     let (cols, rows) = terminal::size().unwrap();
//     let text_len = text.chars().count();

//     // Calculate the starting position
//     let start_col = (cols / 2).saturating_sub(text_len as u16 / 2);
//     let mut start_row = rows / 2;
//     start_row = if y_offset < 0 {
//         start_row.saturating_sub(y_offset.abs() as u16)
//     } else {
//         start_row + y_offset as u16
//     };

//     (start_col, start_row)
// }

// pub fn print_centered<S: AsRef<str>>(text: S, y_offset: isize) -> Result<(), std::io::Error> {
//     let (start_col, start_row) = get_coords_for_centered_text(text.as_ref(), y_offset);

//     // Move the cursor to the calculated position and print the text
//     execute!(stdout(), MoveTo(start_col, start_row), Print(text.as_ref()))?;

//     Ok(())
// }
