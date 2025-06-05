use crate::{Sdf, SdfState, sdf::state::SdfBindStateOperation};
use num::Float;
use std::{array, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Translated<Scalar: Float, T, const DIM: usize, State: SdfState>
where
    T: Sdf<Scalar, DIM, State>,
{
    inner: T,
    inverse_translation: [Scalar; DIM],
    _marker: PhantomData<State>,
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Translated<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.inner
            .distance_from_slice(&array::from_fn(|i| point[i] + self.inverse_translation[i]))
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        self.inner.state(point)
    }
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Translated<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(inner: T, tranlation: &[Scalar; DIM]) -> Self {
        Self {
            inner,
            inverse_translation: array::from_fn(|i| tranlation[i].neg()),
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Translated<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Translated<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Translated {
            inner: self.inner.bind(s),
            inverse_translation: self.inverse_translation,
            _marker: PhantomData,
        }
    }
}
