use nalgebra::allocator::Allocator;
use nalgebra::constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use nalgebra::dimension::{DimAdd, DimSum};
use nalgebra::storage::{Storage, StorageMut};
use nalgebra::DefaultAllocator;
use nalgebra::{Dim, Matrix, OMatrix, Scalar};
use num_traits::Zero;

// This file is based on an excellent piece of code by @Andlon on GitHub
//
// Should check regularly whether the bcat! macro has been implemented into nalgebra itself,
// there's a PR up on GitHub at time of writing.

pub trait Block<T> {
    type Rows: Dim;
    type Cols: Dim;

    fn shape(&self) -> (Self::Rows, Self::Cols);

    fn populate<S>(&self, output: &mut Matrix<T, Self::Rows, Self::Cols, S>)
    where
        T: Scalar,
        S: StorageMut<T, Self::Rows, Self::Cols>;
}

impl<T, R, C, S> Block<T> for Matrix<T, R, C, S>
where
    T: Scalar,
    R: Dim,
    C: Dim,
    S: Storage<T, R, C>,
{
    type Rows = R;
    type Cols = C;

    fn shape(&self) -> (Self::Rows, Self::Cols) {
        self.data.shape()
    }

    fn populate<S2>(&self, output: &mut Matrix<T, Self::Rows, Self::Cols, S2>)
    where
        T: Scalar,
        S2: StorageMut<T, Self::Rows, Self::Cols>,
    {
        output.copy_from(self)
    }
}

pub struct HCat<X>(pub X);
pub struct VCat<X>(pub X);

impl<T, B> Block<T> for HCat<(B,)>
where
    B: Block<T>,
{
    type Rows = B::Rows;
    type Cols = B::Cols;

    fn shape(&self) -> (Self::Rows, Self::Cols) {
        self.0 .0.shape()
    }

    fn populate<S>(&self, output: &mut Matrix<T, Self::Rows, Self::Cols, S>)
    where
        T: Scalar,
        S: StorageMut<T, Self::Rows, Self::Cols>,
    {
        self.0 .0.populate(output);
    }
}

impl<T, B1, B2> Block<T> for HCat<(B1, B2)>
where
    B1: Block<T>,
    B2: Block<T>,
    B1::Cols: DimAdd<B2::Cols>,
    ShapeConstraint: SameNumberOfRows<B1::Rows, B2::Rows>,
{
    type Rows = <ShapeConstraint as SameNumberOfRows<B1::Rows, B2::Rows>>::Representative;
    type Cols = DimSum<B1::Cols, B2::Cols>;

    fn shape(&self) -> (Self::Rows, Self::Cols) {
        let (r1, c1) = self.0 .0.shape();
        let (_, c2) = self.0 .1.shape();
        let r = <Self::Rows as Dim>::from_usize(r1.value());
        let c = c1.add(c2);
        (r, c)
    }

    fn populate<S>(&self, output: &mut Matrix<T, Self::Rows, Self::Cols, S>)
    where
        T: Scalar,
        S: StorageMut<T, Self::Rows, Self::Cols>,
    {
        assert_eq!(self.0 .0.shape().0.value(), self.0 .1.shape().0.value());

        let mut output_0 = output.generic_slice_mut((0, 0), self.0 .0.shape());
        self.0 .0.populate(&mut output_0);

        let offset = self.0 .0.shape().1.value();
        let mut output_1 = output.generic_slice_mut((0, offset), self.0 .1.shape());
        self.0 .1.populate(&mut output_1);
    }
}

impl<T, B1, B2> Block<T> for VCat<(B1, B2)>
where
    B1: Block<T>,
    B2: Block<T>,
    B1::Rows: DimAdd<B2::Rows>,
    ShapeConstraint: SameNumberOfColumns<B1::Cols, B2::Cols>,
{
    type Rows = DimSum<B1::Rows, B2::Rows>;
    type Cols = <ShapeConstraint as SameNumberOfColumns<B1::Cols, B2::Cols>>::Representative;

    fn shape(&self) -> (Self::Rows, Self::Cols) {
        let (r1, c1) = self.0 .0.shape();
        let (r2, _) = self.0 .1.shape();
        let r = r1.add(r2);
        let c = <Self::Cols as Dim>::from_usize(c1.value());
        (r, c)
    }

    fn populate<S>(&self, output: &mut Matrix<T, Self::Rows, Self::Cols, S>)
    where
        T: Scalar,
        S: StorageMut<T, Self::Rows, Self::Cols>,
    {
        assert_eq!(self.0 .0.shape().1.value(), self.0 .1.shape().1.value());

        let mut output_0 = output.generic_slice_mut((0, 0), self.0 .0.shape());
        self.0 .0.populate(&mut output_0);

        let offset = self.0 .0.shape().0.value();
        let mut output_1 = output.generic_slice_mut((offset, 0), self.0 .1.shape());
        self.0 .1.populate(&mut output_1);
    }
}

pub fn allocate_block_output<T, B>(block: &B) -> OMatrix<T, B::Rows, B::Cols>
where
    T: Scalar + Zero,
    B: Block<T>,
    DefaultAllocator: Allocator<T, B::Rows, B::Cols>,
{
    let (rows, cols) = block.shape();
    OMatrix::zeros_generic(rows, cols)
}

#[macro_export]
macro_rules! bcat {
    ($( $( $x: expr ),*);*) => {
        {
            let block_expression = VCat(($(HCat(($($x),*,))),*));
            let mut output = allocate_block_output(&block_expression);
            block_expression.populate(&mut output);
            output
        }
    }
}
