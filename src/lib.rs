#[derive(Debug)]
#[allow(dead_code)]
pub struct Fraction {
    pub denominator: u32,
    pub numerator: i32,
}

impl Default for Fraction {
    fn default() -> Fraction {
        Fraction {
            denominator: 1,
            numerator: 1,
        }
    }
}
