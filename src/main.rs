use wordle_solver::guess_generator::{entropy_votes, optimial_guess};
use wordle_solver::guess_result::GuessResult;
use wordle_solver::io::{load_dictionary, read_wordle_output};
use wordle_solver::rank::outcome;

const WORD_SOURCE: &str = "scrabble.txt";

// TODO: Bug when a the guess is a complete miss

fn main() {
    let dictionary = load_dictionary(WORD_SOURCE);
    let mut candidates = dictionary.clone();
    println!("Five Letter Words: {}", dictionary.len());
    let mut votes: Vec<f32> = vec![0.0; usize::pow(3, candidates[0].len().try_into().unwrap())];

    while !candidates.is_empty() {
        let guess_full = optimial_guess(&candidates, &dictionary);
        println!(
            "Full: {} => {}",
            guess_full,
            entropy_votes(&guess_full, &candidates, &mut votes)
        );
        let response = read_wordle_output();
        candidates = prune_candidates(guess_full, &response, &candidates);
    }
}

fn prune_candidates(guess: &str, guess_result: &GuessResult, candidates: &[String]) -> Vec<String> {
    let guess_score = guess_result.score();
    let all_correct = 242; // 3^5 - 1
    candidates
        .iter()
        .filter(|can| outcome(guess, *can).score() == guess_score && guess_score != all_correct)
        .map(|s| s.clone())
        .collect::<Vec<String>>()
}
