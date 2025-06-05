use crate::{Sdf, SdfState};
use num::Float;
use std::marker::PhantomData;

use super::SdfCombinationOperations;
use crate::sdf::transformers::SdfTransformOperations;

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Difference<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
    lhs: Lhs,
    rhs: Rhs,
    phantom: PhantomData<(Scalar, State)>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Difference<Scalar, Lhs, Rhs, DIM, State>
where
    Lhs: Sdf<Scalar, DIM, State> + 'static,
    Rhs: Sdf<Scalar, DIM, State> + 'static,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let sdf = (&self.lhs).mul((&self.rhs).invert());
        sdf.distance_from_slice(point)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState>
    Difference<Scalar, Lhs, Rhs, DIM, State>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
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
