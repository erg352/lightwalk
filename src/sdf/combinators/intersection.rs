use crate::Sdf;
use num::Float;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Intersection<Scalar: Float, Lhs, Rhs, const DIM: usize>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    lhs: Lhs,
    rhs: Rhs,
    phantom: PhantomData<Scalar>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Sdf<Scalar, DIM>
    for Intersection<Scalar, Lhs, Rhs, DIM>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.lhs
            .distance_from_slice(point)
            .max(self.rhs.distance_from_slice(point))
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Intersection<Scalar, Lhs, Rhs, DIM>
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
