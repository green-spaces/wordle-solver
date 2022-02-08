use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wordle_solver::guess_generator::optimial_guess;
use wordle_solver::io::load_dictionary;

const WORD_SOURCE: &str = "less-less-english-words.txt";

fn time_to_first_guess(i: i32) -> i32 {
    let dictionary = load_dictionary(WORD_SOURCE);
    let candidates = dictionary.clone();
    let guess_full = optimial_guess(&candidates, &dictionary);
    i * 2
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("outcome", |b| b.iter(|| time_to_first_guess(black_box(2))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
