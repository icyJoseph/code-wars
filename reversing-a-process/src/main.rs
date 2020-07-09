use std::convert::TryInto;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
static IMPOSSIBLE: &str = "Impossible to decode";

fn inverse_mod(factor: u32, num: u32) -> u32 {
    (0..26)
        .position(|scale| scale * factor % 26 == num)
        .unwrap()
        .try_into()
        .unwrap()
}

fn char_code(c: char) -> u32 {
    ALPHABET
        .chars()
        .position(|x| x == c)
        .unwrap()
        .try_into()
        .unwrap()
}

fn look_up_char(position: u32) -> char {
    ALPHABET.chars().nth(position as usize).unwrap()
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn decode(s: &str) -> String {
    // your code
    let mut message = String::new();
    let mut num = 0;
    for element in s.chars() {
        if char::is_numeric(element) {
            num = num * 10 + element.to_digit(10).unwrap();
        } else {
            message.push(element);
        }
    }

    if gcd(num, 26) != 1 {
        return IMPOSSIBLE.to_string();
    }

    let factor = num % 26;

    let mut decoded_message = String::new();

    for ch in message.chars() {
        let naive_inv = inverse_mod(factor, char_code(ch));
        let decoded_char = look_up_char(naive_inv);
        decoded_message.push(decoded_char)
    }

    return decoded_message;
}

fn main() {
    println!("{}", decode("6015ekx"));
}

#[test]
fn basic_tests() {
    fn dotest(s: &str, exp: &str) -> () {
        let ans = decode(s);
        println!("{:?}", ans == exp);
        assert_eq!(ans, exp, "Testing: {}", s);
    }
    let mut s = "10559625hbkeohysnztuuqdznnkkcgjndbujej";
    dotest(s, "dtiygdoenhxqqsfhnniimkpnftqpyp");

    s = "1273409kuqhkoynvvknsdwljantzkpnmfgf";
    dotest(s, "uogbucwnddunktsjfanzlurnyxmx");

    s = "761328qockcouoqmoayqwmkkic";
    dotest(s, "Impossible to decode");
}
