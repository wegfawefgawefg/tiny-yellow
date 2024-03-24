static CONSONANTS: [&str; 21] = [
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x",
    "y", "z",
];

static VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

pub fn gen_syllable() -> String {
    let mut syllable = String::new();
    syllable.push_str(CONSONANTS[rand::random::<usize>() % CONSONANTS.len()]);
    syllable.push_str(VOWELS[rand::random::<usize>() % VOWELS.len()]);
    syllable
}

pub fn gen_name(length: usize) -> String {
    let mut name = String::new();
    for _ in 0..length {
        name.push_str(&gen_syllable());
    }
    name
}

pub fn gen_name_of_rand_length() -> String {
    let length = rand::random::<usize>() % 5 + 3;
    gen_name(length)
}
