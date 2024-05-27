use crate::App;
use crate::TypingEvent;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn get_nth_word_boundaries(app: &mut App, word_offset: usize) -> (usize, usize, usize) {
    let mut word_start = 0;
    let mut word_end = 0;
    let mut words_found = 0;
    let mut found_current = false;
    let mut line_offset = 0;

    // Iterate over lines starting from the current line
    for (line_idx, line) in app.lines.iter().enumerate().skip(app.cur_line) {
        if line_idx > app.cur_line {
            line_offset = line_idx - app.cur_line; // Update line offset
        }

        for idx in 0..line.len() {
            if !found_current {
                if idx == app.position && line_idx == app.cur_line {
                    // found our first word
                    word_start = idx;
                    found_current = true;
                }
            } else {
                if idx == 0 {
                    word_start = 0;
                    words_found += 1;
                    continue;
                }

                if (idx > 0 && line[idx - 1].c.is_whitespace()) || idx == line.len() - 1 {
                    words_found += 1;
                    word_end = idx;
                    if line_offset > 0 && word_start > word_end {
                        word_start = 0;
                    }

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

pub fn capitalize_20_percent(vec: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    let total_count = vec.len();
    let capitalize_count = (total_count as f64 * 0.2).ceil() as usize;

    let mut indices: Vec<usize> = (0..total_count).collect();
    indices.shuffle(&mut rng);

    let capitalize_indices = &indices[..capitalize_count];

    vec.into_iter()
        .enumerate()
        .map(|(i, s)| {
            if capitalize_indices.contains(&i) {
                let mut chars = s.chars();
                match chars.next() {
                    None => s,
                    Some(first_char) => {
                        first_char.to_uppercase().collect::<String>() + chars.as_str()
                    }
                }
            } else {
                s
            }
        })
        .collect()
}

pub fn convert_15_percent_to_numbers(vec: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    let total_count = vec.len();
    let convert_count = (total_count as f64 * 0.15).ceil() as usize;

    let mut indices: Vec<usize> = (0..total_count).collect();
    indices.shuffle(&mut rng);

    let convert_indices = &indices[..convert_count];

    vec.into_iter()
        .enumerate()
        .map(|(i, s)| {
            if convert_indices.contains(&i) {
                rng.gen_range(0..1000).to_string()
            } else {
                s
            }
        })
        .collect()
}

pub fn add_symbols(vec: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    let total_count = vec.len();
    let modify_count = (total_count as f64 * 0.2).ceil() as usize;

    let mut indices: Vec<usize> = (0..total_count).collect();
    indices.shuffle(&mut rng);

    let modify_indices = &indices[..modify_count];

    let common_symbols = [',', '.'];
    let less_common_symbols = ['?', '-', '!', ':'];
    let surrounding_symbols = ["()", "\"\""];

    vec.into_iter()
        .enumerate()
        .map(|(i, s)| {
            if modify_indices.contains(&i) {
                let choice = rng.gen_range(0..100);
                match choice {
                    0..=50 => {
                        // More often add common symbols
                        let random_symbol = common_symbols.choose(&mut rng).unwrap();
                        format!("{}{}", s, random_symbol)
                    }
                    51..=80 => {
                        // Less often add less common symbols
                        let random_symbol = less_common_symbols.choose(&mut rng).unwrap();
                        format!("{}{}", s, random_symbol)
                    }
                    81..=99 => {
                        // Occasionally surround with ()
                        let surrounding = surrounding_symbols.choose(&mut rng).unwrap();
                        let (left, right) = surrounding.split_at(1);
                        format!("{}{}{}", left, s, right)
                    }
                    _ => s,
                }
            } else {
                s
            }
        })
        .collect()
}

pub fn calculate_wpm_and_errors_datasets(
    events: &[TypingEvent],
) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut wpm_data = Vec::new();
    let mut error_data = Vec::new();
    let mut total_chars = 0;

    for (i, event) in events.iter().enumerate() {
        let secs = event.duration_since_start.as_secs_f64();
        let minutes = secs / 60.0;

        if !event.error {
            total_chars += 1;
        } else {
            error_data.push((secs, 5.0));
        }

        let wpm = (total_chars as f64) / 5.0 / minutes;
        // also push a zero time value with the wpm of the first event
        // to make the graph start at 0
        if i == 0 {
            wpm_data.push((0.0, wpm));
        }
        wpm_data.push((secs, wpm));
    }

    (wpm_data, error_data)
}
