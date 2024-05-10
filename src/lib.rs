mod options;
mod statistics;
mod text_generator;
mod timer;
pub mod tui;
mod ui;
mod utils;

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use options::{CyclicOption, Highlight, TextDifficulty};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use statistics::{load_typing_stats, save_typing_stats};
use std::collections::HashMap;
use text_generator::{Character, TextGenerator};
use timer::Timer;
use ui::ui;

#[derive(Debug)]
pub struct App {
    characters: Vec<Vec<Character>>,
    stats: HashMap<char, TypingStat>,
    cur_line: usize,
    position: usize,
    typed_chars: usize,
    errors: usize,
    pause: bool,
    quit: bool,
    timer: Timer,
    text_generator: TextGenerator,
    difficulty: CyclicOption<TextDifficulty>,
    highlight: CyclicOption<Highlight>,
    debug: bool,
    debug_text: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TypingStat {
    ms_average: Option<usize>,
    typed: Option<usize>,
    errors: Option<usize>,
    acc_speed: Option<f64>, // Typing speed divided by accuracy, large is worse
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

impl App {
    pub fn new() -> Self {
        Self {
            characters: vec![],
            stats: HashMap::new(),
            cur_line: 0,
            position: 0,
            typed_chars: 0,
            errors: 0,
            pause: false,
            quit: false,
            timer: Timer::new(),
            difficulty: CyclicOption::new(
                vec![
                    TextDifficulty::Lowercase,
                    TextDifficulty::Numbers,
                    TextDifficulty::Uppercase,
                    TextDifficulty::Symbols,
                ],
                'c',
            ),
            highlight: CyclicOption::new(
                vec![
                    Highlight::Nothing,
                    Highlight::Character,
                    Highlight::Word,
                    Highlight::NextWord,
                    Highlight::TwoWords,
                ],
                'h',
            ),
            text_generator: TextGenerator::new(TextDifficulty::Lowercase),
            debug: true,
            debug_text: String::new(),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.text_generator
            .load_snippets()
            .wrap_err("Loading snippets failed.")?;
        self.text_generator.calculate_character_weights();

        self.stats = load_typing_stats().unwrap_or_else(|_| HashMap::new());

        // Generate some initial snippets
        self.add_snippet();

        while !self.quit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events().wrap_err("handle events failed")?;
        }

        save_typing_stats(&self.stats).unwrap();
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
            .generate_characters(*random_worst_char, 50);
        self.characters.append(&mut characters);
    }

    fn check_character(&mut self, c: char) {
        if !self.timer.running {
            self.timer.start();
        }
        self.typed_chars += 1;
        let errors = self.characters[self.cur_line][self.position].set_typed(c);
        self.errors = errors;
        self.position += 1;

        // update stats
        self.stats
            .entry(c)
            .and_modify(|stat| {
                stat.ms_average = Some(
                    (stat.ms_average.unwrap() * stat.typed.unwrap()
                        + self.timer.elapsed_last_action().as_millis() as usize)
                        / stat.typed.unwrap(),
                );
                stat.typed = Some(stat.typed.unwrap() + 1);
                stat.errors = Some(stat.errors.unwrap() + errors);
                let mut acc = stat.typed.unwrap() as f64
                    - stat.errors.unwrap() as f64 / stat.typed.unwrap() as f64;
                if acc == 0.0 {
                    acc = 1.0;
                }
                stat.acc_speed = Some(stat.ms_average.unwrap() as f64 / acc);
            })
            .or_insert(TypingStat {
                ms_average: Some(self.timer.elapsed_last_action().as_millis() as usize),
                typed: Some(1),
                errors: Some(errors),
                acc_speed: Some(
                    (self.timer.elapsed_last_action().as_millis()
                        * (if errors > 0 { errors as u128 } else { 1 })) as f64,
                ),
            });

        self.timer.reset_last_action();

        if self.position == self.characters[self.cur_line].len() {
            self.add_snippet();

            self.position = 0;
            self.cur_line += 1;
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if self.pause {
            match key_event.code {
                KeyCode::Esc => self.unpause(),
                KeyCode::Char('q') => self.quit(),
                KeyCode::Char('r') => self.reset(),
                KeyCode::Char('c') => {
                    self.difficulty.next();
                    self.reset();
                }
                KeyCode::Char('h') => {
                    self.highlight.next();
                }
                _ => {}
            }
            return Ok(());
        } else {
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

                    self.timer.reset_last_action();
                }

                KeyCode::Esc => self.pause(),
                _ => {}
            }
        }
        Ok(())
    }

    fn pause(&mut self) {
        self.pause = true;
        self.timer.pause();
    }

    fn unpause(&mut self) {
        self.pause = false;
        self.timer.start();
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn reset(&mut self) {
        self.characters = vec![];
        self.cur_line = 0;
        self.position = 0;
        self.typed_chars = 0;
        self.errors = 0;
        self.timer = Timer::new();
        self.text_generator = TextGenerator::new(self.difficulty.current().clone());
        self.text_generator
            .load_snippets()
            .wrap_err("Loading snippets failed.")
            .unwrap();
        self.text_generator.calculate_character_weights();
        self.add_snippet();
    }
}
