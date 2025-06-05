use num::Float;

use crate::{
    Sdf, SdfState,
    sdf::state::{SdfBindStateOperation, SdfMapStateOperation},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line<Scalar: Float, const DIM: usize, State: SdfState = ()> {
    direction: [Scalar; DIM],
    state: State,
}

impl<Scalar: Float, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Line<Scalar, DIM, State>
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let dot = point
            .iter()
            .zip(self.direction.iter())
            .fold(Scalar::zero(), |acc, (&lhs, &rhs)| acc + lhs * rhs);

        (1..DIM)
            .map(|i| point[i] - self.direction[i] * dot)
            .fold(Scalar::zero(), |acc, e| acc + e * e)
            .sqrt()
    }

    #[inline]
    fn state(&self, _point: &[Scalar; DIM]) -> State {
        self.state.clone()
    }
}

impl<Scalar: Float, const DIM: usize, State: SdfState> Line<Scalar, DIM, State> {
    /// # Safety
    /// This function does not verify that the direction is normalized, which should always be
    /// true.
    #[inline]
    pub unsafe fn new_unchecked(direction: [Scalar; DIM], state: State) -> Self {
        Self { direction, state }
    }

    pub fn new(mut direction: [Scalar; DIM], state: State) -> Self {
        let norm = direction
            .iter()
            .map(|e| *e * *e)
            .fold(Scalar::zero(), |acc, e| acc + e)
            .sqrt();

        if norm == Scalar::zero() {
            panic!("Cannot define a plane with a null normal");
        }

        let inverse_norm = Scalar::one() / norm;

        for scalar in &mut direction {
            *scalar = *scalar * inverse_norm;
        }

        // Safety: We just normalized the input, so we are good to go!
        unsafe { Self::new_unchecked(direction, state) }
    }
}

impl<Scalar: Float, const DIM: usize, State: SdfState> SdfBindStateOperation<Scalar, DIM, State>
    for Line<Scalar, DIM>
{
    type Output = Line<Scalar, DIM, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        // Safety: direction should already be normalized as it comes from Self which should have
        // normalized it during initializion.
        unsafe { Line::new_unchecked(self.direction, s) }
    }
}

impl<Scalar: Float, const DIM: usize, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, DIM, InState, OutState> for Line<Scalar, DIM, InState>
{
    type Output = Line<Scalar, DIM, OutState>;

    #[inline]
    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        // Safety: direction should already be normalized as it comes from Self which should have
        // normalized it during initializion.
        unsafe { Line::new_unchecked(self.direction, (f)(self.state)) }
    }
}
