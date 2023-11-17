extern crate nalgebra as na;
use crate::interfaces::Numeric;
use na::{Const, Matrix, Matrix3, Vector3, ViewStorage};
use num::Zero;

pub struct So3Matrix<T: Numeric<T>>(pub Matrix3<T>);

pub trait ToSo3<T: Numeric<T>> {
    fn to_so3(&self) -> So3Matrix<T>;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_so3_array() {
        let test_mat = Matrix3::new(0.0, -3.3, 2.2, 3.3, 0.0, -1.1, -2.2, 1.1, 0.0);
        let vec: [f64; 3] = [1.1, 2.2, 3.3];
        let res = vec.to_so3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn to_so3_vector3() {
        let t_vec = Vector3::new(1, 2, 3);
        let t_mat = t_vec.to_so3();
        let target = Matrix3::new(0, -3, 2, 3, 0, -1, -2, 1, 0);
        assert_eq!(t_mat.0, target);
    }

    #[test]
    fn to_so3_matrix3() {
        let target = Matrix3::new(0, -3, 2, 3, 0, -1, -2, 1, 0);
        let res = target.to_so3();
        assert_eq!(res.0, target);
    }
}
