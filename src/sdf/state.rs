use num::Float;

use crate::{Sdf, SdfState};

pub trait SdfBindStateOperation<Scalar: Float, const DIM: usize, State: SdfState>:
    Sdf<Scalar, DIM>
{
    type Output: Sdf<Scalar, DIM, State>;

    fn bind(self, s: State) -> Self::Output;
}

pub trait SdfMapStateOperation<
    Scalar: Float,
    const DIM: usize,
    InState: SdfState,
    OutState: SdfState,
>: Sdf<Scalar, DIM, InState>
{
    type Output: Sdf<Scalar, DIM, OutState>;

    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output;
}
