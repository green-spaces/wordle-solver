use std::fs::File;
use std::io::{self, prelude::*};

const WORD_SOURCE: &str = "scrabble.txt";

fn main() {
    let dictionary = load_dictionary(WORD_SOURCE);
    let mut candidates = dictionary.clone();

    let guess = optimial_guess(&candidates, &dictionary);
    println!("{} => {}", guess, worst_cast_words_left(&guess, &candidates));


    println!("Five Letter Words: {}", dictionary.len());
}

fn optimial_guess(candidates: &Vec<String>, dictionary: &Vec<String>) -> String {
    let mut best_word: String = dictionary[0].clone();
    let mut min_options = u32::MAX;

    for word in dictionary.iter() {
        let res = worst_cast_words_left(word, &candidates);
        if res <= min_options {
            best_word = word.clone();
            min_options = res;
            println!("{} => {}", best_word, min_options);
        }
    }
    best_word
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
fn worst_cast_words_left(word: &String, word_list: &Vec<String>) -> u32 {
    let votes = matching_votes(word, word_list);
    *votes.iter().max().unwrap()
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
