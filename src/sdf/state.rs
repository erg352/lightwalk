use std::marker::PhantomData;

use num::Float;

use crate::{Sdf, SdfState};

pub trait SdfBindStateOperation<Scalar: Float, const DIM: usize, State: SdfState>:
    Sdf<Scalar, DIM, ()>
where
    Self: Sized,
{
    fn bind(self, state: State) -> StateBound<Scalar, DIM, Self, State> {
        StateBound {
            inner: self,
            state,
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, const DIM: usize, State: SdfState, T> SdfBindStateOperation<Scalar, DIM, State>
    for T
where
    T: Sdf<Scalar, DIM, ()> + Sized,
{
}

pub struct StateBound<Scalar: Float, const DIM: usize, T, State: SdfState>
where
    T: Sdf<Scalar, DIM, ()>,
{
    inner: T,
    state: State,
    _marker: PhantomData<Scalar>,
}

impl<Scalar: Float, const DIM: usize, State: SdfState, T> Sdf<Scalar, DIM, State>
    for StateBound<Scalar, DIM, T, State>
where
    T: Sdf<Scalar, DIM, ()>,
{
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.inner.distance_from_slice(point)
    }

    fn state(&self, _: &[Scalar; DIM]) -> State {
        self.state.clone()
    }
}
