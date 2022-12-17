extern crate nalgebra as na;
use na::{base::allocator::Allocator, DefaultAllocator, OMatrix, Dim, Matrix3};

pub fn concat_horiz<T, R, C>(a: OMatrix<T, R, C>, b: OMatrix<T, R, C>) -> OMatrix<T, R, C>
    where
        R: Dim,
        C: Dim,
        DefaultAllocator: Allocator<T, R, C> {
    let (arows, acols) = a.shape_generic();
    return Matrix3::from_diagonal(1, 2, 3);
}
