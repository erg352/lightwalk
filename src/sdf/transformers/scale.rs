use crate::{Sdf, SdfState, sdf::state::SdfBindStateOperation};
use num::Float;
use std::{array, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scaled<Scalar: Float, T, const DIM: usize, State: SdfState>
where
    T: Sdf<Scalar, DIM, State>,
{
    inner: T,
    scale: Scalar,
    _marker: PhantomData<State>,
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Scaled<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let inv_scale = Scalar::one() / self.scale;
        self.inner
            .distance_from_slice(&array::from_fn(|i| point[i] * inv_scale))
            * self.scale
    }
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Scaled<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(inner: T, scale: Scalar) -> Self {
        Self {
            inner,
            scale,
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Scaled<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Scaled<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Scaled::new(self.inner.bind(s), self.scale)
    }
}
