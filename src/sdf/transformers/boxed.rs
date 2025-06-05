use std::marker::PhantomData;

use crate::{Sdf, SdfState};
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
