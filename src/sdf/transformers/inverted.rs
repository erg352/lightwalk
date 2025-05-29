use std::marker::PhantomData;

use crate::Sdf;
use num::Float;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Inverted<Scalar: Float, T, const DIM: usize>(T, PhantomData<Scalar>)
where
    T: Sdf<Scalar, DIM>;

impl<Scalar: Float, T, const DIM: usize> Sdf<Scalar, DIM> for Inverted<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        -self.0.distance_from_slice(point)
    }
}

impl<Scalar: Float, T, const DIM: usize> Inverted<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(inner, PhantomData)
    }
}
