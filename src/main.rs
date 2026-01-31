use std::collections::HashSet;
use std::error::Error;
use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Example CLI", long_about = None)]
struct Args {
    #[arg(short, long)]
    length: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // gather the URL, adding `length` if between 2-15 inclusive
    let url = match args.length {
        Some(length) if (2..=15).contains(&length) => {
            format!(
                "https://random-word-api.herokuapp.com/word?length={length}"
            )
        }
        _ => "https://random-word-api.herokuapp.com/word".to_string()
    };

    // set up game
    let word = fetch_word(&url).await?;
    let stages = get_stages();
    let mut stage_index = 0;

    let characters = word.chars().collect::<HashSet<char>>();
    let mut guesses: HashSet<char> = HashSet::new();

    while stage_index < stages.len() - 1 {

        // clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let revealed_word = get_revealed_word(&word, &guesses);

        println!(
            "Guesses:{:?}\n{}\n{}",
            guesses, stages[stage_index], revealed_word
        );

        if revealed_word == word {
            println!("Congratulations! You guessed the word!");
            break;
        }

        let guess = prompt_user()?;
        guesses.insert(guess);

        if !characters.contains(&guess) {
            stage_index += 1;
        }

        if stage_index == stages.len() - 1 {
            println!("Game Over! The word was: {}", word);
        }
    }

    Ok(())
}

// fetches a random word from the given API URL
async fn fetch_word(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url)
        .await?
        .json::<Vec<String>>()
        .await?
        .into_iter()
        .next()
        .unwrap())
}

// returns ASCII art stages to display hangman progress
fn get_stages() -> Vec<String> {
    let stages = [
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
    ];

    stages.iter().map(|&s| s.to_string()).collect()
}

// prompts the user for a single character guess
fn prompt_user() -> Result<char, Box<dyn Error>> {
    println!("Enter a guess:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.chars().next().unwrap())
}

// returns the partially revealed word based on previous guesses
fn get_revealed_word(word: &str, guesses: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guesses.contains(&c) { c } else { '_' })
        .collect()
}
