use super::rank::outcome_score;

pub fn optimial_guess<'a>(candidates: &[String], dictionary: &'a [String]) -> &'a String {
    let mut best_word: &String = &dictionary[0];
    let mut max_entropy = f32::MIN;

    let mut votes: Vec<f32> = vec![0.0; usize::pow(3, candidates[0].len().try_into().unwrap())];

    for word in dictionary.iter() {
        let res = entropy_votes(word, candidates, &mut votes);
        if res > max_entropy || (res == max_entropy && candidates.iter().any(|cad| cad == word)) {
            best_word = &word;
            max_entropy = res;
        }
    }
    best_word
}

/// Returns the worst case remaining number of words
pub fn entropy_votes(word: &str, word_list: &[String], votes: &mut [f32]) -> f32 {
    matching_votes(word, word_list, votes);
    entropy(votes)
}

fn matching_votes(word: &str, word_list: &[String], votes: &mut [f32]) {
    for i in 0..votes.len() {
        votes[i] = 0.0;
    }

    for guess in word_list {
        votes[outcome_score(word, guess)] += 1.0;
    }
}

fn entropy(votes: &[f32]) -> f32 {
    let count: f32 = votes.iter().sum();
    votes
        .iter()
        .map(|v| -*v / count * (f32::ln(*v / count + 0.001)))
        .sum::<f32>()
}
