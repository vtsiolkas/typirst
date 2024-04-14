use crate::App;

pub fn get_nth_word_boundaries(app: &App, word_offset: usize) -> (usize, usize, usize) {
    let mut word_start = 0;
    let mut word_end = 0;
    let mut words_found = 0;
    let mut found_current = false;
    let mut line_offset = 0;
    let mut position = app.position;

    // Iterate over lines starting from the current line
    for (line_idx, line) in app.characters.iter().enumerate().skip(app.cur_line) {
        if line_idx > app.cur_line {
            line_offset = line_idx - app.cur_line; // Update line offset
        }

        for (idx, c) in line.iter().enumerate() {
            if (idx == position && !found_current) || found_current {
                // Starting from current cursor position or already started in a previous iteration
                found_current = true;
                if c.c != ' ' && c.c != '\n' && word_end != 0 {
                    words_found += 1;
                    if words_found > word_offset {
                        return (word_start, word_end, line_offset);
                    }
                    word_start = idx; // Start of a new word
                    word_end = 0; // Reset end since it's a new word
                } else if c.c == ' ' || c.c == '\n' {
                    if word_end == 0 {
                        // First time setting end after finding start
                        word_end = idx;
                    }
                }
            }
        }
        // If the end of a line doesn't terminate a word, set it here
        if word_end == 0 && word_start != 0 {
            word_end = line.len();
        }
        // Check if we've found enough words
        if words_found > word_offset {
            return (word_start, word_end, line_offset);
        }
        // Reset for the next line
        found_current = true; // Start at the beginning of the next line
        position = 0; // Reset position for the next line
    }

    // Return the last word found if we run out of text, along with line offset
    (word_start, word_end, line_offset)
}
