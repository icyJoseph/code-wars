use std::f64;

fn move_point(point: (f64, f64), d: f64, ang: f64) -> (f64, f64) {
    (point.0 + d * ang.cos(), point.1 + d * ang.sin())
}

fn crusoe(n: i32, d: f64, ang: i32, distmult: f64, angmult: f64) -> (f64, f64) {
    let float_ang = (ang as f64) * std::f64::consts::PI / 180.;

    (0..n).fold((0., 0.), |point, iteration| {
        move_point(
            point,
            distmult.powi(iteration) * d,
            angmult.powi(iteration) * float_ang,
        )
    })
}

fn main() {
    println!("{:?}", crusoe(8, 0.22, 3, 1.01, 1.15));
}

#[test]
fn basic_tests() {
    fn assert_fuzzy(
        n: i32,
        d: f64,
        ang: i32,
        distmult: f64,
        angmult: f64,
        expect: (f64, f64),
    ) -> () {
        let merr = 1e-12;
        let actual = crusoe(n, d, ang, distmult, angmult);
        let inrange = (actual.0 - expect.0).abs() <= merr && (actual.0 - expect.0).abs() <= merr;
        if inrange == false {
            println!("Expected.0 value near: {} but got: {}", expect.0, actual.0);
            println!("Expected.1 value near: {} but got: {}", expect.1, actual.1);
        }
        assert_eq!(inrange, true);
    }
    assert_fuzzy(8, 0.22, 3, 1.01, 1.15, (1.814652098870, 0.164646220964));
    assert_fuzzy(29, 0.13, 21, 1.01, 1.09, (0.318341393410, 2.292862212314));
    assert_fuzzy(45, 0.10, 3, 1.01, 1.10, (2.689897523779, 2.477953232467));
}
