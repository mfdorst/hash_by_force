use rand::Rng;
use std::num::Wrapping;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut checked = 0;
    let mut total_checked = 0;
    while start.elapsed().as_secs() < 20 {
        let word = random_word();
        let hash = djb2(&word);
        checked += 1;
        total_checked += 1;
        if hash < 1000000000000000 {
            println!("{} -> {:020} checked: {}", word, hash, checked);
            checked = 0;
        }
    }
    let checked_per_second = total_checked / start.elapsed().as_secs();
    println!(
        "{} strings checked in {:.2} seconds. {:.0} checked per second.",
        total_checked,
        start.elapsed().as_secs(),
        checked_per_second
    );
}

fn djb2(string: &str) -> u64 {
    let mut hash = Wrapping(5381u64);
    for c in string.chars() {
        hash = hash * Wrapping(33) + Wrapping(c as u64);
    }
    hash.0
}

fn random_word() -> String {
    (0..32)
        .map(|_| rand::thread_rng().gen_range(b'!'..b'~') as char)
        .collect::<String>()
}
