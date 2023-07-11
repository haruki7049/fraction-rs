#[cfg(test)]
mod fraction_test {
    use fraction_rs::Fraction;

    #[test]
    fn fraction_new() {
        let _f: Fraction = Fraction {
            denominator: 1,
            numerator: 1,
        };
    }
}
