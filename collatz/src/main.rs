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

fn collatz(num: i32) -> i32 {
    match num % 2 {
        0 => num / 2,
        _ => 3 * num + 1,
    }
}

fn main() {
    let mut arr: [i32; 6] = [2, 1, 4, 3, 9, 7];

    println!("Before {:?}", arr);

    quicksort::<i32>(&mut arr[..]);

    println!("Sorted {:?}", arr);

    println!("collatz {}", collatz(32));
}
