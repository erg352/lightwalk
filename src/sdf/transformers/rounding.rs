use crate::Sdf;
use num::Float;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rounded<Scalar: Float, T, const DIM: usize>
where
    T: Sdf<Scalar, DIM>,
{
    inner: T,
    factor: Scalar,
}

impl<Scalar: Float, T, const DIM: usize> Sdf<Scalar, DIM> for Rounded<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.inner.distance_from_slice(point) - self.factor
    }
}

impl<Scalar: Float, T, const DIM: usize> Rounded<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    pub fn new(inner: T, factor: Scalar) -> Self {
        Self { inner, factor }
    }
}
