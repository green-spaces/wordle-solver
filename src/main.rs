
use wordle_solver::guess_result::GuessResult;
use wordle_solver::io::{read_wordle_output, load_dictionary};

const WORD_SOURCE: &str = "scrabble.txt";

// TODO: Bug when a the guess is a complete miss

fn main() {
    let dictionary = load_dictionary(WORD_SOURCE);
    let mut candidates = dictionary.clone();
    println!("Five Letter Words: {}", dictionary.len());

    while !candidates.is_empty() {
        let guess = optimial_guess(&candidates, &dictionary);
        println!(
            "{} => {}",
            guess,
            entropy_votes(&guess, &candidates)
        );
        let response = read_wordle_output();
        candidates = prune_candidates(&guess, response.into_inner(), &candidates);
        println!("{:?}", candidates);
    }
}

fn prune_candidates(guess: &str, response: &str, candidates: &[String]) -> Vec<String> {
    let mut new_candidates = Vec::new();
    for candidate in candidates {
        let mut keep = true;

        // Check direct matches
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

                    keep &=  c_count < g_count;
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

// These outcomes are not computed correctly
fn word_votes(word: &str, guess: &str) -> usize {
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

fn matching_votes(word: &str, word_list: &[String]) -> Vec<usize> {
    let mut votes: Vec<usize> = vec![0; usize::pow(3, word.len().try_into().unwrap())];
    for guess in word_list {
        votes[word_votes(word, guess)] += 1;
    }
    votes
}

fn entropy(votes: &[usize]) -> f32 {
    let incorrect = &votes[..votes.len()];
    let count = incorrect.iter().sum::<usize>() as f32;
    incorrect.into_iter().map(| v| - (*v as f32)/count * f32::ln((*v as f32 + 0.01) / count )).sum::<f32>()
}

/// Returns the worst case remaining number of words
fn entropy_votes(word: &str, word_list: &[String]) -> f32 {
    let votes = matching_votes(word, word_list);
    entropy(&votes)
}

