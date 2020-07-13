use std::convert::TryInto;

fn count_kprimes(k: i32, start: i32, end: i32) -> Vec<i32> {
    let mut sieve: Vec<(i32, i32)> = (start..=end).map(|x| (0, x)).collect();

    let exp: u32 = k.try_into().unwrap();
    let range: Vec<i32> = (2..)
        .take_while(|x| match (2i32.pow(exp - 1)).checked_mul(*x) {
            Some(n) => n < end,
            None => false,
        })
        .collect();

    for (index, _) in sieve.to_vec().iter().enumerate() {
        let mut pivot = sieve[index].1;
        for num in &range[..] {
            while pivot % num == 0 {
                sieve[index].0 = sieve[index].0 + 1;
                pivot = pivot / num;
            }

            if sieve[index].0 > k {
                break;
            }
        }
    }

    return sieve.iter().filter(|x| x.0 == k).map(|x| x.1).collect();
}

fn puzzle(s: i32) -> i32 {
    return s;
}

fn main() {
    println!("8-Primes: {:?}", count_kprimes(8, 10000000, 10000200));
    println!("Sum of primes + 3-prime + 7-prime: {}", puzzle(143));
}

#[test]
fn basics_count_kprimes() {
    fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: Vec<i32>) -> () {
        assert_eq!(count_kprimes(k, start, nd), exp)
    }

    testing_count_kprimes(
        5,
        1000,
        1100,
        vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100],
    );
    testing_count_kprimes(12, 100000, 100100, vec![]);
}

#[test]
fn basics_puzzle() {
    fn testing(n: i32, exp: i32) -> () {
        assert_eq!(puzzle(n), exp)
    }
    testing(100, 0);
    testing(144, 0);
    testing(143, 2);
}
