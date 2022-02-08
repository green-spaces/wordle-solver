use super::guess_result::GuessResult;

// These outcomes are not computed correctly
pub fn overlap(word: &str, guess: &str) -> usize {
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

pub fn outcome(guess: &str, target: &str) -> GuessResult {
    let mut res = String::new();
    for (g, t) in guess.chars().zip(target.chars()) {
        if g == t {
            res.push('g');
        } else if guess
            .chars()
            .zip(target.chars())
            // excluding target character that match their guess character
            .filter(|(g1, t1)| g1 != t1)
            .any(|(_, t2)| g == t2)
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
    }
}
