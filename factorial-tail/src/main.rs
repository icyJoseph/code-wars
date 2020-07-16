fn decompose_factorial(n: i32) -> Vec<(i32, i32)> {
    let mut sieve: Vec<(bool, i32, Vec<(i32, i32)>)> = (0..=n)
        .map(|x| {
            if x < 2 {
                (false, x, vec![])
            } else {
                (true, x, vec![])
            }
        })
        .collect();

    for (index, _) in sieve.to_vec().iter().enumerate() {
        let (is_prime, prime, _) = sieve[index];

        if is_prime {
            let range: Vec<usize> = (1..).take_while(|p| p * index < sieve.len()).collect();

            for element in range {
                sieve[element * index].0 = false;

                let mut value = sieve[element * index].1;
                let mut acc = 0;

                while value % prime == 0 {
                    acc = acc + 1;
                    value = value / prime;
                }

                sieve[element * index].2.push((prime, acc));
            }
        }
    }

    let mut factors: Vec<&(i32, i32)> = sieve.iter().flat_map(|(_, _, fact)| fact).collect();

    factors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let first_factor = factors.iter().next().unwrap().0;

    factors.iter().fold(vec![], |mut acc, curr| {
        let last = match acc.pop() {
            Some((prime, power)) => (prime, power),
            None => (first_factor, 0),
        };

        if last.0 == curr.0 {
            acc.push((curr.0, curr.1 + last.1));
        } else {
            acc.push(last);
            acc.push((curr.0, curr.1));
        }

        return acc;
    })
}

fn decompose(n: i32) -> Vec<(i32, i32)> {
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

    let mut pivot = n;
    let mut factors = vec![];

    for prime in sieve.iter().filter(|x| x.0).map(|x| x.1) {
        let mut acc = 0;
        while pivot % prime == 0 {
            acc = acc + 1;
            pivot = pivot / prime;
        }
        if acc > 0 {
            factors.push((prime, acc));
        }
        if pivot == 1 {
            break;
        }
    }
    factors
}

fn zeroes(base: i32, number: i32) -> i32 {
    println!("b: {}, n: {}", base, number);

    let decomposed_factorial = decompose_factorial(number);
    let decomposed_base = decompose(base);

    let mut limit = number - 1;

    for factor in decomposed_base.iter() {
        match decomposed_factorial.iter().position(|x| x.0 == factor.0) {
            Some(index) => {
                let component = decomposed_factorial[index].1;
                let ratio = component / factor.1;
                if ratio < limit {
                    limit = ratio;
                }
            }
            None => {
                limit = 0;
            }
        }
    }

    limit
}

fn main() {
    println!("Hello, world!");
}
