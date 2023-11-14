extern crate nalgebra as na;
use na::{Matrix3, Matrix4, Matrix6, Scalar, Vector3};
use num::traits::Num;

// File to contain definitions of different matrix types based on group theory
// Intention is to simplify function signatures of other functions that use these matrix groups
// Idea is to use traits to make constructing objects type-generic
//
// Comments are to lay out which ones to do
//
// SO3 matrix
// SE3 matrix
// Generic 3-vector
// v (angular velocity vector) - if needing special traits
// V (6-vector spatial velocity vec)
// ad matrix
//
pub struct So3Matrix<T: Num + Scalar>(pub Matrix3<T>);
