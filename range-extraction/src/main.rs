mod solution {
    struct Node {
        prev_delta: i32,
        next_delta: i32,
        value: i32,
    }

    pub fn range_extraction(a: &[i32]) -> String {
        let iter = a
            .iter()
            .scan((0, 0), |state, &value| {
                let prev_delta = value - state.0;

                let next_delta = match a.get(state.1 + 1) {
                    Some(n) => n - value,
                    None => value,
                };

                *state = (value, state.1 + 1);

                Some(Node {
                    prev_delta,
                    value,
                    next_delta,
                })
            })
            .fold("".to_string(), |state, node| {
                if node.prev_delta == 1 && node.next_delta != 1 {
                    let last = state.chars().last().unwrap();
                    if last == '-' {
                        format!("{}{}", state, node.value)
                    } else {
                        format!("{},{}", state, node.value)
                    }
                } else if node.prev_delta == 1 {
                    let last = state.chars().last().unwrap();
                    if last == '-' {
                        state
                    } else {
                        format!("{}-", state)
                    }
                } else if state.is_empty() {
                    format!("{}", node.value)
                } else {
                    format!("{},{}", state, node.value)
                }
            });

        return iter;
    }
}

fn main() {
    let range = [
        -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20,
    ];
    let extracted_range = solution::range_extraction(&range);
    println!("Extracted Range from: {:?} is: {}", range, extracted_range);
}

#[test]
fn example() {
    assert_eq!(
        "-6,-3-1,3-5,7-11,14,15,17-20",
        solution::range_extraction(&[
            -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
        ])
    );
    assert_eq!(
        "-3--1,2,10,15,16,18-20",
        solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
    );
}
