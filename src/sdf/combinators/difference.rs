use crate::Sdf;
use num::Float;
use std::marker::PhantomData;

use super::SdfCombinationOperations;
use crate::sdf::transformers::SdfTransformOperations;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Difference<Scalar: Float, Lhs, Rhs, const DIM: usize>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    lhs: Lhs,
    rhs: Rhs,
    phantom: PhantomData<Scalar>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Sdf<Scalar, DIM>
    for Difference<Scalar, Lhs, Rhs, DIM>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let sdf = (&self.lhs).mul((&self.rhs).invert());
        sdf.distance_from_slice(point)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Difference<Scalar, Lhs, Rhs, DIM>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    pub fn new(lhs: Lhs, rhs: Rhs) -> Self {
        Self {
            lhs,
            rhs,
            phantom: PhantomData,
        }
    }
}
