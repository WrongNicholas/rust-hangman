use std::collections::HashSet;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let word = fetch_word().await?;
    let stages = get_stages();
    let mut stage_index = 0;

    let characters = word.chars().collect::<HashSet<char>>();
    let mut guesses: HashSet<char> = HashSet::new();

    while stage_index < stages.len() - 1 {
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

async fn fetch_word() -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get("https://random-word-api.herokuapp.com/word")
        .await?
        .json::<Vec<String>>()
        .await?
        .into_iter()
        .next()
        .unwrap())
}

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

fn prompt_user() -> Result<char, Box<dyn Error>> {
    println!("Enter a guess:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.chars().next().unwrap())
}

fn get_revealed_word(word: &str, guesses: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guesses.contains(&c) { c } else { '_' })
        .collect()
}
