fn max_diff(numbers: &[i32]) -> i32 {
    let head: i32 = match numbers.get(0) {
        Some(n) => *n,
        None => 0,
    };

    numbers
        .iter()
        .fold((head, head, 0), |acc, curr| match acc {
            (x, y, _) if *curr <= x => (*curr, y, y - *curr),
            (x, y, _) if *curr >= y => (x, *curr, *curr - x),
            (_, _, _) => acc,
        })
        .2
}

fn main() {
    let arr = [-0, 1, 2, -3, 4, 5, -6];
    println!("Max diff on {:?} is {}.", arr, max_diff(&arr));
}

#[test]
fn test_sample_1() {
    assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 6]), 6);
}

#[test]
fn test_sample_2() {
    assert_eq!(max_diff(&[-0, 1, 2, -3, 4, 5, -6]), 11);
}

#[test]
fn test_sample_3() {
    assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 16]), 16);
}

#[test]
fn test_sample_4() {
    assert_eq!(max_diff(&[16]), 0);
}

#[test]
fn test_sample_5() {
    assert_eq!(max_diff(&[]), 0);
}
