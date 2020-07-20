fn count_patterns(start: char, limit: i32) -> i32 {
    if limit < 2 {
        return limit;
    }

    if limit > 9 {
        return 0;
    };

    let as_number = start as i32 - 64;

    use std::convert::TryInto;
    let limit_u32: u32 = limit.try_into().unwrap();

    let lower_bound = as_number * (10i32).pow(limit_u32 - 1);
    let upper_bound = lower_bound + (10i32).pow(limit_u32 - 1);

    println!("{}, {}", lower_bound, upper_bound);
    let patterns = (lower_bound..upper_bound)
        .filter(|num| {
            let as_chars: Vec<char> = num.to_string().chars().collect();
            return !as_chars.contains(&'0');
        })
        .filter(|num| {
            use std::collections::HashSet;
            let unique: HashSet<char> = num.to_string().chars().collect();
            return unique.len() == num.to_string().chars().collect::<Vec<char>>().len();
        })
        .filter(|num| {
            println!("{}", num);
            let as_string = num.to_string();
            let indeces: std::ops::Range<usize> = 0..(as_string.len() - 1);

            for current_index in indeces {
                let next_index = current_index + 1;
                let current = as_string.chars().nth(current_index).unwrap();
                let next = as_string.chars().nth(next_index).unwrap();

                if current == '5' {
                    continue;
                }

                let current_digit = current.to_digit(10).unwrap();
                let next_digit = next.to_digit(10).unwrap();

                let max = std::cmp::max(current_digit, next_digit);
                let min = std::cmp::min(current_digit, next_digit);

                let req = match (max, min) {
                    (9, 7) => '8',
                    (6, 4) => '5',
                    (9, 1) => '5',
                    (7, 3) => '5',
                    (8, 2) => '5',
                    (7, 1) => '4',
                    (9, 3) => '6',
                    (3, 1) => '2',
                    (_, _) => '0',
                };

                if req == '0' {
                    continue;
                }

                let slice = &as_string[0..current_index];

                if slice.contains(req) {
                    continue;
                } else {
                    return false;
                }
            }
            return true;
        })
        .collect::<Vec<i32>>();

    println!("{:?}", patterns);
    return patterns.len().try_into().unwrap();
}

fn main() {
    let start = 'E';
    let size = 8;

    println!(
        "count_patterns({}, {}) = {}",
        start,
        size,
        count_patterns(start, size)
    );
}

#[test]
fn basic_tests() {
    assert_eq!(count_patterns('A', 0), 0);
    assert_eq!(count_patterns('A', 10), 0);
    assert_eq!(count_patterns('B', 1), 1);
    assert_eq!(count_patterns('C', 2), 5);
    assert_eq!(count_patterns('D', 3), 37);
    assert_eq!(count_patterns('E', 4), 256);
    assert_eq!(count_patterns('E', 8), 23280);
}
