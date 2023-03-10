extern crate nalgebra as na;
use na::{Matrix3, Matrix4};

pub fn vec_to_so3<T: Copy>(omega: &[T; 3]) -> Matrix3<f64> where f64: std::convert::From<T> {
    return Matrix3::new(0.0, -f64::from(omega[2]), f64::from(omega[1]),
                           f64::from(omega[2]), 0.0, -f64::from(omega[0]),
                           -f64::from(omega[1]), f64::from(omega[0]), 0.0);
}

pub fn vec_to_se3<T: Copy>(omega: &[T; 6]) -> Matrix4<f64> where f64: std::convert::From<T> {
    return Matrix4::new(0.0, -f64::from(omega[2]), f64::from(omega[1]), f64::from(omega[3]),
                           f64::from(omega[2]), 0.0, -f64::from(omega[0]), f64::from(omega[4]),
                           -f64::from(omega[1]), f64::from(omega[0]), 0.0, f64::from(omega[5]),
                           0.0, 0.0, 0.0, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_so3() {
        let test_mat = Matrix3::new(0.0, -3.3, 2.2,
                                    3.3, 0.0, -1.1,
                                    -2.2, 1.1, 0.0);
        let vec: [f64; 3] = [1.1, 2.2, 3.3];
        let res = vec_to_so3(&vec);
        assert_eq!(res, test_mat);
    }

    #[test]
    fn test_so3_conversion() {
        let test_mat = Matrix3::new(0.0, -3.0, 2.0,
                                    3.0, 0.0, -1.0,
                                    -2.0, 1.0, 0.0);
        let vec: [i32; 3] = [1, 2, 3];
        let res = vec_to_so3(&vec);
        assert_eq!(res, test_mat);
    }

    #[test]
    fn test_so3_parameter() {
        let test_mat = Matrix3::new(0.0, -3.0, 2.0,
                                    3.0, 0.0, -1.0,
                                    -2.0, 1.0, 0.0);
        let res = vec_to_so3(&[1, 2, 3]);
        assert_eq!(res, test_mat);
    }
    
    #[test]
    fn test_se3() {
        let test_mat = Matrix4::new(0.0, -3.3, 2.2, 4.4,
                                    3.3, 0.0, -1.1, 5.5,
                                    -2.2, 1.1, 0.0, 6.6,
                                    0.0, 0.0, 0.0, 0.0);
        let vec: [f64; 6] = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let res = vec_to_se3(&vec);
        assert_eq!(res, test_mat);
    }

    #[test]
    fn test_se3_conversion() {
        let test_mat = Matrix4::new(0.0, -3.0, 2.0, 4.0,
                                    3.0, 0.0, -1.0, 5.0,
                                    -2.0, 1.0, 0.0, 6.0,
                                    0.0, 0.0, 0.0, 0.0);
        let vec: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let res = vec_to_se3(&vec);
        assert_eq!(res, test_mat);
    }

    #[test]
    fn test_se3_parameter() {
        let test_mat = Matrix4::new(0.0, -3.0, 2.0, 4.0,
                                    3.0, 0.0, -1.0, 5.0,
                                    -2.0, 1.0, 0.0, 6.0,
                                    0.0, 0.0, 0.0, 0.0);
        let res = vec_to_se3(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(res, test_mat);
    }
}
