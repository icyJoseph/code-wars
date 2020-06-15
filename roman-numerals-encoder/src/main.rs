use std::convert::TryFrom;

fn dec_to_roman(num: u32) -> String {
    if num >= 4000 {
        panic!("Roman Numerals higher than 3999 are not covered here");
    }

    let bases = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut pivot = num;
    let mut digits: Vec<(u32, u32)> = vec![];

    for base in bases.iter() {
        let qty = pivot / base;
        digits.push((*base, qty));
        pivot = pivot % base;
    }

    let mut result = String::new();

    for (base, qty) in digits.iter() {
        let char = match base {
            1 => "I",
            4 => "IV",
            5 => "V",
            9 => "IX",
            10 => "X",
            40 => "XL",
            50 => "L",
            90 => "XC",
            100 => "C",
            400 => "CD",
            500 => "D",
            900 => "CM",
            1000 => "M",
            _ => "",
        };

        if *qty != 0u32 {
            let repeats = usize::try_from(*qty).unwrap();
            let to_push = char.repeat(repeats);
            result.push_str(&to_push)
        }
    }

    return result;
}

fn main() {
    let result = dec_to_roman(1001);
    println!("Hello, world!, {}", result);
}

#[test]
fn known_roman_numerals() {
    let decimals: Vec<u32> = vec![39, 246, 789, 2421, 160, 207, 1009, 1066, 1954, 2014, 900];
    let expected = vec![
        "XXXIX",
        "CCXLVI",
        "DCCLXXXIX",
        "MMCDXXI",
        "CLX",
        "CCVII",
        "MIX",
        "MLXVI",
        "MCMLIV",
        "MMXIV",
        "CM",
    ];

    let romans: Vec<String> = decimals.iter().map(|dec| dec_to_roman(*dec)).collect();

    assert_eq!(romans, expected);
}
