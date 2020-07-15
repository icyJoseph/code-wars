fn fib(n: u64) -> (u64, u64) {
    return fib_iter(1, 0, 0, 1, n);
}

fn fib_iter(a: u64, b: u64, p: u64, q: u64, count: u64) -> (u64, u64) {
    if count == 0 {
        return (a, b);
    } else if count % 2 == 0 {
        return fib_iter(a, b, p * p + q * q, q * q + 2 * p * q, count / 2);
    }
    return fib_iter(b * q + a * q + a * p, b * p + a * q, p, q, count - 1);
}

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut it = 2;

    loop {
        let seq = fib(it);
        let current_prod = seq.0 * (seq.0 + seq.1);

        if current_prod >= prod {
            return (seq.0, seq.0 + seq.1, current_prod == prod);
        }

        it = it + 1;
    }
}

fn main() {
    let prod = 4895;
    let res = product_fib(prod);
    println!(
        "Prod: {}, happens with Fn:{} and Fn+1: {}? {}",
        prod, res.0, res.1, res.2
    );
}

#[test]
fn basics_product_fib() {
    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
