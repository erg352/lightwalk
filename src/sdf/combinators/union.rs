use crate::{Sdf, SdfState};
use num::Float;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Union<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState, B>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
    B: Fn((Scalar, State), (Scalar, State)) -> State,
{
    lhs: Lhs,
    rhs: Rhs,
    state_blender: B,
    _marker: PhantomData<(Scalar, State)>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState, B> Sdf<Scalar, DIM, State>
    for Union<Scalar, Lhs, Rhs, DIM, State, B>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
    B: Fn((Scalar, State), (Scalar, State)) -> State,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.lhs
            .distance_from_slice(point)
            .min(self.rhs.distance_from_slice(point))
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        let lhs_distance = self.lhs.distance_from_slice(point);
        let rhs_distance = self.rhs.distance_from_slice(point);

        let lhs_state = self.lhs.state(point);
        let rhs_state = self.rhs.state(point);

        (self.state_blender)((lhs_distance, lhs_state), (rhs_distance, rhs_state))
    }

    #[inline]
    fn distance_and_state(&self, point: impl Into<[Scalar; DIM]>) -> (Scalar, State) {
        let point = point.into();

        let lhs_distance = self.lhs.distance_from_slice(&point);
        let rhs_distance = self.rhs.distance_from_slice(&point);

        let lhs_state = self.lhs.state(&point);
        let rhs_state = self.rhs.state(&point);

        let distance = lhs_distance.min(rhs_distance);
        let state = (self.state_blender)((lhs_distance, lhs_state), (rhs_distance, rhs_state));

        (distance, state)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState, B>
    Union<Scalar, Lhs, Rhs, DIM, State, B>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
    B: Fn((Scalar, State), (Scalar, State)) -> State,
{
    #[inline]
    pub fn new(lhs: Lhs, rhs: Rhs, state_blender: B) -> Self {
        Self {
            lhs,
            rhs,
            state_blender,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IterUnion<Scalar: Float, I, T, const DIM: usize, State: SdfState>
where
    T: Sdf<Scalar, DIM, State>,
    I: Iterator<Item = T> + Clone,
{
    iter: I,
    _marker: PhantomData<(Scalar, State)>,
}

impl<Scalar: Float, I, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for IterUnion<Scalar, I, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
    I: Iterator<Item = T> + Clone,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.iter
            .clone()
            .map(|sdf| sdf.distance_from_slice(point))
            .reduce(|acc, e| acc.min(e))
            .unwrap_or(Scalar::infinity())
    }

    #[inline]
    fn state(&self, point: &[Scalar; DIM]) -> State {
        let (sdf, _) = self
            .iter
            .clone()
            .map(|sdf| {
                let point = sdf.distance_from_slice(point);
                (sdf, point)
            })
            .reduce(|acc, e| (acc.0, acc.1.min(e.1)))
            .unwrap();

        sdf.state(point)
    }
}

impl<Scalar: Float, I, T, const DIM: usize, State: SdfState> IterUnion<Scalar, I, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
    I: Iterator<Item = T> + Clone,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            _marker: PhantomData,
        }
    }
}
