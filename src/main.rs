use serde::Deserialize;
use std::collections::HashSet;
use std::io;

#[derive(Deserialize)]
struct Word {
    word: String,
}

#[derive(Deserialize)]
struct Stages {
    stages: Vec<String>,
}

#[tokio::main]
async fn main() {
    let word = fetch_word().await;
    let stages = get_stages();
    let mut stage_index = 0;

    let characters = word.chars().collect::<HashSet<char>>();

    let mut guesses: HashSet<char> = HashSet::new();

    display(
        &stages,
        stage_index,
        &get_revealed_word(&word, &guesses),
        &guesses,
    );

    while stage_index < stages.len() - 1 {
        let guess = prompt_user();
        guesses.insert(guess);

        if !characters.contains(&guess) {
            stage_index += 1;
        }

        let revealed_word = get_revealed_word(&word, &guesses);
        display(&stages, stage_index, &revealed_word, &guesses);

        if revealed_word == word {
            println!("Congratulations! You guessed the word: {}", word);
            break;
        }

        if stage_index == stages.len() - 1 {
            println!("Game Over! The word was: {}", word);
        }
    }
}

fn get_revealed_word(word: &str, guesses: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guesses.contains(&c) { c } else { '_' })
        .collect()
}

fn prompt_user() -> char {
    println!("Enter a guess:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.chars().next().unwrap()
}

fn display(stages: &[String], stages_index: usize, word: &String, guesses: &HashSet<char>) {
    println!("Guesses:{:?}\n{}\n{}", guesses, stages[stages_index], word);
}

async fn fetch_word() -> String {
    let url = String::from("https://random-word-api.herokuapp.com/word");

    reqwest::get(&url)
        .await
        .expect("Error sending GET request.")
        .json::<Word>()
        .await
        .expect("Failed to parse response as JSON")
        .word
}

fn get_stages() -> Vec<String> {
    let file = std::fs::File::open("res/stages.json").expect("File should open read only");
    serde_json::from_reader::<std::fs::File, Stages>(file)
        .expect("Error reading or parsing JSON")
        .stages
}
