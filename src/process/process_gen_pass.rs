use rand::seq::IndexedRandom;

pub fn process_gen_pass(
    length: u8,
    number: bool,
    lower: bool,
    upper: bool,
    symbol: bool,
) -> String {
    let mut chars = Vec::with_capacity(96);
    let mut result = String::with_capacity(length as usize);

    if number {
        chars.extend_from_slice(b"0123456789");
    }

    if lower {
        chars.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }

    if upper {
        chars.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if symbol {
        chars.extend_from_slice(b"!@#$%^&*");
    }

    for _ in 0..length {
        let mut rng = rand::rng();
        let c = chars.choose(&mut rng).expect("Chars won't be empty");

        result.push(*c as char);
    }

    result
}
