use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};
use textwrap::fill;
use time::Instant;
use owo_colors::OwoColorize;
use strsim::{jaro_winkler, damerau_levenshtein};

fn calculate_accuracy(sentence: &str, input: &str) -> f64 {
    let similarity = jaro_winkler(sentence, input);
    similarity * 100.0
}

fn highlight_damerau_levenshtein(sentence: &str, input: &str, distance: usize) -> String {
    let mut sentence_chars = sentence.chars().peekable();
    let mut input_chars = input.chars().peekable();
    let mut result = String::new();
    let mut remaining_distance = distance;

    while sentence_chars.peek().is_some() || input_chars.peek().is_some() {
        let sentence_char = sentence_chars.peek().cloned();
        let input_char = input_chars.peek().cloned();

        match (sentence_char, input_char) {
            (Some(sc), Some(ic)) if sc == ic => {
                result.push(ic);
                sentence_chars.next();
                input_chars.next();
            }
            _ => {
                if remaining_distance > 0 {
                    if let Some(ic) = input_char {
                        result.push_str(&ic.red().to_string());
                    }
                    input_chars.next();
                    remaining_distance -= 1;
                } else {
                    break;
                }
            }
        }
    }

    result
}

fn highlight_errors(sentence: &str, input: &str) -> String {
    let distance = damerau_levenshtein(sentence, input);
    highlight_damerau_levenshtein(sentence, input, distance)
}



fn main() {
    let sentences = vec![
        "The quick brown fox jumps over the lazy dog.",
        "Pack my box with five dozen liquor jugs.",
        "How quickly daft jumping zebras vex.",
        // Add more sentences here.
    ];

    let mut rng = rand::thread_rng();
    let sentence = sentences.choose(&mut rng).unwrap();
    let wrapped_sentence = fill(sentence, 80);

    println!("Type the following sentence as fast as you can:\n");
    println!("{}", wrapped_sentence);
    print!("\nPress Enter to start...");
    stdout().flush().unwrap();
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();

    let start_time = Instant::now();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let end_time = Instant::now();
    let duration = end_time - start_time;
    let words = sentence.split_whitespace().count() as f64;
    let minutes = duration.as_seconds_f64() / 60.0;
    let wpm = words / minutes;

    let accuracy = calculate_accuracy(&sentence, input.trim());
    let highlighted_input = highlight_errors(&sentence, input.trim());

    println!("\nYour typing speed is {:.2} WPM with {:.2}% accuracy.", wpm, accuracy);
    println!("\nYour input with errors highlighted in red:");
    println!("{}", highlighted_input);
}




