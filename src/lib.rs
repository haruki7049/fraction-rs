pub struct Fraction {
    denominator: u32,
    numerator: i32,
}

impl Fraction {
    fn new(d: u32, n: i32) -> Fraction {
        Fraction {
            denominator: d,
            numerator: n,
        }
    }
}
