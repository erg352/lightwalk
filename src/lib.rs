#![doc = include_str!("../docs/lib.rs.md")]

pub mod combinators;
pub mod prelude;
pub mod primitives;

use num::Float;

pub trait Sdf<Scalar: Float, const DIM: usize> {
    fn distance(&self, point: &[Scalar; DIM]) -> Scalar;
}
