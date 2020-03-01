use std::collections::HashMap;

/// Find number of divisors, but not the actual values
/// by finding the prime factors and how many times
/// these occur.
fn find_number_of_divisors(mut n: u64) -> u64 {
    let mut primes: HashMap<u64, u64> = HashMap::new();
    // flush out all powers of two contained in number
    while n % 2 == 0 {
        let curr = match primes.get(&2) {
            Some(val) => *val,
            None => 0,
        };

        primes.insert(2, curr + 1);

        n = n / 2;
    }

    let mut pivot = 3;

    while pivot * pivot <= n {
        // flush out the current pivot powers
        while n % pivot == 0 {
            let curr = match primes.get(&pivot) {
                Some(val) => *val,
                None => 0,
            };

            primes.insert(pivot, curr + 1);

            n = n / pivot;
        }
        // don't need to do even numbers
        pivot = pivot + 2;
    }
    // primes greater than 2
    if n > 2 {
        primes.insert(n, 1);
    }

    let mut number_of_divisors = 1;

    for (_key, val) in primes.iter() {
        // Magic. Given n = (a ^ 2) * (b ^ 3), with, a and b prime
        // then n has (2 + 1) * (3 + 1) divisors, including 1 and n
        number_of_divisors = number_of_divisors * (val + 1);
    }

    number_of_divisors
}

/// It is known that if the amount of divisors is odd
/// then the number is a perfect square
fn is_perfect_square(x: u64) -> bool {
    let divisors = find_number_of_divisors(x);
    divisors % 2 != 0
}

/// Raw and simple way to find actual divisors
fn find_divisors(x: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let mut pivot = 1;

    while pivot * pivot <= x {
        if x % pivot == 0 {
            divisors.push(pivot);

            let comp = x / pivot;

            if pivot != comp {
                divisors.push(comp);
            }
        }

        pivot = pivot + 1;
    }

    divisors
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    // your code
    let mut results: Vec<(u64, u64)> = Vec::new();

    for num in m..(n + 1) {
        // find actual divisors
        let divisors: Vec<u64> = find_divisors(num);
        // calculate the addition
        let sum_of_divisor_squares = divisors.iter().fold(0, |acc, curr| curr * curr + acc);

        // test if the result is perfect square
        if is_perfect_square(sum_of_divisor_squares) {
            results.push((num, sum_of_divisor_squares))
        }
    }

    results
}

fn main() {
    let results = list_squared(1, 21352);
    println!("{:?}", results);
}

#[test]
fn test_list_squared() {
    let expected: Vec<(u64, u64)> = vec![
        (1, 1),
        (42, 2500),
        (246, 84100),
        (287, 84100),
        (728, 722500),
        (1434, 2856100),
        (1673, 2856100),
        (1880, 4884100),
        (4264, 24304900),
        (6237, 45024100),
        (9799, 96079204),
        (9855, 113635600),
        (18330, 488410000),
        (21352, 607622500),
        (21385, 488410000),
        (24856, 825412900),
        (36531, 1514610724),
        (39990, 2313610000),
        (46655, 2313610000),
        (57270, 4747210000),
        (66815, 4747210000),
        (92664, 13011964900),
        (125255, 16430112400),
        (156570, 35532250000),
        (182665, 35532250000),
        (208182, 60762250000),
        (212949, 51437332804),
    ];

    let results = list_squared(1, 212949);

    assert_eq!(results, expected);
}
