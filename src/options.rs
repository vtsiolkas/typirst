#[derive(Debug, Clone)]
pub struct CyclicOption<T> {
    options: Vec<T>,
    pub keybinding: char,
    current: usize,
}

impl<T> CyclicOption<T> {
    pub fn new(options: Vec<T>, c: char) -> Self {
        Self {
            options,
            keybinding: c,
            current: 0,
        }
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.options.len();
    }

    pub fn current(&self) -> &T {
        &self.options[self.current]
    }

    pub fn surrounding(&self) -> (&T, &T, &T) {
        let prev = if self.current == 0 {
            self.options.len() - 1
        } else {
            self.current - 1
        };

        let next = (self.current + 1) % self.options.len();

        (
            &self.options[prev],
            &self.options[self.current],
            &self.options[next],
        )
    }
}
pub trait Labeled {
    fn label(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDifficulty {
    Lowercase,
    Numbers,
    Uppercase,
    Symbols,
}
impl Labeled for TextDifficulty {
    fn label(&self) -> String {
        match self {
            TextDifficulty::Lowercase => "lowercase".to_string(),
            TextDifficulty::Numbers => "numbers".to_string(),
            TextDifficulty::Uppercase => "uppercase".to_string(),
            TextDifficulty::Symbols => "symbols".to_string(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Highlight {
    Nothing,
    Character,
    Word,
    NextWord,
    TwoWords,
}
impl Highlight {
    pub fn get_words_ahead(&self) -> usize {
        match self {
            Highlight::NextWord => 1,
            Highlight::TwoWords => 2,
            _ => 0,
        }
    }
}
impl Labeled for Highlight {
    fn label(&self) -> String {
        match self {
            Highlight::Nothing => "nothing".to_string(),
            Highlight::Character => "character".to_string(),
            Highlight::Word => "word".to_string(),
            Highlight::NextWord => "word ahead".to_string(),
            Highlight::TwoWords => "2 words ahead".to_string(),
        }
    }
}
