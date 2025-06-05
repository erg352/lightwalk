use std::marker::PhantomData;

use crate::{Sdf, SdfState, prelude::SdfMapStateOperation, sdf::state::SdfBindStateOperation};
use num::Float;

#[derive(Debug, Default, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boxed<Scalar: Float, T, const DIM: usize, State: SdfState = ()>(
    Box<T>,
    PhantomData<(Scalar, State)>,
)
where
    T: Sdf<Scalar, DIM, State>;

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Boxed<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State> + 'static,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.0.distance_from_slice(point)
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        self.0.state(point)
    }
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Boxed<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(Box::new(inner), PhantomData)
    }
}

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Boxed<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Boxed<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Boxed::new(self.0.bind(s))
    }
}

impl<Scalar: Float, T, U, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Boxed<Scalar, T, DIM, InState>
where
    T: SdfMapStateOperation<Scalar, DIM, InState, OutState, Output = U> + 'static,
    U: Sdf<Scalar, DIM, OutState> + 'static,
{
    type Output = Boxed<Scalar, U, DIM, OutState>;

    #[inline]
    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Boxed::new(self.0.map_state(f))
    }
}
