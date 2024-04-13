use color_eyre::Result;
use rand::seq::SliceRandom;
use std::collections::HashMap;

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

    pub fn set_typed(&mut self, typed_c: char) -> usize {
        self.typed_c = typed_c;

        if self.c == typed_c {
            self.state = CharState::Correct;
            0
        } else {
            self.state = CharState::Incorrect;
            1
        }
    }

    pub fn reset(&mut self) {
        self.typed_c = self.c;
        self.state = CharState::Untouched;
    }
}

#[derive(Debug)]
pub struct TextGenerator {
    snippets: Vec<String>,
    character_weights: HashMap<String, HashMap<usize, usize>>,
}

impl TextGenerator {
    pub fn new() -> Self {
        Self {
            snippets: vec![],
            character_weights: HashMap::new(),
        }
    }

    pub fn load_snippets(&mut self) -> Result<()> {
        let text = include_str!("../assets/text.txt");
        self.snippets = text
            .lines()
            .map(|s| s.to_string())
            .filter(|s| s != "#!#!#!#!#!")
            .collect();
        Ok(())
    }

    pub fn calculate_character_weights(&mut self) {
        for (idx, line) in self.snippets.iter().enumerate() {
            for c in line.chars() {
                let c_str = c.to_string();
                let weight = self
                    .character_weights
                    .entry(c_str)
                    .or_insert(HashMap::new());
                weight.entry(idx).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    pub fn generate_characters(&self, c: char, max_len: usize) -> Vec<Vec<Character>> {
        let snippet = self.select_snippet(c);
        self.split_string(&snippet, max_len)
    }

    fn select_snippet(&self, c: char) -> String {
        let c_str = c.to_string();
        let weights = self
            .character_weights
            .get(&c_str)
            .cloned()
            .unwrap_or_else(HashMap::new);

        let mut weights_vec: Vec<_> = weights.iter().collect();
        weights_vec.sort_by(|a, b| b.1.cmp(a.1));

        let top_5: Vec<_> = weights_vec.into_iter().take(5).collect();

        let random_snippet = top_5
            .choose(&mut rand::thread_rng())
            .unwrap_or_else(|| &(&0, &0));

        self.snippets[*random_snippet.0].clone()
    }

    fn split_string(&self, input: &str, max_len: usize) -> Vec<Vec<Character>> {
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
                    split_at // + (if c == '\n' { 1 } else { 0 })
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
