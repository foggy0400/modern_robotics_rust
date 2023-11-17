pub fn near_zero<T: PartialOrd>(x: T, eps: Option<T>) -> bool
where
    f64: std::convert::From<T>,
{
    match eps {
        Some(ep) => f64::from(x) < f64::from(ep),
        None => f64::from(x) < 1e-6,
    }
}

pub fn near_zero_float(x: f64, eps: Option<f64>) -> bool {
    match eps {
        Some(ep) => x < ep,
        None => x < 1e-6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_zero_default() {
        assert!(near_zero(1e-7, None))
    }

    #[test]
    fn near_zero_custom() {
        assert!(near_zero(1e-5, Some(1e-3)))
    }

    #[test]
    fn near_zero_f64() {
        assert!(near_zero_float(1e-7, None))
    }

    #[test]
    fn near_zero_fail() {
        assert!(!near_zero(3, None))
    }
}
