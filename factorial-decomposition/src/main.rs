fn decomp(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    }

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

    let mut result: Vec<String> = vec![];

    let first_factor = factors.iter().next().unwrap().0;
    let aggregated: Vec<(i32, i32)> = factors.iter().fold(vec![], |mut acc, curr| {
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
    });

    for (prime, power) in aggregated.iter() {
        let formatted = match power {
            1 => format!("{}", prime),
            _ => format!("{}^{}", prime, power),
        };

        result.push(formatted);
    }

    return result.join(" * ");
}

fn main() {
    let n = 17;
    println!("{}! = {}", n, decomp(n));
}

#[test]
fn basic_tests() {
    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }

    dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
    dotest(5, "2^3 * 3 * 5");
    dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
    dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
    dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");
}
