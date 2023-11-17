extern crate nalgebra as na;
use na::{Matrix3, Matrix4, Scalar, Vector3};

pub fn trans_to_rp<T: Copy>(v: Matrix4<T>) -> (Matrix3<T>, Vector3<T>)
where
    f64: std::convert::From<T>,
    T: Scalar,
{
    (
        v.fixed_view::<3, 3>(0, 0).into(),
        v.column(3).clone_owned().remove_row(3),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // More tests are needed for this function, this is just the example from MR

    // More tests are needed for this function, this is just the example from MR
    #[test]
    fn trans_to_rp_conversion() {
        let test_t = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 3.0, 0.0, 0.0, 0.0, 1.0,
        );
        let test_r = Matrix3::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
        let test_p = Vector3::new(0.0, 0.0, 3.0);
        let (r, p) = trans_to_rp(test_t);
        assert_eq!(r, test_r);
        assert_eq!(p, test_p);
    }
}
