fn reduce_exp_tower(exponents: Vec<u64>) -> u64 {
    let mut reduced = 1;
    for &exponent in exponents.iter().rev() {
        if exponent == 1 {
            reduced = 1;
        } else if exponent == 0 && reduced == 0 {
            reduced = 1;
        } else if exponent == 0 {
            reduced = 0;
        } else if reduced == 0 {
            reduced = 1;
        } else {
            reduced = 2 + exponent % 2;
        }
    }
    reduced
}

fn calc_exp(base: u64, reduced_exp: u64) -> u64 {
    if base == 0 && reduced_exp == 0 {
        return 1;
    } else if base == 0 {
        return 0;
    } else {
        (0..reduced_exp).fold(1, |acc, _| (acc * base) % 4) + 4
    }
}

fn last_digit(lst: &[u64]) -> u64 {
    if lst.len() == 0 {
        return 1;
    }

    let mut iterable = lst.iter();

    let base = match iterable.next() {
        Some(&n) => n,
        None => 1,
    };

    let first_exp = match iterable.next() {
        Some(&n) => n,
        None => 1,
    };

    let exponents_tower = iterable.map(|&x| x).collect();

    let exp = calc_exp(first_exp, reduce_exp_tower(exponents_tower));

    (base % 10).pow((exp) as u32) % 10
}

fn main() {
    let lst = vec![7, 6, 21];
    let expected = 1;
    println!(
        "last digit on: {:?} is {}, expect: {}",
        lst,
        last_digit(&lst),
        expected
    );
}

#[test]
fn basic_tests() {
    let tests = vec![
        (vec![], 1),
        (vec![0, 0], 1),
        (vec![0, 0, 0], 0),
        (vec![2, 2, 0, 1, 0, 2, 1, 1, 0, 0], 2),
        (vec![2, 0, 1, 1, 1, 0, 1, 0, 0, 0], 1),
        (vec![12, 18], 4),
        (vec![1, 2], 1),
        (vec![3, 4, 5], 1),
        (vec![4, 3, 6], 4),
        (vec![7, 6, 21], 1),
        (vec![12, 30, 21], 6),
        (vec![2, 2, 2, 0], 4),
        (vec![937640, 767456, 981242], 0),
        (vec![123232, 694022, 140249], 6),
        (vec![499942, 898102, 846073], 6),
        (vec![3, 3, 1], 7),
        (
            vec![
                256963, 724947, 27675, 354924, 68370, 586163, 57257, 289869, 192752, 21405,
            ],
            7,
        ),
        (vec![2, 0, 1], 1),
        (
            vec![
                625454, 840728, 127008, 387374, 417708, 606135, 498832, 649978, 413053, 414533,
            ],
            6,
        ),
    ];

    for test in tests {
        assert_eq!(last_digit(&test.0), test.1);
    }
}
