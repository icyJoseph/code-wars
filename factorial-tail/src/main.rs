fn primes_up_to(n: i32) -> Vec<i32> {
    let mut sieve: Vec<(bool, i32)> = (0..=n)
        .map(|x| if x < 2 { (false, x) } else { (true, x) })
        .collect();

    for (index, _) in sieve.to_vec().iter().enumerate() {
        let (is_prime, prime) = sieve[index];

        if is_prime {
            let range: Vec<usize> = (1..)
                .take_while(|p| (index + p) * index < sieve.len())
                .collect();

            for element in range {
                sieve[index * (index + element)].0 = false;
            }
        }

        if prime * prime >= n {
            break;
        }
    }

    sieve.iter().filter(|x| x.0).map(|x| x.1).collect()
}

fn decompose_factorial(n: i32, primes: &[i32], base_factors: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut factors: Vec<(i32, i32)> = (2..=n).flat_map(|x| decompose(x, primes)).collect();

    factors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let first_factor = factors.iter().next().unwrap().0;

    factors
        .iter()
        .filter(|x| base_factors.iter().any(|y| y.0 == x.0))
        .fold(vec![], |mut acc, curr| {
            let last = match acc.pop() {
                Some((prime, power)) => (prime, power),
                None => (first_factor, 0),
            };

            if last.0 == curr.0 {
                acc.push((curr.0, curr.1 + last.1));
            } else {
                acc.push((last.0, last.1));
                acc.push((curr.0, curr.1));
            }

            return acc;
        })
        .iter()
        .map(|p| match base_factors.iter().position(|x| x.0 == p.0) {
            Some(index) => (p.0, p.1 / base_factors[index].1),
            None => (p.0, p.1),
        })
        .filter(|p| p.1 != 0)
        .collect()
}

fn decompose(n: i32, primes: &[i32]) -> Vec<(i32, i32)> {
    let mut pivot = n;
    let mut factors = vec![];

    for prime in primes {
        let mut acc = 0;
        if pivot < *prime {
            break;
        }
        while pivot % prime == 0 {
            acc = acc + 1;
            pivot = pivot / prime;
        }
        if acc > 0 {
            factors.push((*prime, acc));
        }
        if pivot == 1 {
            break;
        }
    }

    factors
}

fn zeroes(base: i32, number: i32) -> i32 {
    use std::cmp;
    let primes = primes_up_to(cmp::min(number, base));

    let decomposed_base = decompose(base, &primes);
    let decomposed_factorial = decompose_factorial(number, &primes, &decomposed_base[..]);

    let limit = match decomposed_factorial.iter().map(|x| x.1).min() {
        None => 0,
        Some(x) => x,
    };

    limit
}

fn main() {
    let base = 10;
    let fact = 10;
    println!("zeroes({}, {}) = {}", base, fact, zeroes(base, fact));
    zeroes(128, 3434);
    zeroes(128, 9772);
    zeroes(128, 3442);
    zeroes(128, 7815);
    zeroes(128, 7483);
    zeroes(128, 4793);
    zeroes(17, 100);
    zeroes(170, 100);
    zeroes(221, 100);
}

#[test]
fn test_cases() {
    assert_eq!(zeroes(128, 3434), 489);
    assert_eq!(zeroes(128, 9772), 1395);
    assert_eq!(zeroes(128, 3442), 490);
    assert_eq!(zeroes(128, 7815), 1115);
    assert_eq!(zeroes(128, 7483), 1067);
    assert_eq!(zeroes(128, 4793), 683);
    assert_eq!(zeroes(17, 100), 5);
    assert_eq!(zeroes(170, 100), 5);
    assert_eq!(zeroes(221, 100), 5);
}
