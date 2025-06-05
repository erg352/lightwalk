use std::marker::PhantomData;

use crate::{Sdf, SdfState, prelude::SdfMapStateOperation, sdf::state::SdfBindStateOperation};
use num::Float;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rounded<Scalar: Float, T, const DIM: usize, State: SdfState>
where
    T: Sdf<Scalar, DIM, State>,
{
    inner: T,
    factor: Scalar,
    _marker: PhantomData<State>,
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Rounded<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.inner.distance_from_slice(point) - self.factor
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        self.inner.state(point)
    }
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Rounded<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(inner: T, factor: Scalar) -> Self {
        Self {
            inner,
            factor,
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Rounded<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Rounded<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Rounded::new(self.inner.bind(s), self.factor)
    }
}

impl<Scalar: Float, T, U, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Rounded<Scalar, T, DIM, InState>
where
    T: SdfMapStateOperation<Scalar, DIM, InState, OutState, Output = U> + 'static,
    U: Sdf<Scalar, DIM, OutState> + 'static,
{
    type Output = Rounded<Scalar, U, DIM, OutState>;

    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Rounded::new(self.inner.map_state(f), self.factor)
    }
}
