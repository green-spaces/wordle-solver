use super::guess_result::GuessResult;

pub fn outcome2(guess: &str, target: &str) -> GuessResult {
    let mut res = String::new();

    // mark greens
    guess.chars().zip(target.chars()).for_each(|(g, t)| {
        if g == t {
            res.push('g');
        } else {
            res.push('b');
        }
    });
    

    GuessResult::new(&res)
}

pub fn outcome(guess: &str, target: &str) -> GuessResult {
    let mut res = String::new();

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
    GuessResult::new(&res)
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
                let expected = GuessResult::new("bbbgb");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round2() {
                let guess = "indol";
                let expected = GuessResult::new("bbgyb");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round3() {
                let guess = "coxed";
                let expected = GuessResult::new("ggygy");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round4() {
                let guess = "codex";
                let expected = GuessResult::new("ggggg");
                assert_eq!(outcome(guess, TARGET), expected);
            }
        }

        mod game2 {
            use super::*;
            const TARGET: &str = "peace";

            #[test]
            fn round1() {
                let guess = "tares";
                let expected = GuessResult::new("bybyb");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round2() {
                let guess = "aland";
                let expected = GuessResult::new("bbgbb");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round3() {
                let guess = "hempy";
                let expected = GuessResult::new("bgbyb");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round4() {
                let guess = "peage";
                let expected = GuessResult::new("gggbg");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round5() {
                let guess = "peace";
                let expected = GuessResult::new("ggggg");
                assert_eq!(outcome(guess, TARGET), expected);
            }
        }

        mod game3 {
            use super::*;
            const TARGET: &str = "prose";

            #[test]
            fn round1() {
                let guess = "tares";
                let expected = GuessResult::new("bbyyy");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round2() {
                let guess = "poise";
                let expected = GuessResult::new("gybgg");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round3() {
                let guess = "stats";
                let expected = GuessResult::new("ybbbb");
                assert_eq!(outcome(guess, TARGET), expected);
            }

            #[test]
            fn round4() {
                let guess = "prose";
                let expected = GuessResult::new("ggggg");
                assert_eq!(outcome(guess, TARGET), expected);
            }
        }
    }
}
