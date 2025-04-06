#![doc = include_str!("../docs/lib.rs.md")]

pub mod combinators;
pub mod prelude;
pub mod primitives;
pub mod transformers;

use num::Float;

pub trait Sdf<Scalar: Float, const DIM: usize> {
    fn distance(&self, point: &[Scalar; DIM]) -> Scalar;
}
