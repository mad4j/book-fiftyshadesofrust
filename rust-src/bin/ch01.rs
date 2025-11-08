fn is_pangram(s: &str) -> bool {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>()
        .len() == 26
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    println!("{}", is_pangram(text)); // true
}
