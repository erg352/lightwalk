use crate::{Sdf, SdfState};
use num::Float;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Union<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
    lhs: Lhs,
    rhs: Rhs,
    _marker: PhantomData<(Scalar, State)>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Union<Scalar, Lhs, Rhs, DIM, State>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
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

        if lhs_distance < rhs_distance {
            self.lhs.state(point)
        } else {
            self.rhs.state(point)
        }
    }

    #[inline]
    fn distance_and_state(&self, point: impl Into<[Scalar; DIM]>) -> (Scalar, State) {
        let point = point.into();

        let lhs_distance = self.lhs.distance_from_slice(&point);
        let rhs_distance = self.rhs.distance_from_slice(&point);

        if lhs_distance < rhs_distance {
            (lhs_distance, self.lhs.state(&point))
        } else {
            (rhs_distance, self.rhs.state(&point))
        }
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState> Union<Scalar, Lhs, Rhs, DIM, State>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(lhs: Lhs, rhs: Rhs) -> Self {
        Self {
            lhs,
            rhs,
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
