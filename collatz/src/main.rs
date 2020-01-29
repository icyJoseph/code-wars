use std::collections::HashMap;

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let mut pivot_index = 0;

    for index in 0..slice.len() - 1 {
        if slice[index] < slice[slice.len() - 1] {
            slice.swap(pivot_index, index);
            pivot_index = pivot_index + 1;
        }
    }

    slice.swap(pivot_index, slice.len() - 1);
    pivot_index
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot_index = partition(slice);

    quicksort(&mut slice[..pivot_index]);

    quicksort(&mut slice[pivot_index + 1..]);
}

/// The collatz conjecture says that all natural numbers
/// under the rule  n / 2 or 3 * n + 1, for even and odd
/// respectively, eventually arrive to 1, in which case,
/// any successive operations return to 1.
///
/// Given the decimal expansion of any natural number:
/// n = 10^i*a_i +...+ 10^0*a_0 = ... + a_0
///
/// When n is odd, then a_0, is restricted to 1,3,5,7,9; then
/// it follows that 3 * n + 1 = .... + 3*a_0 + 1
///
/// For the space of possible a_0, the last digit of 3*n + 1
/// is always even, so we can shortcut one step by dividing
/// (3 * n + 1) / 2, and counting the operation as two steps
fn collatz_step((num, step): (i32, i32)) -> (i32, i32) {
    match num % 2 {
        0 => (num / 2, step + 1),
        _ => ((3 * num + 1) / 2, step + 2),
    }
}

/// Calculates the number of steps to arrive from n to 1
fn collatz<'a>(num: &'a i32, map: &mut HashMap<i32, i32>) -> i32 {
    if *num < 1 {
        return *num;
    }

    let mut next = (*num, 0);

    while next.0 != 1 {
        match map.get(&next.0) {
            Some(n) => {
                next = (1, next.1 + n);
                return next.1;
            }
            _ => {
                next = collatz_step(next);
            }
        }
    }

    // stopping time is the number of steps to arrive to 1
    let stopping_time = next.1;
    // the cycle length required is the total number of
    // numbers generated when going from n to 1, including 1
    let cycle_length = stopping_time + 1;

    // save to hashMap that for this number, the steps down to 1 is next.1
    map.insert(*num, cycle_length);

    cycle_length
}

#[test]
fn collatz_test() {
    let mut results: HashMap<i32, i32> = HashMap::new();
    assert_eq!(collatz(&123, &mut results), 47);
    assert_eq!(collatz(&2123, &mut results), 33);
    assert_eq!(collatz(&27, &mut results), 112);
    assert_eq!(collatz(&871, &mut results), 179);
}

// TODO: upgrade to i64
fn main() {
    // get the args
    let args = std::env::args().skip(1);
    let bounds: Vec<i32> = args.take(2).map(|x| x.parse::<i32>().unwrap()).collect();

    // create a HashMap to look up values
    let mut results: HashMap<i32, i32> = HashMap::new();
    // store results as we move up the range
    let mut values: Vec<i32> = Vec::new();

    for num in bounds[0]..=bounds[1] {
        let value = collatz(&num, &mut results);
        values.push(value);
    }

    quicksort(&mut values);

    let last = values.last().unwrap();

    println!("{} {} {}", bounds[0], bounds[1], last);
}
