use std::marker::PhantomData;

use crate::Sdf;
use num::Float;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boxed<Scalar: Float, T, const DIM: usize>(Box<T>, PhantomData<Scalar>)
where
    T: Sdf<Scalar, DIM>;

impl<Scalar: Float, T, const DIM: usize> Sdf<Scalar, DIM> for Boxed<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.0.distance_from_slice(point)
    }
}

impl<Scalar: Float, T, const DIM: usize> Boxed<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(Box::new(inner), PhantomData)
    }
}
