fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut vector = vec![];

    for item in sequence {
        vector.push(item)
    }

    vector.dedup();

    vector
}

fn main() {
    let seq1 = [11, 1, 1, 1, 2, 3, 3, 3, 3, 4, 4, 3, 5];
    let seq2 = "AAAABBBCCDAABBB";
    println!(
        "Sequence: {:?} in unique order: {:?}",
        seq1,
        unique_in_order(seq1.iter())
    );
    println!(
        "Sequence: {:?} in unique order: {:?}",
        seq2,
        unique_in_order(seq2.chars())
    );
}

#[test]
fn sample_test() {
    assert_eq!(
        unique_in_order("AAAABBBCCDAABBB".chars()),
        vec!['A', 'B', 'C', 'D', 'A', 'B']
    );
}
