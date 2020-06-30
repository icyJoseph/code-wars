use std::collections::{HashMap, HashSet};

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
                word.split(" ")
                    .filter_map(|letter| dict.get(letter))
                    .fold("".to_string(), |acc, curr| acc + curr)
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(" ");

        return decoded;
    }

    fn decode_bits(&self, stream: &str) -> String {
        let str_vector = &self.bit_chunks(stream);

        let lengths: HashSet<usize> = str_vector.iter().map(|x| x.len()).collect();

        // strictly speaking, lengths should be L = K + 3 * M + 7 * N
        // where K is the number of the lesser length present in lengths
        // M is the middle length and N is the greatest, that means that lengths should
        // have only 3 elements

        let mut sampling_rate: usize = *lengths.iter().next().unwrap();

        for length in lengths.iter() {
            if *length < sampling_rate {
                sampling_rate = *length;
            }
        }

        let mut unpacked: Vec<&str> = vec![];

        for vector in str_vector.iter() {
            let units = vector.len() / sampling_rate;
            let bit = vector.iter().next().unwrap();

            match units {
                1 => {
                    if bit == &'1' {
                        unpacked.push(".");
                    }
                }
                3 => {
                    if bit == &'1' {
                        unpacked.push("-");
                    } else {
                        unpacked.push(" ");
                    }
                }
                7 => {
                    unpacked.push("   ");
                }
                _ => {
                    println!("Strange unit");
                }
            }
        }

        return unpacked.join("");
    }

    fn bit_chunks(&self, stream: &str) -> Vec<Vec<char>> {
        let trimmed_stream: String = stream
            .trim_start_matches("0")
            .trim_end_matches("0")
            .to_string();

        let mut str_vector: Vec<Vec<char>> = vec![];

        let mut carry: Vec<char> = vec![];
        let mut last = trimmed_stream.chars().next().unwrap();

        for (index, char) in trimmed_stream.chars().enumerate() {
            if char == last {
                carry.push(char);
            } else {
                str_vector.push(carry.clone());
                carry.clear();
                carry.push(char)
            }
            last = char;
            if index == trimmed_stream.len() - 1 {
                str_vector.push(carry.clone());
            }
        }

        return str_vector;
    }

    fn naive_median(&self, lengths: &HashSet<usize>) -> usize {
        let length_of_set = lengths.len();
        let mut to_sort: Vec<usize> = vec![];

        for item in lengths.iter() {
            to_sort.push(*item);
        }

        to_sort.sort();

        let median_index = if length_of_set % 2 == 0 {
            length_of_set / 2
        } else {
            (length_of_set - 1) / 2 + (length_of_set + 1) / 2
        };

        return to_sort[median_index];
    }

    fn decode_bits_advanced(&self, stream: &str) -> String {
        // as a stream is parsed
        // group chunks of zeroes and ones
        // those of zeros are either: space between chars, between words, or end of symbol
        // those of ones are either: dot or dash
        let chunks = &self.bit_chunks(stream);
        // if ever in doubt, assume it is a dot
        let mut maybe_dot_or_dash: Vec<(usize, Vec<char>)> = vec![];
        let mut a_type_of_space: Vec<(usize, Vec<char>)> = vec![];

        for (index, chunk) in chunks.iter().enumerate() {
            match chunk.iter().next().unwrap() {
                '1' => maybe_dot_or_dash.push((index, chunk.to_vec())),
                '0' => a_type_of_space.push((index, chunk.to_vec())),
                _ => panic!("Strange Bit!"),
            }
        }

        let maybe_dot_or_dash_lengths: HashSet<usize> =
            maybe_dot_or_dash.iter().map(|x| x.1.len()).collect();

        let a_type_of_space_lengths: HashSet<usize> =
            a_type_of_space.iter().map(|x| x.1.len()).collect();

        let median = &self.naive_median(&maybe_dot_or_dash_lengths);

        let mut shortest_dot_length: usize = *maybe_dot_or_dash_lengths.iter().next().unwrap();
        for length in maybe_dot_or_dash_lengths.iter() {
            if *length < shortest_dot_length {
                shortest_dot_length = *length;
            }
        }

        let mut longest_dot_length: usize = *maybe_dot_or_dash_lengths.iter().next().unwrap();
        for length in maybe_dot_or_dash_lengths.iter() {
            if *length > longest_dot_length && *length < *median {
                longest_dot_length = *length;
            }
        }

        let mut longest_dash_length: usize = *maybe_dot_or_dash_lengths.iter().next().unwrap();
        for length in maybe_dot_or_dash_lengths.iter() {
            if *length > longest_dash_length {
                longest_dash_length = *length;
            }
        }

        let mut upper_word_space_bound: usize = longest_dash_length + 1;
        for length in a_type_of_space_lengths.iter() {
            if *length > upper_word_space_bound {
                upper_word_space_bound = *length;
            }
        }

        let one_unit_range = shortest_dot_length..=longest_dot_length;
        let three_unit_range = *median..=longest_dash_length;
        let seven_unit_range = longest_dash_length + 1..=upper_word_space_bound;

        // println!("{:?}", one_unit_range);
        // println!("{:?}", three_unit_range);
        // println!("{:?}", seven_unit_range);

        // go through chunks transforming according to ranges
        let mut result: Vec<&str> = vec![];
        for chunk in chunks {
            let chunk_length = chunk.len();

            let bit = chunk.iter().next().unwrap();
            if one_unit_range.contains(&chunk_length) {
                if bit == &'1' {
                    result.push(".");
                }
                continue;
            }

            if three_unit_range.contains(&chunk_length) {
                if bit == &'1' {
                    result.push("-");
                } else {
                    result.push(" ");
                }
                continue;
            }
            if seven_unit_range.contains(&chunk_length) {
                result.push("   ");
                continue;
            }
        }

        return result.join("");
    }
}

pub fn main() {
    let decoder = MorseDecoder::new();
    let message = decoder.decode_morse(".... . -.--   .--- ..- -.. .");
    println!("{}", message);
    println!("{}", decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")));
    println!("{}", decoder.decode_morse(&decoder.decode_bits_advanced("0000000011011010011100000110000001111110100111110011111100000000000111011111111011111011111000000101100011111100000111110011101100000100000")));
}

#[test]
fn test_hey_jude() {
    let decoder = MorseDecoder::new();
    assert_eq!(
        decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
        "HEY JUDE"
    );
}
