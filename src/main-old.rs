mod utils;

use crossterm::{
    cursor::{MoveLeft, MoveTo, MoveToColumn, SetCursorStyle},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use utils::{get_coords_for_centered_text, print_centered, split_string};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        execute!(
            stdout(),
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::Black),
            Print("\nPress Escape to exit Typirst...\n")
        )
        .expect("Could not print exit message");
        // Wait for the user to press Enter before exiting the Alternate Screen and the program
        loop {
            if event::poll(std::time::Duration::from_millis(500))
                .expect("Could not poll for events")
            {
                if let Event::Key(key_event) = event::read().expect("Could not read event") {
                    match key_event.code {
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        terminal::disable_raw_mode().expect("Could not disable raw mode");
        execute!(stdout(), LeaveAlternateScreen).expect("Could not leave alternate screen");
    }
}

fn update_scores(wpm: f64, errors: usize, position: u16, current_line_idx: u16) {
    // update_wpm(wpm);
    // Restore the cursor position to the typing area
    // execute!(stdout(), MoveTo(position, current_line_idx + 1))
    // .expect("Could not restore cursor position");
}

fn update_wpm(wpm: f64) {
    let term_size = terminal::size().expect("Could not get terminal size");

    // Update the WPM in top right corner of the screen
    execute!(
        stdout(),
        MoveToColumn(term_size.0 - 10),
        Print("WPM: "),
        Print(wpm.to_string())
    )
    .expect("Could not update WPM");
}

fn print_lines_to_center(lines: &Vec<String>) {
    // Print the lines
    for (idx, line) in lines.iter().enumerate() {
        print_centered(line, idx as isize - lines.len() as isize / 2)
            .expect("Could not print centered line");
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    let _clean_up = CleanUp;

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let lines = split_string("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus vitae ante in ligula aliquet commodo eget sed felis. Ut semper metus eu gravida rhoncus. Curabitur fermentum vestibulum ultrices. Nullam vel pretium dolor. In sed tellus convallis, tincidunt ipsum id, pulvinar nisl.

Morbi rutrum placerat elit, et vehicula lectus commodo suscipit. Aliquam convallis pharetra mollis. Duis laoreet posuere mi et vulputate. Sed tincidunt interdum dolor et ornare. Aliquam ac ultrices ex. Aliquam vitae ullamcorper sem. Sed hendrerit pretium tempus. Nunc efficitur congue semper. Ut sapien nunc, varius at vulputate vitae, placerat ut sem. Nulla quis sapien facilisis, maximus velit at, ultricies justo. Praesent dapibus nibh id magna posuere blandit.", 20);

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        MoveTo(0, 0),
        SetCursorStyle::SteadyBar,
        // SetBackgroundColor(Color::AnsiValue(24)),
    )?;

    print_lines_to_center(&lines);

    // Move the cursor to the start of the first line for input
    let (start_col, start_row) =
        get_coords_for_centered_text(&lines[0], 0 - lines.len() as isize / 2);

    execute!(stdout, MoveTo(start_col, start_row))?;

    let mut typed_chars = Vec::new();
    let mut position: usize = 0;
    let mut current_line_idx: usize = 0;
    let start = std::time::Instant::now();
    let mut wpm: f64 = 0.0;
    let mut errors: usize = 0;

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        let correct = lines[current_line_idx].chars().nth(position).unwrap() == c;

                        let elapsed = start.elapsed().as_secs_f64() / 60.0;
                        wpm = (typed_chars.len() as f64 / 5.0) / elapsed;
                        if !correct {
                            errors += 1;
                        }

                        // let (start_col, start_row) = get_coords_for_centered_text(
                        //     &lines[current_line_idx],
                        //     current_line_idx as isize - lines.len() as isize / 2,
                        // );

                        execute!(
                            stdout,
                            // MoveTo(start_col + position as u16, start_row),
                            SetForegroundColor(if correct { Color::Green } else { Color::Red }),
                            SetBackgroundColor(if c == ' ' && !correct {
                                Color::Red
                            } else {
                                Color::Reset
                            }),
                            Print(c),
                        )?;
                        stdout.flush()?;
                        typed_chars.push(c);

                        position += 1;
                        // If we've reached the end of the line, move to the next line
                        if position == lines[current_line_idx].len() {
                            if current_line_idx == lines.len() - 1 {
                                break;
                            }
                            position = 0;
                            current_line_idx += 1;
                            let (start_col, start_row) = get_coords_for_centered_text(
                                &lines[current_line_idx],
                                current_line_idx as isize - lines.len() as isize / 2,
                            );
                            execute!(stdout, MoveTo(start_col, start_row),)?;
                        }
                        update_scores(wpm, errors, position as u16, current_line_idx as u16);
                    }
                    KeyCode::Backspace => {
                        // Handle backspace, removing the last character typed

                        // Handle if we're at the beginning of the first line
                        if position == 0 && current_line_idx == 0 {
                            continue;
                        }

                        typed_chars.pop();
                        if position > 0 {
                            position -= 1;
                        } else {
                            current_line_idx -= 1;
                            position = lines[current_line_idx].len() - 1;
                            let (start_col, start_row) = get_coords_for_centered_text(
                                &lines[current_line_idx],
                                current_line_idx as isize - lines.len() as isize / 2,
                            );
                            execute!(
                                stdout,
                                MoveTo(start_col + position as u16 + 1, start_row as u16),
                            )?;
                        };
                        // println!("Position: {}, Current Line: {}", position, current_line_idx);

                        execute!(
                            stdout,
                            ResetColor,
                            SetForegroundColor(Color::Reset),
                            MoveLeft(1),
                            Print(lines[current_line_idx].chars().nth(position).unwrap()),
                            MoveLeft(1),
                        )?;
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
