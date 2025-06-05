use crate::{Sdf, SdfState};
use num::Float;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
    lhs: Lhs,
    rhs: Rhs,
    phantom: PhantomData<(Scalar, State)>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for Intersection<Scalar, Lhs, Rhs, DIM, State>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.lhs
            .distance_from_slice(point)
            .max(self.rhs.distance_from_slice(point))
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState>
    Intersection<Scalar, Lhs, Rhs, DIM, State>
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
    #[inline]
    pub fn new(lhs: Lhs, rhs: Rhs) -> Self {
        Self {
            lhs,
            rhs,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IterIntersection<Scalar: Float, I, T, const DIM: usize, State: SdfState>
where
    T: Sdf<Scalar, DIM, State>,
    I: Iterator<Item = T> + Clone,
{
    iter: I,
    phantom: PhantomData<(Scalar, State)>,
}

impl<Scalar: Float, I, T, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State>
    for IterIntersection<Scalar, I, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
    I: Iterator<Item = T> + Clone,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.iter
            .clone()
            .map(|sdf| sdf.distance_from_slice(point))
            .reduce(|acc, e| acc.max(e))
            .unwrap_or(Scalar::infinity())
    }
}

impl<Scalar: Float, I, T, const DIM: usize, State: SdfState>
    IterIntersection<Scalar, I, T, DIM, State>
where
    T: Sdf<Scalar, DIM, State>,
    I: Iterator<Item = T> + Clone,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            phantom: PhantomData,
        }
    }
}
