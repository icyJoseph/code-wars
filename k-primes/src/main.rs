use std::collections::HashSet;
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
    let lists: Vec<Vec<i32>> = [1, 3, 7].iter().map(|k| count_kprimes(*k, 1, s)).collect();
    let primes: Vec<i32> = lists[0].iter().map(|x| *x).collect();
    let three: Vec<i32> = lists[1].iter().map(|x| *x).collect();
    let seven: Vec<i32> = lists[2].iter().map(|x| *x).collect();

    let primes_set: HashSet<&i32> = primes.iter().clone().collect();
    let three_set: HashSet<&i32> = three.iter().clone().collect();
    let seven_set: HashSet<&i32> = seven.iter().clone().collect();
    let three_len = three_set.len();
    let seven_len = seven_set.len();

    let mut p7 = 0;
    let mut p3 = 0;
    let mut p1 = 0;

    let mut number_of_solutions = 0;
    if seven_len == 0 {
        return number_of_solutions;
    }

    loop {
        // find any solution
        loop {
            if p7 > seven_len - 1 {
                break;
            }

            let sum = seven[p7] + three[p3];

            if sum >= s {
                p3 = p3 + 1;
                if p3 == three_len - 1 {
                    p3 = 0;
                    p7 = p7 + 1;
                    if p7 == seven_len - 1 {
                        break;
                    }
                }
            }

            let candidate = s - sum;

            if primes_set.contains(&candidate) {
                p1 = primes.iter().position(|x| *x == candidate).unwrap();
                number_of_solutions = number_of_solutions + 1;
                break;
            } else {
                p3 = p3 + 1;
                if p3 == three_len - 1 {
                    p3 = 0;
                    p7 = p7 + 1;
                    if p7 == seven_len - 1 {
                        break;
                    }
                }
            }
        }

        if number_of_solutions == 0 {
            return number_of_solutions;
        }

        let mut step = 1;
        loop {
            // if the next 3 prime has delta respect to the current
            // with the 7 prime being constant, the next prime has to
            // account for the delta
            let delta = three[p3 + step] - three[p3];
            let candidate = primes[p1] - delta;
            if candidate % 2 == 0 && candidate != 2 {
                step = step + 1;
                if p3 + step == three_len - 1 {
                    break;
                }
                continue;
            }
            if candidate < 0 {
                break;
            }

            if primes_set.contains(&candidate) {
                p1 = primes.iter().position(|x| *x == candidate).unwrap();
                number_of_solutions = number_of_solutions + 1;
                p3 = p3 + step;
                step = 1;
            } else {
                step = step + 1;
                if p3 + step == three_len - 1 {
                    break;
                }
            }
        }

        p7 = p7 + 1;
        p3 = 0;

        if p7 > seven_len - 1 {
            break;
        }
    }

    return number_of_solutions;
}

fn main() {
    println!("8-Primes: {:?}", count_kprimes(8, 10000000, 10000200));
    println!("Sum of primes + 3-prime + 7-prime: {}", puzzle(143));
    println!("Sum of primes + 3-prime + 7-prime: {}", puzzle(300));
    println!("Sum of primes + 3-prime + 7-prime: {}", puzzle(220));
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
    testing(300, 10);
    testing(220, 6);
}
