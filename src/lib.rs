mod options;
mod text_generator;
mod timer;
pub mod tui;
mod ui;
mod utils;

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use options::{CyclicOption, Highlight, TextDifficulty};
use std::time::Instant;
use text_generator::{Character, TextGenerator};
use timer::Timer;
use ui::ui;

#[derive(Debug)]
pub struct App {
    characters: Vec<Vec<Character>>,
    stats: Vec<TypingEvent>,
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

#[derive(Debug)]
struct TypingEvent {
    instant: Instant,
    error: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            characters: vec![],
            stats: Vec::new(),
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

        // Generate some initial snippets
        self.add_snippet();

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

    fn add_snippet(&mut self) {
        let mut characters = self.text_generator.generate_characters(50);
        self.characters.append(&mut characters);
    }

    fn check_character(&mut self, c: char) {
        if !self.timer.running {
            self.timer.start();
        }
        self.typed_chars += 1;
        let error = self.characters[self.cur_line][self.position].set_typed(c);
        if error {
            self.errors += 1;
        }
        self.position += 1;

        self.stats.push(TypingEvent {
            instant: Instant::now(),
            error,
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
