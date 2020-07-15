fn eq(x: f64) -> f64 {
    1. + (-4. * x).exp() * (0.5f64) - (-2. * x).exp() * (0.5f64)
}

fn ode(x: f64, y: f64, dx: f64) -> f64 {
    (2. - (-4. * x).exp() - 2. * y) * dx
}

fn euler_combinator(
    x: f64,
    y: f64,
    iterations: i32,
    step: f64,
    acc: &mut Vec<(f64, f64)>,
) -> Vec<(f64, f64)> {
    if iterations == 0 {
        return acc.to_vec();
    }

    let next_x = x + step;
    let next_y = y + ode(x, y, step);

    acc.push((next_x, next_y));

    euler_combinator(next_x, next_y, iterations - 1, step, acc)
}

fn euler(x: f64, y: f64, iterations: i32) -> Vec<(f64, f64)> {
    let mut acc: Vec<(f64, f64)> = vec![(x, y)];
    let step = (iterations as f64).recip();

    euler_combinator(x, y, iterations, step, &mut acc)
}

fn truncate(number: f64, limit: i32) -> f64 {
    let factor = (10.0f64).powi(limit);
    (number * factor).trunc() / factor
}

fn ex_euler(nb: i32) -> f64 {
    let approx_solutions: Vec<(f64, f64)> = euler(0., 1., nb);

    let mean: f64 = approx_solutions
        .iter()
        .map(|(xk, yk)| {
            let zk = eq(*xk);
            (yk - zk).abs() / zk
        })
        .fold(0., |prev, curr| prev + curr)
        / (nb as f64 + 1.);

    truncate(mean, 6)
}

fn main() {
    let first = 17;
    let second = 10;
    println!(
        "ex_euler({}) = {}, should be 0.015193",
        first,
        ex_euler(first)
    );
    println!(
        "ex_euler({}) = {}, should be 0.026314",
        second,
        ex_euler(second)
    );
}

#[allow(dead_code)]
fn assert_fuzzy_equals(n: i32, expected: f64) {
    let merr = 1.0e-6;
    let actual = ex_euler(n);
    let inrange = if expected == 0.0 {
        actual.abs() <= merr
    } else {
        (actual - expected).abs() / expected <= merr
    };
    if inrange == false {
        println!(
            " Expected value must be near: {:e} but was:{:e}",
            expected, actual
        );
    }
    assert_eq!(inrange, true);
    println!(" {}", inrange);
    println!("{}", "-")
}

#[test]
fn basic_tests() {
    assert_fuzzy_equals(10, 0.026314);
    assert_fuzzy_equals(17, 0.015193);
    assert_fuzzy_equals(1, 0.5);
}
