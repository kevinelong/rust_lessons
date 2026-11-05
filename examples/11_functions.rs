//! Section 11, running: the deck's function exercises.
//!
//! `cargo run --example 11_functions`

use rust_lessons::section_11_functions::{
    average_roll, chart_average, max, min, reverse_me, roll, track_average, Rng,
};

fn main() {
    println!("rust_lessons {} — section 11", rust_lessons::VERSION);

    let values = [13, 2, 4, 7, 93];
    println!("  values {values:?}: min {:?} max {:?}", min(&values), max(&values));

    let mut rng = Rng::new(2026);
    println!("  roll(3, 6)      = {}", roll(3, 6, &mut rng));
    println!("  average_roll(2, 6, 1000) = {:.2}", average_roll(2, 6, 1000, &mut rng));

    println!();
    println!("  track_average(1, 6, 600), then chart_average:");
    let tally = track_average(1, 6, 600, &mut rng);
    print!("{}", chart_average(&tally, 30));

    println!();
    for s in ["abc", "héllo", "racecar"] {
        println!("  reverse_me({s:?}) = {:?}", reverse_me(s));
    }

    // The same seed replays the same rolls, which is why the dice exercises are testable.
    let mut a = Rng::new(7);
    let mut b = Rng::new(7);
    println!();
    println!("  seeded replay: {:?}", (0..5).map(|_| roll(1, 6, &mut a)).collect::<Vec<_>>());
    println!("  seeded replay: {:?}", (0..5).map(|_| roll(1, 6, &mut b)).collect::<Vec<_>>());
}
