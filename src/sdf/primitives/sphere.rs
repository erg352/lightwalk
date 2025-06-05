use crate::{
    Sdf, SdfState,
    sdf::state::{SdfBindStateOperation, SdfMapStateOperation},
};
use num::Float;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sphere<State: SdfState = ()>(pub(crate) State);

impl<Scalar: Float, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State> for Sphere<State> {
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let value: Scalar = if DIM == 1 {
            // The squaring of the scalar followed by the square root may not get optimized out
            // when DIM == 1. The if statement is optimized out as DIM is constant with regards to
            // the function.
            point[0].abs()
        } else {
            point
                .iter()
                .map(|scalar| *scalar * *scalar)
                .fold(Scalar::zero(), |acc, e| acc + e)
                .sqrt()
        };

        value - Scalar::one()
    }

    #[inline]
    fn state(&self, _point: &[Scalar; DIM]) -> State {
        self.0.clone()
    }
}

impl<Scalar: Float, const DIM: usize, State: SdfState> SdfBindStateOperation<Scalar, DIM, State>
    for Sphere<()>
{
    type Output = Sphere<State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Sphere(s)
    }
}

impl<Scalar: Float, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Sphere<InState>
{
    type Output = Sphere<OutState>;

    #[inline]
    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Sphere((f)(self.0))
    }
}
