use rand::{
    Rng,
    distr::uniform::{SampleRange, SampleUniform},
};

fn main() {
    // constants
    const LENGTH_PASS: u8 = 16;
    const LENGTH_GROUP: u8 = LENGTH_PASS / 4;

    // ranges: alphabet, numbers, special characters
    let uppercase_symbols: Vec<_> = ('A'..='Z').collect();
    let lowercase_symbols: Vec<_> = ('a'..='z').collect();
    let numeric_symbols: Vec<_> = ('0'..='9').collect();
    let special_symbols = ['!', '@', '#', '$', '*', '_'];

    // generate password with all groups - uppercase, lowercase, special characters, numbers
    let mut password = String::new();
    for _ in 0..LENGTH_GROUP {
        let uppercase_char = uppercase_symbols[generate_random_number(0..uppercase_symbols.len())];
        let lowercase_char = lowercase_symbols[generate_random_number(0..lowercase_symbols.len())];
        let numeric_char = numeric_symbols[generate_random_number(0..numeric_symbols.len())];
        let special_char = special_symbols[generate_random_number(0..special_symbols.len())];

        password.push(uppercase_char);
        password.push(lowercase_char);
        password.push(numeric_char);
        password.push(special_char);
    }

    // shuffle characters
    let password = shuffle(&password);
    println!("{}", password);
}

fn shuffle(s: &str) -> String {
    let mut characters: Vec<_> = s.chars().collect();

    for i in (1..characters.len()).rev() {
        let j = generate_random_number(0..=i);
        characters.swap(i, j);
    }

    characters.into_iter().collect()
}

fn generate_random_number<T: SampleUniform, U: SampleRange<T>>(range: U) -> T {
    let mut rng = rand::rng();
    rng.random_range(range)
}
