use crate::Sdf;
use num::Float;

use super::Union;

pub trait SdfCombinationOperations<Scalar: Float, Rhs, const DIM: usize>:
    Sdf<Scalar, DIM> + Sized
where
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn add(self, rhs: Rhs) -> Union<Scalar, Self, Rhs, DIM> {
        Union::new(self, rhs)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> SdfCombinationOperations<Scalar, Rhs, DIM> for Lhs
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
}
