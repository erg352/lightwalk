use crate::Sdf;
use num::Float;
use std::marker::PhantomData;

pub struct Union<Scalar: Float, Lhs, Rhs, const DIM: usize>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    lhs: Lhs,
    rhs: Rhs,
    phantom: PhantomData<Scalar>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Sdf<Scalar, DIM> for Union<Scalar, Lhs, Rhs, DIM>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance(&self, point: &[Scalar; DIM]) -> Scalar {
        self.lhs.distance(point).min(self.rhs.distance(point))
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Union<Scalar, Lhs, Rhs, DIM>
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
