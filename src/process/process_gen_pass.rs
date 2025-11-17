use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::{Score, zxcvbn};

const NUMBER: &[u8] = b"0123456789";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOL: &[u8] = b"!@#$%^&*";

pub fn process_gen_pass(
    length: u8,
    number: bool,
    lower: bool,
    upper: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut chars = Vec::with_capacity(96);
    let mut result = Vec::with_capacity(length as usize);
    let mut rng = rand::rng();

    if number {
        chars.extend_from_slice(NUMBER);
        result.push(*chars.choose(&mut rng).expect("Chars won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        result.push(*chars.choose(&mut rng).expect("Chars won't be empty"));
    }

    if upper {
        chars.extend_from_slice(UPPER);
        result.push(*chars.choose(&mut rng).expect("Chars won't be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        result.push(*chars.choose(&mut rng).expect("Chars won't be empty"));
    }

    for _ in 0..((length as usize) - result.len()) {
        let c = chars.choose(&mut rng).expect("Chars won't be empty");
        result.push(*c);
    }

    result.shuffle(&mut rng);
    let password = String::from_utf8(result)?;
    check_password_strength(&password);

    Ok(password)
}

pub fn check_password_strength(password: &str) {
    let estimate = zxcvbn(password, &[]);

    eprintln!("Password: {}", password);
    eprintln!("Score: {}/4", estimate.score()); // 0-4 分
    eprintln!("Crack time: {:?}", estimate.crack_times());

    match estimate.score() {
        Score::Zero | Score::One | Score::Two => eprintln!("Weak password"),
        Score::Three => eprintln!("Medium password"),
        Score::Four => eprintln!("Strong password"),
        _ => unreachable!(),
    }

    if let Some(feedback) = estimate.feedback() {
        if let Some(warning) = &feedback.warning() {
            eprintln!("Warning: {}", warning);
        }
        for suggestion in feedback.suggestions() {
            eprintln!("Suggestion: {}", suggestion);
        }
    }
}
