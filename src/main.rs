use wordle_solver::guess_result::{self, GuessResult};
use wordle_solver::io::{load_dictionary, read_wordle_output};
use wordle_solver::rank::outcome;

const WORD_SOURCE: &str = "scrabble.txt";

// TODO: Bug when a the guess is a complete miss

fn main() {
    let dictionary = load_dictionary(WORD_SOURCE);
    let mut candidates = dictionary.clone();
    println!("Five Letter Words: {}", dictionary.len());

    while !candidates.is_empty() {
        let guess_full = optimial_guess(&candidates, &dictionary);
        let guess_constrained = optimial_guess(&candidates, &candidates);
        println!(
            "Full: {} => {}",
            guess_full,
            entropy_votes(&guess_full, &candidates)
        );
        println!(
            "Remaining: {} => {}",
            guess_constrained,
            entropy_votes(&guess_constrained, &candidates)
        );
        let response = read_wordle_output();
        candidates = prune_candidates(&guess_full, &response, &candidates);
        // println!("Remaining words: {}", candidates.len());
    }
}

fn prune_candidates(guess: &str, guess_result: &GuessResult, candidates: &[String]) -> Vec<String> {
    let guess_score = guess_result.score();

    candidates
        .iter()
        .filter(|can| outcome(guess, *can).score() == guess_score)
        .map(|s| s.clone())
        .collect::<Vec<String>>()
}

fn optimial_guess(candidates: &[String], dictionary: &[String]) -> String {
    let mut best_word: String = dictionary[0].clone();
    let mut max_entropy = f32::MIN;

    for word in dictionary.iter() {
        let res = entropy_votes(word, candidates);
        if res > max_entropy || (res == max_entropy && candidates.iter().any(|cad| cad == word)) {
            best_word = word.clone();
            max_entropy = res;
        }
    }
    best_word
}

/// Returns the worst case remaining number of words
fn entropy_votes(word: &str, word_list: &[String]) -> f32 {
    let votes = matching_votes(word, word_list);
    entropy(&votes)
}

fn matching_votes(word: &str, word_list: &[String]) -> Vec<usize> {
    let mut votes: Vec<usize> = vec![0; usize::pow(3, word.len().try_into().unwrap())];
    for guess in word_list {
        votes[outcome(word, guess).score()] += 1;
    }
    votes
}

fn entropy(votes: &[usize]) -> f32 {
    let incorrect = &votes[..votes.len()];
    let count = incorrect.iter().sum::<usize>() as f32;
    incorrect
        .iter()
        .map(|v| -(*v as f32) / count * f32::ln((*v as f32 + 0.01) / count))
        .sum::<f32>()
}
