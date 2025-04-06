use crate::Sdf;
use num::Float;

use super::{Intersection, Union};

pub trait SdfCombinationOperations<Scalar: Float, Rhs, const DIM: usize>:
    Sdf<Scalar, DIM> + Sized
where
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn add(self, rhs: Rhs) -> Union<Scalar, Self, Rhs, DIM> {
        Union::new(self, rhs)
    }

    #[inline]
    fn mul(self, rhs: Rhs) -> Intersection<Scalar, Self, Rhs, DIM> {
        Intersection::new(self, rhs)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> SdfCombinationOperations<Scalar, Rhs, DIM> for Lhs
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
}
