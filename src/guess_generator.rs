use super::rank::outcome;

pub fn optimial_guess(candidates: &[String], dictionary: &[String]) -> String {
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
pub fn entropy_votes(word: &str, word_list: &[String]) -> f32 {
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
