static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            let was_uppercased = c.is_ascii_uppercase();

            match ALPHABET.find(c.to_ascii_lowercase()) {
                Some(e) => {
                    let encoded = match ALPHABET.chars().nth((e + 13) % ALPHABET.len()) {
                        Some(e) => e,
                        None => panic!("Something has gone terribly wrong!"),
                    };

                    if was_uppercased {
                        return encoded.to_ascii_uppercase();
                    }

                    encoded
                }
                None => c,
            }
        })
        .collect()
}

fn main() {
    println!("{}", rot13("Hello, world!"));
}

#[test]
fn test_fixed() {
    assert_eq!(rot13("test"), "grfg");
    assert_eq!(rot13("Test"), "Grfg");
}
