use std::fs::File;
use std::io::{self, prelude::*};

const WORD_SOURCE: &str = "less-englisth-words.txt";

fn main() {
    let words = read_lines(WORD_SOURCE);
    let mut best_word: String;
    let mut min_options = u32::MAX;

    for word in words.iter() {
        let res = remaining_words(word, &words);
        if res <= min_options {
            best_word = word.clone();
            min_options = res;
            println!("{} => {}", best_word, min_options);
        }
    }

    println!("Five Letter Words: {}", words.len());
}

fn word_votes(word: &String, guess: &String) -> usize {
    let mut out = 0;
    for (idx, w) in word.chars().enumerate(){
        if guess.chars().any(|g| g == w) {
            out += usize::pow(2, idx.try_into().unwrap());
        }
    }
    out
}

fn matching_votes(word: &String, word_list: &Vec<String>) -> Vec<u32> {
    let mut votes = vec![0; usize::pow(2, word.len().try_into().unwrap())];
    for guess in word_list {
        votes[word_votes(word, guess)] += 1;
    }
    votes
}

/// Returns the worst case remaining number of words
fn remaining_words(word: &String, word_list: &Vec<String>) -> u32 {
    let votes = matching_votes(word, word_list);
    *votes.iter().max().unwrap()
}

fn read_lines(file: &str) -> Vec<String> {
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
