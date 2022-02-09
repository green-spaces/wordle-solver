#[macro_use]
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wordle_solver::guess_generator::optimial_guess;
use wordle_solver::io::load_dictionary;

// const WORD_SOURCE: &str = "less-english-words.txt";
const WORD_SOURCE: &str = "scrabble.txt";

lazy_static! {
    static ref DICTIONARY: Vec<String> = load_dictionary(WORD_SOURCE);
}

fn time_to_first_guess(i: i32) -> i32 {
    let candidates = DICTIONARY.clone();
    let _guess_full = optimial_guess(&candidates, &DICTIONARY);
    i * 2
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("outcome", |b| b.iter(|| time_to_first_guess(black_box(2))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
