use crate::{Sdf, SdfState, prelude::SdfMapStateOperation, sdf::state::SdfBindStateOperation};
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

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Repeated<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Repeated<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Repeated::new(self.inner.bind(s), self.repeat_spacing)
    }
}

impl<Scalar: Float, T, U, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Repeated<Scalar, T, DIM, InState>
where
    T: SdfMapStateOperation<Scalar, DIM, InState, OutState, Output = U> + 'static,
    U: Sdf<Scalar, DIM, OutState> + 'static,
{
    type Output = Repeated<Scalar, U, DIM, OutState>;

    #[inline]
    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Repeated::new(self.inner.map_state(f), self.repeat_spacing)
    }
}
