use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};
use textwrap::fill;
use time::Instant;

fn calculate_accuracy(sentence: &str, input: &str) -> f64 {
    let sentence_chars = sentence.chars().collect::<Vec<char>>();
    let input_chars = input.chars().collect::<Vec<char>>();
    let mut correct_chars = 0;

    for (i, input_char) in input_chars.iter().enumerate() {
        if let Some(sentence_char) = sentence_chars.get(i) {
            if input_char == sentence_char {
                correct_chars += 1;
            }
        }
    }

    (correct_chars as f64 / sentence_chars.len() as f64) * 100.0
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

    println!("\nYour typing speed is {:.2} WPM with {:.2}% accuracy.", wpm, accuracy);
}



