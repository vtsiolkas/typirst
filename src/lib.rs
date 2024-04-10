mod text_generator;
pub mod tui;
mod ui;
mod utils;

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::seq::SliceRandom;
use ratatui::style::palette::tailwind::{EMERALD, RED, SLATE};
use ratatui::style::Color;
use std::collections::HashMap;
use std::time::Instant;
use text_generator::{Character, TextGenerator};
use ui::ui;

#[derive(Debug)]
pub struct App {
    characters: Vec<Vec<Character>>,
    stats: HashMap<char, TypingStat>,
    cur_line: usize,
    position: usize,
    typed_chars: usize,
    errors: usize,
    exit: bool,
    start_time: Instant,
    last_action_time: Instant,
    text_generator: TextGenerator,
}

#[derive(Debug, Default)]
struct TypingStat {
    ms_average: usize,
    typed: usize,
    errors: usize,
    acc_speed: f64, // Typing speed divided by accuracy, large is worse
}
impl PartialEq for TypingStat {
    fn eq(&self, other: &Self) -> bool {
        self.acc_speed == other.acc_speed
    }
}

impl Eq for TypingStat {}
impl Ord for TypingStat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.acc_speed.partial_cmp(&other.acc_speed).unwrap()
    }
}
impl PartialOrd for TypingStat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Colors {
    untyped: Color,
    correct: Color,
    incorrect: Color,
}

impl App {
    pub fn new() -> Self {
        Self {
            characters: vec![],
            stats: HashMap::new(),
            cur_line: 0,
            position: 0,
            typed_chars: 0,
            errors: 0,
            exit: false,
            start_time: Instant::now(),
            last_action_time: Instant::now(),
            text_generator: TextGenerator::new(),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.text_generator
            .load_snippets()
            .wrap_err("Loading snippets failed.")?;
        self.text_generator.calculate_character_weights();

        // Generate some initial snippets
        self.add_snippet();

        self.start_time = std::time::Instant::now();
        while !self.exit {
            terminal.draw(|frame| ui(frame, &self))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn add_snippet(&mut self) {
        let mut stats: Vec<_> = self.stats.iter().collect();
        stats.sort_by(|a, b| b.1.cmp(a.1));
        let top_5: Vec<_> = stats.into_iter().take(5).collect();

        let random_worst_char = top_5
            .choose(&mut rand::thread_rng())
            .map(|(c, _)| *c)
            .unwrap_or(&'j');

        let mut characters = self
            .text_generator
            .generate_characters(*random_worst_char, 40);
        self.characters.append(&mut characters);
    }

    fn check_character(&mut self, c: char) {
        self.typed_chars += 1;
        let errors = self.characters[self.cur_line][self.position].set_typed(c);
        self.errors = errors;
        self.position += 1;

        // update stats
        self.stats
            .entry(c)
            .and_modify(|stat| {
                stat.ms_average = (stat.ms_average * stat.typed
                    + self.last_action_time.elapsed().as_millis() as usize)
                    / stat.typed;
                stat.typed += 1;
                stat.errors += errors;
                stat.acc_speed = stat.ms_average as f64 / (stat.errors as f64 / stat.typed as f64);
            })
            .or_insert(TypingStat {
                ms_average: self.start_time.elapsed().as_millis() as usize,
                typed: 1,
                errors: errors,
                acc_speed: 0.0,
            });

        self.last_action_time = Instant::now();

        if self.position == self.characters[self.cur_line].len() {
            // Switch to next line or exit if we're at the end
            if self.cur_line == self.characters.len() - 1 {
                self.exit();
            }
            self.add_snippet();

            self.position = 0;
            self.cur_line += 1;
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char(c) => {
                self.check_character(c);
            }
            KeyCode::Enter => {
                self.check_character('\n');
            }
            KeyCode::Backspace => {
                // Handle if we're at the beginning of the first line
                if self.position == 0 && self.cur_line == 0 {
                    return Ok(());
                }

                if self.position > 0 {
                    self.position -= 1;
                } else {
                    self.cur_line -= 1;
                    self.position = self.characters[self.cur_line].len() - 1;
                }

                self.characters[self.cur_line][self.position].reset();

                self.last_action_time = Instant::now();
            }

            KeyCode::Esc => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn get_colors(&self, line_idx: usize) -> Colors {
        if line_idx == self.cur_line {
            Colors {
                untyped: SLATE.c50,
                correct: EMERALD.c400,
                incorrect: RED.c400,
            }
        } else if (line_idx as isize - self.cur_line as isize).abs() <= 1 {
            Colors {
                untyped: SLATE.c400,
                correct: EMERALD.c700,
                incorrect: RED.c800,
            }
        } else {
            Colors {
                untyped: SLATE.c500,
                correct: EMERALD.c800,
                incorrect: RED.c900,
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
