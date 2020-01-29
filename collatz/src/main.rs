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

fn collatz<'a>(num: &'a i32) -> i32 {
    let mut next = (*num, 0);
    while next.0 != 1 {
        next = collatz_step(next)
    }
    next.1
}

fn main() {
    let mut arr: [i32; 6] = [2, 1, 4, 3, 9, 7];

    let num = 27;

    println!("Before {:?}", arr);
    quicksort::<i32>(&mut arr[..]);
    println!("Sorted {:?}", arr);
    println!("Steps collatz {}", collatz(&num));
}
