use crate::Sdf;
use num::Float;
use std::array;

pub struct Translated<Scalar: Float, T, const DIM: usize>
where
    T: Sdf<Scalar, DIM>,
{
    inner: T,
    inverse_translation: [Scalar; DIM],
}

impl<Scalar: Float, T, const DIM: usize> Sdf<Scalar, DIM> for Translated<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance(&self, point: &[Scalar; DIM]) -> Scalar {
        self.inner
            .distance(&array::from_fn(|i| point[i] + self.inverse_translation[i]))
    }
}

impl<Scalar: Float, T, const DIM: usize> Translated<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    #[inline]
    pub fn new(inner: T, tranlation: &[Scalar; DIM]) -> Self {
        Self {
            inner,
            inverse_translation: array::from_fn(|i| tranlation[i].neg()),
        }
    }
}
