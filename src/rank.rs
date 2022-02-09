use super::guess_result::GuessResult;

pub fn outcome(guess: &str, target: &str) -> GuessResult {
    let mut res = String::with_capacity(guess.len());

    for (idx, (g, t)) in guess.chars().zip(target.chars()).enumerate() {
        if g == t {
            res.push('g');
        } else if target.chars().all(|tb| tb != g) {
            res.push('b');
        } else if guess
            .chars()
            .zip(target.chars())
            // excluding target character that match their guess character
            .filter(|(g1, t1)| g1 != t1)
            .any(|(_, t2)| g == t2)
            && (guess
                .chars()
                .enumerate()
                .filter(|(idx2, g2)| *idx2 <= idx && *g2 == g)
                .count()
                <= target.chars().filter(|t2| *t2 == g).count())
        {
            res.push('y');
        } else {
            res.push('b');
        }
    }
    GuessResult::new(res)
}

pub fn outcome_score(guess: &str, target: &str) -> usize {
    let mut res = 0;

    for (idx, (g, t)) in guess.chars().zip(target.chars()).enumerate() {
        if g == t {
            res += 2 * usize::pow(3, idx.try_into().unwrap());
        } else if target.chars().all(|tb| tb != g) {
            continue;
        } else {
            let exclued_matches_target_count = guess
                .chars()
                .zip(target.chars())
                // Remove characters to be marked greem
                .filter(|(g1, t1)| g1 != t1 && g == *t1)
                .count();

            let guess_count = guess
                .chars()
                .enumerate()
                .filter(|(idx2, g2)| *idx2 <= idx && *g2 == g)
                .count();

            if guess_count <= exclued_matches_target_count && guess_count != 0 {
                res += usize::pow(3, idx.try_into().unwrap());
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    mod outcome_method {
        use super::*;

        mod game1 {
            use super::*;
            const TARGET: &str = "codex";

            #[test]
            fn round1() {
                let guess = "tares";
                let expected = GuessResult::new("bbbgb".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round2() {
                let guess = "indol";
                let expected = GuessResult::new("bbgyb".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round3() {
                let guess = "coxed";
                let expected = GuessResult::new("ggygy".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round4() {
                let guess = "codex";
                let expected = GuessResult::new("ggggg".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }
        }

        mod game2 {
            use super::*;
            const TARGET: &str = "peace";

            #[test]
            fn round1() {
                let guess = "tares";
                let expected = GuessResult::new("bybyb".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round2() {
                let guess = "aland";
                let expected = GuessResult::new("bbgbb".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round3() {
                let guess = "hempy";
                let expected = GuessResult::new("bgbyb".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round4() {
                let guess = "peage";
                let expected = GuessResult::new("gggbg".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round5() {
                let guess = "peace";
                let expected = GuessResult::new("ggggg".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }
        }

        mod game3 {
            use super::*;
            const TARGET: &str = "prose";

            #[test]
            fn round1() {
                let guess = "tares";
                let expected = GuessResult::new("bbyyy".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round2() {
                let guess = "poise";
                let expected = GuessResult::new("gybgg".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round3() {
                let guess = "stats";
                let expected = GuessResult::new("ybbbb".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }

            #[test]
            fn round4() {
                let guess = "prose";
                let expected = GuessResult::new("ggggg".to_string());
                assert_eq!(outcome(guess, TARGET), expected);
                assert_eq!(outcome_score(guess, TARGET), expected.score())
            }
        }
    }
}
