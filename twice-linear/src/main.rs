use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::convert::TryInto;

fn dbl_linear(n: u32) -> u32 {
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    let nsize: usize = (n).try_into().unwrap();

    let mut index: usize = 0;
    let mut prev = 0;

    loop {
        let first = match heap.pop() {
            Some(x) => x,
            None => Reverse(1),
        };

        if prev == first.0 {
            continue;
        }

        if nsize == index {
            return first.0;
        }

        heap.push(Reverse(first.0 * 2 + 1));
        heap.push(Reverse(first.0 * 3 + 1));

        index = index + 1;
        prev = first.0;
    }
}

fn main() {
    let num = 60000;
    println!(
        "dbl_linear({}) is {}, expected: {}",
        num,
        dbl_linear(num),
        1511311
    );
}

#[test]
fn basics_dbl_linear() {
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    testing(10, 22);
    testing(20, 57);
    testing(30, 91);
    testing(50, 175);
    testing(100, 447);
    testing(500, 3355);
    testing(1000, 8488);
    testing(2000, 19773);
    testing(6000, 80914);
    testing(60000, 1511311);
}
