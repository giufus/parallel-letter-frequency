extern crate parallel_letter_frequency;


use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;


fn bench_tiny_parallel(c: &mut Criterion) {
    let tiny = &["a"];
    c.bench_function("bench_tiny_parallel", |b| b.iter(|| parallel_letter_frequency::frequency(tiny, 3)) );
}


fn bench_tiny_parallel_no_mutex(c: &mut Criterion) {
    let tiny = &["a"];
    c.bench_function("bench_tiny_parallel_no_mutex", |b| b.iter(|| parallel_letter_frequency::frequency_no_mutex(tiny, 3)) );
}

fn bench_tiny_sequential(c: &mut Criterion) {
    let tiny = &["a"];
    c.bench_function("bench_tiny_sequential", |b| b.iter(|| frequency(tiny)));
}


fn bench_small_parallel(c: &mut Criterion) {
    let texts = all_texts(1);
    c.bench_function("bench_small_parallel", |b| b.iter(|| parallel_letter_frequency::frequency(&texts, 3)));
}


fn bench_small_parallel_no_mutex(c: &mut Criterion) {
    let texts = all_texts(1);
    c.bench_function("bench_small_parallel_no_mutex", |b| b.iter(|| parallel_letter_frequency::frequency_no_mutex(&texts, 3)));
}


fn bench_small_sequential(c: &mut Criterion) {
    let texts = all_texts(1);
    c.bench_function("bench_small_sequential",|b| b.iter(|| frequency(&texts)));
}


fn bench_large_parallel(c: &mut Criterion) {
    let texts = all_texts(30);
    c.bench_function("bench_large_parallel", |b| b.iter(|| parallel_letter_frequency::frequency(&texts, 3)));
}

fn bench_large_parallel_no_mutex(c: &mut Criterion) {
    let texts = all_texts(30);
    c.bench_function("bench_large_parallel_no_mutex", |b| b.iter(|| parallel_letter_frequency::frequency_no_mutex(&texts, 3)));
}


fn bench_large_sequential(c: &mut Criterion) {
    let texts = all_texts(30);
    c.bench_function("bench_large_sequential", |b| b.iter(|| frequency(&texts)));
}

/// Simple sequential char frequency. Can it be beat?
pub fn frequency(texts: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}

fn all_texts(repeat: usize) -> Vec<&'static str> {
    [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER]
        .iter()
        .cycle()
        .take(3 * repeat)
        .flat_map(|anthem| anthem.iter().cloned())
        .collect()
}

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
pub const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

// Dutch national anthem
pub const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];

// American national anthem
pub const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

criterion_group!(benches, 
    bench_tiny_sequential, bench_tiny_parallel,bench_tiny_parallel_no_mutex, 
    bench_small_sequential, bench_small_parallel,bench_small_parallel_no_mutex, 
    bench_large_sequential, bench_large_parallel,bench_large_parallel_no_mutex, 
);
criterion_main!(benches);
