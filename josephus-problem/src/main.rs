fn combinator(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    return (combinator(n - 1, k) + k) % n;
}

fn josephus_survivor(n: i32, k: i32) -> i32 {
    return combinator(n, k) + 1;
}

fn main() {
    let survivor = josephus_survivor(131333, 113);
    println!("Survivor: {}", survivor);
}

#[test]
fn basic_tests() {
    assert_eq!(josephus_survivor(7, 3), 4);
    assert_eq!(josephus_survivor(11, 19), 10);
    assert_eq!(josephus_survivor(40, 3), 28);
    assert_eq!(josephus_survivor(14, 2), 13);
    assert_eq!(josephus_survivor(100, 1), 100);
    assert_eq!(josephus_survivor(1, 300), 1);
    assert_eq!(josephus_survivor(2, 300), 1);
    assert_eq!(josephus_survivor(5, 300), 1);
    assert_eq!(josephus_survivor(7, 300), 7);
    assert_eq!(josephus_survivor(300, 300), 265);
}
