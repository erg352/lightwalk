use std::marker::PhantomData;

use crate::{Sdf, SdfState, sdf::state::SdfBindStateOperation};
use num::Float;

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inverted<Scalar: Float, T, const DIM: usize, State: SdfState>(
    T,
    PhantomData<(Scalar, State)>,
)
where
    T: Sdf<Scalar, DIM, State>;

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Inverted<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        -self.0.distance_from_slice(point)
    }
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Inverted<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(inner, PhantomData)
    }
}

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Inverted<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Inverted<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Inverted::new(self.0.bind(s))
    }
}
