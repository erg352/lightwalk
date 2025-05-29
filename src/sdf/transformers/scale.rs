use crate::Sdf;
use num::Float;
use std::array;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Scaled<Scalar: Float, T, const DIM: usize>
where
    T: Sdf<Scalar, DIM>,
{
    inner: T,
    scale: Scalar,
}

impl<Scalar: Float, T, const DIM: usize> Sdf<Scalar, DIM> for Scaled<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let inv_scale = Scalar::one() / self.scale;
        self.inner
            .distance_from_slice(&array::from_fn(|i| point[i] * inv_scale))
            * self.scale
    }
}

impl<Scalar: Float, T, const DIM: usize> Scaled<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    pub fn new(inner: T, scale: Scalar) -> Self {
        Self { inner, scale }
    }
}
