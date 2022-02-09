use super::rank::outcome_score;
use rand::Rng;
use rayon::prelude::*;

pub fn optimial_guess<'a>(candidates: &[String], dictionary: &'a [String]) -> &'a String {
    let mut best_word: &String = &dictionary[0];
    let mut max_entropy = f32::MIN;

    let entropies: Vec<f32> = dictionary
        .into_par_iter()
        .map(|word| calculate_entropy(word, candidates))
        .collect();

    for (&entropy, word) in entropies.iter().zip(dictionary) {
        if entropy > max_entropy
            || (entropy == max_entropy && candidates.iter().any(|cad| cad == word))
        {
            best_word = word;
            max_entropy = entropy;
        }
    }

    best_word
}

pub fn calculate_entropy(word: &str, word_list: &[String]) -> f32 {
    calculate_entropy_from_dist(&calculate_outcome_freq(word, word_list))
}

fn calculate_entropy_from_dist(votes: &[f32]) -> f32 {
    let count: f32 = votes.iter().sum();
    votes
        .iter()
        .map(|v| -*v / count * (f32::ln(*v / count + 0.001)))
        .sum::<f32>()
}

fn calculate_outcome_freq(word: &str, word_list: &[String]) -> Vec<f32> {
    let mut outcome_freq: Vec<f32> = vec![0.0; usize::pow(3, 5)];
    let mut rng = rand::thread_rng();
    let num_of_words = word_list.len() as f32;
    let min_sample = 3.0 * f32::powf(3.0, 5.0);

    let sub_list= word_list
        .iter()
        .filter(|_| {
            rng.gen_range(0.0..num_of_words) < f32::max(f32::sqrt(num_of_words), min_sample)
        });
    
    for guess in sub_list {
        outcome_freq[outcome_score(word, guess)] += 1.0;
    }

    outcome_freq
}
