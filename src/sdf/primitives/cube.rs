use crate::{
    Sdf, SdfState,
    sdf::state::{SdfBindStateOperation, SdfMapStateOperation},
};
use num::Float;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cube<State: SdfState = ()>(pub(crate) State);

impl<Scalar: Float, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State> for Cube<State> {
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        point
            .iter()
            .map(|axis| axis.abs() - Scalar::from(0.5).unwrap())
            .reduce(|acc, e| acc.max(e))
            .unwrap_or(Scalar::zero())
    }

    #[inline]
    fn state(&self, _point: &[Scalar; DIM]) -> State {
        self.0.clone()
    }
}

impl<Scalar: Float, const DIM: usize, State: SdfState> SdfBindStateOperation<Scalar, DIM, State>
    for Cube
{
    type Output = Cube<State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Cube(s)
    }
}

impl<Scalar: Float, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Cube<InState>
{
    type Output = Cube<OutState>;

    #[inline]
    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Cube((f)(self.0))
    }
}
