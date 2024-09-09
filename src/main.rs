use std::collections::HashSet;
use std::time::Instant;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const COLOUR_RESET: &str = "\x1b[0m";

fn caesar_shift(word: &str, shift: usize) -> String {
    word.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' };
                let offset = (c as u8 - base as u8 + shift as u8) % 26;
                (base as u8 + offset) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let dictionary = include_str!("dictionary.txt");
    let words = dictionary.split('\n').collect::<Vec<_>>();
    let word_set: HashSet<&str> = words.iter().copied().collect();
    let total_words = words.len();
    let mut processed_count = 0;
    let mut results = Vec::new();

    let start_time = Instant::now();

    for (_, word) in words.iter().enumerate() {
        if word.len() < 3 {
            continue;
        }
        for shift in 1..26 {
            let shifted_word = caesar_shift(word, shift);
            if word_set.contains(shifted_word.as_str()) {
                results.push((word.to_string(), shift, shifted_word));
            }
        }

        processed_count += 1;
        if processed_count % 100 == 0 || processed_count == total_words {
            let elapsed = start_time.elapsed().as_secs();
            let percentage = (processed_count as f64 / total_words as f64) * 100.0;
            let remaining = total_words - processed_count;
            let estimated_time = if elapsed > 0 {
                let average_time_per_word = elapsed as f64 / processed_count as f64;
                let remaining_time = average_time_per_word * remaining as f64;
                remaining_time
            } else {
                0.0
            };
            println!(
                "Progress: {:.2}% - Processed: {} / {} - Remaining: {} - Estimated Time Left: {:.2} seconds",
                percentage,
                processed_count,
                total_words,
                remaining,
                estimated_time
            );
        }
    }

    for (original_word, shift, new_word) in results {
        println!(
            "{} -> {} via {}",
            format!("{}{}{}", RED, original_word, COLOUR_RESET),
            format!("{}{}{}", GREEN, new_word, COLOUR_RESET),
            format!("{}{}{}", BLUE, shift, COLOUR_RESET)
        );
    }
}
