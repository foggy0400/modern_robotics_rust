extern crate nalgebra as na;
use crate::bcat;
use crate::concat::{allocate_block_output, Block, HCat, VCat};
use crate::groups::ToSo3;
use crate::interfaces::Numeric;
use na::{Matrix3, Matrix4, Matrix6, Scalar, Vector3};
use num::traits::Zero;

pub fn vec_to_se3<T: Copy>(omega: &[T; 6]) -> Matrix4<f64>
where
    f64: std::convert::From<T>,
{
    return Matrix4::new(
        0.0,
        -f64::from(omega[2]),
        f64::from(omega[1]),
        f64::from(omega[3]),
        f64::from(omega[2]),
        0.0,
        -f64::from(omega[0]),
        f64::from(omega[4]),
        -f64::from(omega[1]),
        f64::from(omega[0]),
        0.0,
        f64::from(omega[5]),
        0.0,
        0.0,
        0.0,
        0.0,
    );
}

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

pub fn ad<T: Numeric<T>>(v: [T; 6]) -> Matrix6<T> {
    let omega = Vector3::from_row_slice(&v[0..3]).to_so3();
    let vmat = Vector3::from_row_slice(&v[3..6]).to_so3();
    let zeros = Matrix3::zeros();
    return bcat![omega.0, zeros;
                vmat.0, omega.0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn se3() {
        let test_mat = Matrix4::new(
            0.0, -3.3, 2.2, 4.4, 3.3, 0.0, -1.1, 5.5, -2.2, 1.1, 0.0, 6.6, 0.0, 0.0, 0.0, 0.0,
        );
        let vec: [f64; 6] = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let res = vec_to_se3(&vec);
        assert_eq!(res, test_mat);
    }

    #[test]
    fn se3_conversion() {
        let test_mat = Matrix4::new(
            0.0, -3.0, 2.0, 4.0, 3.0, 0.0, -1.0, 5.0, -2.0, 1.0, 0.0, 6.0, 0.0, 0.0, 0.0, 0.0,
        );
        let vec: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let res = vec_to_se3(&vec);
        assert_eq!(res, test_mat);
    }

    #[test]
    fn se3_parameter() {
        let test_mat = Matrix4::new(
            0.0, -3.0, 2.0, 4.0, 3.0, 0.0, -1.0, 5.0, -2.0, 1.0, 0.0, 6.0, 0.0, 0.0, 0.0, 0.0,
        );
        let res = vec_to_se3(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(res, test_mat);
    }

    // More tests are needed for this function, this is just the example from MR
    #[test]
    fn ad_conversion() {
        let test_vec = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let res = ad(test_vec);
        let test_mat = Matrix6::new(
            0.0, -3.0, 2.0, 0.0, 0.0, 0.0, 3.0, 0.0, -1.0, 0.0, 0.0, 0.0, -2.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, -6.0, 5.0, 0.0, -3.0, 2.0, 6.0, 0.0, -4.0, 3.0, 0.0, -1.0, -5.0, 4.0, 0.0,
            -2.0, 1.0, 0.0,
        );
        assert_eq!(res, test_mat);
    }

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
