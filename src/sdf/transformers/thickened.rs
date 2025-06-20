use std::marker::PhantomData;

use crate::{Sdf, SdfState};
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
