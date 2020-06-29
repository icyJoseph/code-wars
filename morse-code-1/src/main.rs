use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    fn new() -> Self {
        // this bit came predefined in the Code Wars Kata
        let mut morse_code: HashMap<String, String> = HashMap::new();
        morse_code.insert(".".to_string(), "E".to_string());

        morse_code.insert("..".to_string(), "I".to_string());
        morse_code.insert(".-".to_string(), "A".to_string());

        morse_code.insert("...".to_string(), "S".to_string());
        morse_code.insert("..-".to_string(), "U".to_string());

        morse_code.insert(".-.".to_string(), "R".to_string());
        morse_code.insert(".--".to_string(), "W".to_string());

        morse_code.insert("....".to_string(), "H".to_string());
        morse_code.insert("...-".to_string(), "V".to_string());
        morse_code.insert("..-.".to_string(), "F".to_string());

        morse_code.insert(".-..".to_string(), "L".to_string());
        morse_code.insert(".--.".to_string(), "P".to_string());
        morse_code.insert(".---".to_string(), "J".to_string());

        morse_code.insert("-".to_string(), "T".to_string());

        morse_code.insert("-.".to_string(), "N".to_string());
        morse_code.insert("--".to_string(), "M".to_string());

        morse_code.insert("-..".to_string(), "D".to_string());
        morse_code.insert("-.-".to_string(), "K".to_string());

        morse_code.insert("--.".to_string(), "G".to_string());
        morse_code.insert("---".to_string(), "O".to_string());

        morse_code.insert("-...".to_string(), "B".to_string());
        morse_code.insert("-..-".to_string(), "X".to_string());
        morse_code.insert("-.-.".to_string(), "C".to_string());
        morse_code.insert("-.--".to_string(), "Y".to_string());

        morse_code.insert("--..".to_string(), "Z".to_string());
        morse_code.insert("--.-".to_string(), "Q".to_string());

        morse_code.insert(".----".to_string(), "1".to_string());
        morse_code.insert("..---".to_string(), "2".to_string());
        morse_code.insert("...--".to_string(), "3".to_string());
        morse_code.insert("....-".to_string(), "4".to_string());
        morse_code.insert(".....".to_string(), "5".to_string());
        morse_code.insert("-....".to_string(), "6".to_string());
        morse_code.insert("--...".to_string(), "7".to_string());
        morse_code.insert("---..".to_string(), "8".to_string());
        morse_code.insert("----.".to_string(), "9".to_string());
        morse_code.insert("-----".to_string(), "0".to_string());

        morse_code.insert("...---...".to_string(), "SOS".to_string());

        return MorseDecoder { morse_code };
    }

    fn decode_morse(&self, encoded: &str) -> String {
        let dict = &self.morse_code;

        let decoded: String = encoded
            .trim()
            .split("   ")
            .into_iter()
            .map(|word| {
                word.split(' ')
                    .filter_map(|letter| dict.get(letter))
                    .fold("".to_string(), |acc, curr| acc + curr)
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(" ");

        return decoded;
    }
}

pub fn main() {
    let decoder = MorseDecoder::new();
    let message = decoder.decode_morse(".... . -.--   .--- ..- -.. .");
    println!("{}", message);
}

#[test]
fn test_hey_jude() {
    let decoder = MorseDecoder::new();
    assert_eq!(
        decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
        "HEY JUDE"
    );
}
