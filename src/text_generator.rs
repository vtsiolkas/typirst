use crate::options::{self, NumberOfWords};
use crate::utils::{add_symbols, capitalize_20_percent, convert_15_percent_to_numbers};
use color_eyre::Result;
use options::TextDifficulty;
use rand::seq::SliceRandom;

#[derive(Debug, Default)]
pub enum CharState {
    #[default]
    Untouched,
    Correct,
    Incorrect,
}

#[derive(Debug, Default)]
pub struct Character {
    pub c: char,
    pub typed_c: char,
    pub state: CharState,
}

impl Character {
    pub fn new(c: char) -> Self {
        Self {
            c,
            typed_c: c,
            state: CharState::Untouched,
        }
    }

    pub fn set_typed(&mut self, typed_c: char) -> bool {
        self.typed_c = typed_c;

        if self.c == typed_c {
            self.state = CharState::Correct;
            false
        } else {
            self.state = CharState::Incorrect;
            true
        }
    }

    pub fn reset(&mut self) {
        self.typed_c = self.c;
        self.state = CharState::Untouched;
    }
}

#[derive(Debug)]
pub struct TextGenerator {
    words: Vec<String>,
    difficulty: TextDifficulty,
    number_of_words: NumberOfWords,
}

impl TextGenerator {
    pub fn new(number_of_words: NumberOfWords, difficulty: TextDifficulty) -> Self {
        Self {
            words: vec![],
            number_of_words,
            difficulty,
        }
    }

    pub fn load_words(&mut self) -> Result<()> {
        let text = include_str!("../assets/words.txt");

        // Select words from the text, split by comma
        self.words = text
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        Ok(())
    }

    pub fn generate_lines(&self, max_len: usize) -> Vec<Vec<Character>> {
        let text = self.select_words();
        self.split_string(text, max_len)
    }

    fn select_words(&self) -> String {
        // Select num_words random words
        (0..self.number_of_words as usize)
            .map(|_| self.words.choose(&mut rand::thread_rng()).unwrap().clone())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn apply_difficulty(&self, input: String) -> String {
        let mut words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        words = match self.difficulty {
            TextDifficulty::Lowercase => words,
            TextDifficulty::Uppercase => capitalize_20_percent(words),
            TextDifficulty::Numbers => convert_15_percent_to_numbers(capitalize_20_percent(words)),
            TextDifficulty::Symbols => {
                add_symbols(convert_15_percent_to_numbers(capitalize_20_percent(words)))
            }
        };

        words.join(" ")
    }

    fn split_string(&self, input: String, max_len: usize) -> Vec<Vec<Character>> {
        let input = self.apply_difficulty(input);

        let mut result = Vec::new();
        let mut start = 0;
        let mut last_space = 0;
        let mut len = 0;

        for (idx, c) in input.char_indices() {
            // Increase length for each character
            len += 1;

            // Keep track of the last space encountered
            if c.is_whitespace() {
                last_space = idx;
            }

            // If a newline is encountered or the length reaches max_len
            if c == '\n' || len == max_len {
                // If we've reached max_len and the current character is not a whitespace,
                // and there was a previous space in the current slice,
                // we split at the last space to avoid breaking a word.
                let split_at = if len == max_len && !c.is_whitespace() && last_space > start {
                    last_space + 1
                } else {
                    idx + 1
                };

                // Add the slice to the result, excluding the newline if it's there
                let chars: Vec<char> = input[start..split_at].chars().collect();
                let line: Vec<Character> = chars.iter().map(|c| Character::new(*c)).collect();
                result.push(line);

                // Update start; if we split at a space, start from the next character
                // Otherwise, start from the current character or the next one after the newline
                start = if split_at == last_space {
                    split_at + 1
                } else {
                    split_at //+ (if c == '\n' { 1 } else { 0 })
                };

                // Reset len and last_space for the next slice
                len = 0;
                last_space = start;
            }
        }

        // Check if there's a remaining slice to add
        if start < input.len() {
            let chars: Vec<char> = input[start..].chars().collect();
            let line: Vec<Character> = chars.iter().map(|c| Character::new(*c)).collect();
            result.push(line);
        }

        result
    }
}
