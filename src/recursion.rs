
pub fn fac_iterative(n: u32) -> u32 {
    let mut acc = 1;
    for i in 1..(n + 1) {
        acc = acc * i;
    }
    acc
}

pub fn fac_recursive(n: u32) -> u32 {
    match n {
        0 => 1,
        n => n * fac_recursive(n - 1)
    }
}


#[cfg(test)]
mod test {
    use super::fac_iterative;
    use super::fac_recursive;

    #[test]
    fn basics() {
        assert_eq!(fac_iterative(0), 1);
        assert_eq!(fac_iterative(1), 1);
        assert_eq!(fac_iterative(5), 120);
        assert_eq!(fac_iterative(10), 3628800);

        assert_eq!(fac_recursive(0), 1);
        assert_eq!(fac_recursive(1), 1);
        assert_eq!(fac_recursive(5), 120);
        assert_eq!(fac_recursive(10), 3628800);

    }
}
