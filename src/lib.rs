mod options;
mod text_generator;
mod timer;
pub mod tui;
mod ui;
mod utils;

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use options::{CyclicOption, Highlight, NumberOfWords, TextDifficulty};
use std::time::Duration;
use text_generator::{Character, TextGenerator};
use timer::Timer;
use ui::ui;

#[derive(Debug)]
pub struct App {
    lines: Vec<Vec<Character>>,
    stats: Vec<TypingEvent>,
    cur_line: usize,
    position: usize,
    typed_chars: usize,
    errors: usize,
    pause: bool,
    quit: bool,
    timer: Timer,
    text_generator: TextGenerator,
    number_of_words: CyclicOption<NumberOfWords>,
    difficulty: CyclicOption<TextDifficulty>,
    highlight: CyclicOption<Highlight>,
    showing_stats: bool,
}

#[derive(Debug)]
struct TypingEvent {
    duration_since_start: Duration,
    error: bool,
}

const NUMBER_OF_WORDS_KEYBINDING: char = 'w';
const DIFFICULTY_KEYBINDING: char = 'd';
const HIGHLIGHT_KEYBINGING: char = 'h';

impl App {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            stats: Vec::new(),
            cur_line: 0,
            position: 0,
            typed_chars: 0,
            errors: 0,
            pause: false,
            quit: false,
            timer: Timer::new(),
            number_of_words: CyclicOption::new(
                vec![
                    NumberOfWords::Ten,
                    NumberOfWords::Thirty,
                    NumberOfWords::Fifty,
                    NumberOfWords::OneHundred,
                    NumberOfWords::TwoHundred,
                    NumberOfWords::FiveHundred,
                ],
                NUMBER_OF_WORDS_KEYBINDING,
                "Words",
            ),
            difficulty: CyclicOption::new(
                vec![
                    TextDifficulty::Lowercase,
                    TextDifficulty::Uppercase,
                    TextDifficulty::Numbers,
                    TextDifficulty::Symbols,
                ],
                DIFFICULTY_KEYBINDING,
                "Difficulty",
            ),
            highlight: CyclicOption::new(
                vec![
                    Highlight::Nothing,
                    Highlight::Character,
                    Highlight::Word,
                    Highlight::NextWord,
                    Highlight::TwoWords,
                ],
                HIGHLIGHT_KEYBINGING,
                "Highlight",
            ),
            text_generator: TextGenerator::new(NumberOfWords::Ten, TextDifficulty::Lowercase),
            showing_stats: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.text_generator
            .load_words()
            .wrap_err("Loading word list failed.")?;

        // Generate lines of characters
        self.lines = self.text_generator.generate_lines(50);

        while !self.quit {
            terminal.draw(|frame| ui(frame, self))?;
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

    fn show_stats(&mut self) {
        self.pause();
        self.showing_stats = true;
    }

    fn check_character(&mut self, c: char) {
        let error = self.lines[self.cur_line][self.position].set_typed(c);
        if error {
            self.errors += 1;
        } else {
            self.typed_chars += 1;
        }
        self.position += 1;

        if !self.timer.running {
            self.timer.start();
        } else {
            self.stats.push(TypingEvent {
                duration_since_start: self.timer.elapsed(),
                error,
            });
        }

        if self.position == self.lines[self.cur_line].len() {
            self.position = 0;
            self.cur_line += 1;
            if self.cur_line == self.lines.len() {
                self.show_stats();
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if self.showing_stats {
            match key_event.code {
                KeyCode::Char('q') => self.quit(),
                KeyCode::Char('r') => {
                    self.unpause();
                    self.reset();
                }
                _ => {}
            }
            return Ok(());
        } else if self.pause {
            match key_event.code {
                KeyCode::Esc => self.unpause(),
                KeyCode::Char('q') => self.quit(),
                KeyCode::Char('r') => {
                    self.unpause();
                    self.reset();
                }
                KeyCode::Char(NUMBER_OF_WORDS_KEYBINDING) => {
                    self.number_of_words.next();
                    self.reset();
                }
                KeyCode::Char(DIFFICULTY_KEYBINDING) => {
                    self.difficulty.next();
                    self.reset();
                }
                KeyCode::Char(HIGHLIGHT_KEYBINGING) => {
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
                        self.position = self.lines[self.cur_line].len() - 1;
                    }

                    if self.lines[self.cur_line][self.position].state
                        == text_generator::CharState::Correct
                    {
                        self.typed_chars -= 1;
                    }
                    self.lines[self.cur_line][self.position].reset();
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
        // Only restart the timer if we're not at the beginning of the game
        if self.timer.elapsed().as_secs() > 0 {
            self.timer.start();
        }
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn reset(&mut self) {
        self.cur_line = 0;
        self.position = 0;
        self.typed_chars = 0;
        self.errors = 0;
        self.timer = Timer::new();
        self.stats = Vec::new();
        self.showing_stats = false;
        self.text_generator = TextGenerator::new(
            self.number_of_words.current().clone(),
            self.difficulty.current().clone(),
        );
        self.text_generator
            .load_words()
            .wrap_err("Loading words failed.")
            .unwrap();
        self.lines = self.text_generator.generate_lines(50);
    }
}
