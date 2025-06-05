use crate::{Sdf, SdfState};
use num::Float;
use std::{array, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Repeated<Scalar: Float, T, const DIM: usize, State: SdfState>
where
    T: Sdf<Scalar, DIM, State>,
{
    inner: T,
    repeat_spacing: [Scalar; DIM],
    _marker: PhantomData<State>,
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Repeated<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.inner.distance_from_slice(&array::from_fn(|i| {
            point[i] - self.repeat_spacing[i] * (point[i] / self.repeat_spacing[i]).round()
        }))
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        self.inner.state(point)
    }
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Repeated<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(inner: T, repeat_spacing: [Scalar; DIM]) -> Self {
        Self {
            inner,
            repeat_spacing,
            _marker: PhantomData,
        }
    }
}
