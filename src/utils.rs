use crate::App;

pub fn get_nth_word_boundaries(app: &mut App, word_offset: usize) -> (usize, usize, usize) {
    let mut word_start = 0;
    let mut word_end = 0;
    let mut words_found = 0;
    let mut found_current = false;
    let mut line_offset = 0;

    // Iterate over lines starting from the current line
    for (line_idx, line) in app.characters.iter().enumerate().skip(app.cur_line) {
        if line_idx > app.cur_line {
            line_offset = line_idx - app.cur_line; // Update line offset
        }

        for (idx, c) in line.iter().enumerate() {
            if !found_current {
                if idx == app.position && line_idx == app.cur_line {
                    // found our first word
                    word_start = idx;
                    found_current = true;
                }
            } else {
                if c.c.is_whitespace() || idx == line.len() - 1 {
                    words_found += 1;
                    word_end = idx;
                    if line_offset > 0 && word_start > word_end {
                        word_start = 0;
                    } else {
                        word_start += 1; // Skip the whitespace
                    }
                    app.debug_text = format!(
                        "word_start: {}, word_end: {}, line_offset: {}, line_len: {}",
                        word_start,
                        word_end,
                        line_offset,
                        line.len()
                    );

                    if words_found > word_offset {
                        return (word_start, word_end, line_offset);
                    }
                    word_start = idx; // Start of a new word
                }
            }
        }
    }

    // Return the last word found if we run out of text, along with line offset
    (word_start, word_end, line_offset)
}
