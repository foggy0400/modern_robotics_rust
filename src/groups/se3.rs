extern crate nalgebra as na;
use crate::groups::{So3Matrix, ToSo3, ToTwist, ToVec, Twist};
use crate::interfaces::Numeric;
use na::{Matrix4, Vector3, Vector6};
use num::{One, Zero};

pub struct Se3Matrix<T: Numeric<T>>(pub Matrix4<T>);

pub trait ToSe3<T: Numeric<T>> {
    fn to_se3(&self) -> Se3Matrix<T>;
}

pub trait ToRP<T: Numeric<T>> {
    fn to_rp(&self) -> (So3Matrix<T>, Vector3<T>);
}

impl<T: Numeric<T>> ToSe3<T> for Matrix4<T> {
    fn to_se3(&self) -> Se3Matrix<T> {
        Se3Matrix(*self)
    }
}

impl<T: Numeric<T>> ToSe3<T> for Vector6<T> {
    fn to_se3(&self) -> Se3Matrix<T> {
        Se3Matrix(Matrix4::new(
            Zero::zero(),
            -self[2],
            self[1],
            self[3],
            self[2],
            Zero::zero(),
            -self[0],
            self[4],
            -self[1],
            self[0],
            Zero::zero(),
            self[5],
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        ))
    }
}

impl<T: Numeric<T>> ToSe3<T> for [T; 6] {
    fn to_se3(&self) -> Se3Matrix<T> {
        Se3Matrix(Matrix4::new(
            Zero::zero(),
            -self[2],
            self[1],
            self[3],
            self[2],
            Zero::zero(),
            -self[0],
            self[4],
            -self[1],
            self[0],
            Zero::zero(),
            self[5],
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        ))
    }
}

impl<T: Numeric<T>> ToRP<T> for Se3Matrix<T> {
    fn to_rp(&self) -> (So3Matrix<T>, Vector3<T>) {
        (
            So3Matrix(self.0.fixed_view::<3, 3>(0, 0).clone_owned()),
            self.0.fixed_view::<3, 1>(0, 3).clone_owned(),
        )
    }
}

impl<T: Numeric<T>> ToSo3<T> for Se3Matrix<T> {
    fn to_so3(&self) -> So3Matrix<T> {
        self.to_rp().0
    }
}

impl<T: Numeric<T>> ToVec<T> for Se3Matrix<T> {
    type VecSize = Vector6<T>;

    fn to_vec(&self) -> Vector6<T> {
        Vector6::new(
            self.0.m32, self.0.m13, self.0.m21, self.0.m14, self.0.m24, self.0.m34,
        )
    }
}

impl<T: Numeric<T>> ToTwist<T> for Se3Matrix<T> {
    fn to_twist(&self) -> Twist<T> {
        self.to_vec().to_twist()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::ToSo3;
    use na::Matrix3;

    #[test]
    fn array_to_se3() {
        let test_mat = Matrix4::new(
            0.0, -3.3, 2.2, 4.4, 3.3, 0.0, -1.1, 5.5, -2.2, 1.1, 0.0, 6.6, 0.0, 0.0, 0.0, 1.0,
        );
        let vec: [f64; 6] = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let res = vec.to_se3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn vector6_to_se3() {
        let test_mat = Matrix4::new(0, -3, 2, 4, 3, 0, -1, 5, -2, 1, 0, 6, 0, 0, 0, 1);
        let vec = Vector6::new(1, 2, 3, 4, 5, 6);
        let res = vec.to_se3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn matrix_to_se3() {
        let test_mat = Matrix4::new(
            0.0, -3.0, 2.0, 4.0, 3.0, 0.0, -1.0, 5.0, -2.0, 1.0, 0.0, 6.0, 0.0, 0.0, 0.0, 1.0,
        );
        let res = test_mat.to_se3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn se3_to_rp() {
        let test_t = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 3.0, 0.0, 0.0, 0.0, 1.0,
        )
        .to_se3();
        let test_r = Matrix3::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0).to_so3();
        let test_p = Vector3::new(0.0, 0.0, 3.0);
        let (r, p) = test_t.to_rp();
        assert_eq!(r.0, test_r.0);
        assert_eq!(p, test_p);
    }

    #[test]
    fn se3_to_so3() {
        let test_t = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 3.0, 0.0, 0.0, 0.0, 1.0,
        )
        .to_se3();
        let test_r = Matrix3::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0).to_so3();
        let r = test_t.to_so3();
        assert_eq!(r.0, test_r.0);
    }

    #[test]
    fn se3_to_vec() {
        let se3_mat = Matrix4::new(0, -3, 2, 4, 3, 0, -1, 5, -2, 1, 0, 6, 0, 0, 0, 1).to_se3();
        let target_vec = Vector6::new(1, 2, 3, 4, 5, 6);
        assert_eq!(se3_mat.to_vec(), target_vec);
    }

    #[test]
    fn se3_to_twist() {
        let se3_mat = Matrix4::new(0, -3, 2, 4, 3, 0, -1, 5, -2, 1, 0, 6, 0, 0, 0, 1).to_se3();
        let target_twist = Vector6::new(1, 2, 3, 4, 5, 6).to_twist();
        assert_eq!(se3_mat.to_twist().0, target_twist.0);
    }
}
