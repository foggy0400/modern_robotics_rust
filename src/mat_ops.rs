extern crate nalgebra as na;
use na::SMatrix;

pub fn concat_horiz<T, const R: usize, const C1: usize, const C2: usize, const C3: usize>(a: SMatrix<T, R, C1>, b: SMatrix<T, R, C2>) -> SMatrix<T, R, C3> {
    return SMatrix::from_columns();
}
