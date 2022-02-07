use std::fs::File;
use std::io::{self, prelude::*};

const WORD_SOURCE: &str = "scrabble.txt";

fn main() {
    let dictionary = load_dictionary(WORD_SOURCE);
    let mut candidates = dictionary.clone();
    println!("Five Letter Words: {}", dictionary.len());

    while candidates.len() != 0 {
        let guess = optimial_guess(&candidates, &dictionary);
        println!(
            "{} => {}",
            guess,
            worst_cast_words_left(&guess, &candidates)
        );
        let response = read_input();
        candidates = prune_candidates(&guess, &response, &candidates);
        println!("{:?}", candidates);
    }
}

fn prune_candidates(guess: &String, response: &String, candidates: &Vec<String>) -> Vec<String> {
    let mut new_candidates = Vec::new();
    for candidate in candidates {
        let mut keep = true;

        /// Check direct matches
        for ((idx_g, (g, r)), (idx_c, c)) in guess
            .chars()
            .zip(response.chars())
            .enumerate()
            .zip(candidate.chars().enumerate())
        {
            match r {
                'g' => {
                    keep &= g == c;
                }
                'y' => {
                    keep &= candidate.chars().any(|c| c == g)
                        && guess.chars().nth(idx_g) != candidate.chars().nth(idx_c);
                }
                'b' => {
                    // Needs a special case for words with two letters
                    let c_count = candidate.matches(g).count();
                    let g_count = guess.matches(g).count();

                    // keep &= !candidate.chars().any(|c| c == g);
                    keep &=  !(c_count > g_count);
                }
                _ => {}
            }
        }

        // Dont keep incorrect guesses
        if candidate == guess {
            keep = false;
        }

        if keep {
            new_candidates.push(candidate.clone())
        }
    }

    new_candidates
}

fn read_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn optimial_guess(candidates: &Vec<String>, dictionary: &Vec<String>) -> String {
    if candidates.len() == 1 {
        return candidates[0].clone();
    }

    let mut best_word: String = dictionary[0].clone();
    let mut min_options = usize::MAX;

    for word in dictionary.iter() {
        let res = worst_cast_words_left(word, &candidates);
        if res < min_options || (res == min_options && candidates.iter().any(|cad| cad == word)) {
            best_word = word.clone();
            min_options = res;
        }
    }
    best_word
}

fn word_votes(word: &String, guess: &String) -> usize {
    let mut out = 0;
    for ((idx, w), g) in word.chars().enumerate().zip(guess.chars()) {
        let coeff;
        if w == g {
            coeff = 2
        } else if guess.chars().any(|g| g == w) && guess.chars().nth(idx).unwrap() != w {
            coeff = 1;
        } else {
            coeff = 0;
        }
        out += coeff * usize::pow(3, idx.try_into().unwrap());
    }
    out
}

fn matching_votes(word: &String, word_list: &Vec<String>) -> Vec<usize> {
    let mut votes: Vec<usize> = vec![0; usize::pow(3, word.len().try_into().unwrap())];
    for guess in word_list {
        votes[word_votes(word, guess)] += 1;
    }
    votes
}

/// Returns the worst case remaining number of words
fn worst_cast_words_left(word: &String, word_list: &Vec<String>) -> usize {
    let votes = matching_votes(word, word_list);
    *votes.iter().max().unwrap()
    // votes.into_iter().map(|v| v * usize::ln(v + 1.0)).sum::<usize>()
}

fn load_dictionary(file: &str) -> Vec<String> {
    let file = File::open(file).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut flw: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(word) = line {
            if word.len() == 5 {
                flw.push(word);
            }
        }
    }

    flw
}
