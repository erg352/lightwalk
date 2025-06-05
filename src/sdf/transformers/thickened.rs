use std::marker::PhantomData;

use crate::{Sdf, SdfState, prelude::SdfMapStateOperation, sdf::state::SdfBindStateOperation};
use num::Float;

/// Adds thickness to the SDF. All points who's absolute distance to the surface of the SDF are
/// less than the specified thickness are "inside" the new SDF, and all other points are "outside".
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Thickened<Scalar: Float, T: Sdf<Scalar, DIM, State>, const DIM: usize, State: SdfState> {
    inner: T,
    thickness: Scalar,
    _marker: PhantomData<State>,
}

impl<Scalar: Float, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Thickened<Scalar, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
{
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let base_sdf = self.inner.distance_from_slice(point);

        base_sdf.abs() - self.thickness
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        self.inner.state(point)
    }
}

impl<Scalar: Float, T: Sdf<Scalar, DIM, State>, const DIM: usize, State: SdfState>
    Thickened<Scalar, T, DIM, State>
{
    pub fn new(inner: T, thickness: Scalar) -> Self {
        Self {
            inner,
            thickness,
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, T, U, const DIM: usize, State: SdfState>
    SdfBindStateOperation<Scalar, DIM, State> for Thickened<Scalar, T, DIM, ()>
where
    T: SdfBindStateOperation<Scalar, DIM, State, Output = U> + 'static,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    type Output = Thickened<Scalar, U, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Thickened::new(self.inner.bind(s), self.thickness)
    }
}

impl<Scalar: Float, T, U, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Thickened<Scalar, T, DIM, InState>
where
    T: SdfMapStateOperation<Scalar, DIM, InState, OutState, Output = U> + 'static,
    U: Sdf<Scalar, DIM, OutState> + 'static,
{
    type Output = Thickened<Scalar, U, DIM, OutState>;

    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Thickened::new(self.inner.map_state(f), self.thickness)
    }
}
