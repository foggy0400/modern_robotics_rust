extern crate nalgebra as na;
use crate::bcat;
use crate::concat::{allocate_block_output, Block, HCat, VCat};
use crate::groups::{Se3Matrix, ToVec};
use crate::interfaces::Numeric;
use na::{Const, Matrix, Matrix3, RowVector4, Vector3, ViewStorage};
use num::{One, Zero};

pub struct So3Matrix<T: Numeric<T>>(pub Matrix3<T>);

pub trait ToSo3<T: Numeric<T>> {
    fn to_so3(&self) -> So3Matrix<T>;
}

pub trait So3ToSe3<T: Numeric<T>> {
    fn to_se3(&self, p_vec: Vector3<T>) -> Se3Matrix<T>;
}

impl<T: Numeric<T>> ToSo3<T> for Matrix3<T> {
    fn to_so3(&self) -> So3Matrix<T> {
        So3Matrix(*self)
    }
}

impl<T: Numeric<T>> ToSo3<T> for Vector3<T> {
    fn to_so3(&self) -> So3Matrix<T> {
        So3Matrix(Matrix3::new(
            Zero::zero(),
            -self[2],
            self[1],
            self[2],
            Zero::zero(),
            -self[0],
            -self[1],
            self[0],
            Zero::zero(),
        ))
    }
}

impl<T: Numeric<T>> ToSo3<T>
    for Matrix<T, Const<3>, Const<1>, ViewStorage<'_, T, Const<3>, Const<1>, Const<1>, Const<6>>>
{
    fn to_so3(&self) -> So3Matrix<T> {
        So3Matrix(Matrix3::new(
            Zero::zero(),
            -self[(2, 0)],
            self[(1, 0)],
            self[(2, 0)],
            Zero::zero(),
            -self[(0, 0)],
            -self[(1, 0)],
            self[(0, 0)],
            Zero::zero(),
        ))
    }
}

impl<T: Numeric<T>> ToSo3<T> for [T; 3] {
    fn to_so3(&self) -> So3Matrix<T> {
        So3Matrix(Matrix3::new(
            Zero::zero(),
            -self[2],
            self[1],
            self[2],
            Zero::zero(),
            -self[0],
            -self[1],
            self[0],
            Zero::zero(),
        ))
    }
}

impl<T: Numeric<T>> ToVec<T> for So3Matrix<T> {
    type VecSize = Vector3<T>;

    fn to_vec(&self) -> Vector3<T> {
        Vector3::new(self.0.m32, self.0.m13, self.0.m21)
    }
}

impl<T: Numeric<T>> So3ToSe3<T> for So3Matrix<T> {
    fn to_se3(&self, p_vec: Vector3<T>) -> Se3Matrix<T> {
        Se3Matrix(bcat![self.0, p_vec;
                        RowVector4::new(Zero::zero(), Zero::zero(), Zero::zero(), One::one())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::ToSe3;
    use na::Vector6;

    #[test]
    fn array_to_so3() {
        let test_mat = Matrix3::new(0.0, -3.3, 2.2, 3.3, 0.0, -1.1, -2.2, 1.1, 0.0);
        let vec: [f64; 3] = [1.1, 2.2, 3.3];
        let res = vec.to_so3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn vector3_to_so3() {
        let t_vec = Vector3::new(1, 2, 3);
        let t_mat = t_vec.to_so3();
        let target = Matrix3::new(0, -3, 2, 3, 0, -1, -2, 1, 0);
        assert_eq!(t_mat.0, target);
    }

    #[test]
    fn matrix_to_so3() {
        let target = Matrix3::new(0, -3, 2, 3, 0, -1, -2, 1, 0);
        let res = target.to_so3();
        assert_eq!(res.0, target);
    }

    #[test]
    fn so3_to_vec() {
        let so3_mat = Matrix3::new(0, -3, 2, 3, 0, -1, -2, 1, 0).to_so3();
        let tar = Vector3::new(1, 2, 3);
        assert_eq!(so3_mat.to_vec(), tar);
    }

    #[test]
    fn so3_to_se3() {
        let so3_mat = [1, 2, 3].to_so3();
        let p_vec = Vector3::new(4, 5, 6);
        let tar = Vector6::new(1, 2, 3, 4, 5, 6).to_se3();
        assert_eq!(tar.0, so3_mat.to_se3(p_vec).0);
    }
}
