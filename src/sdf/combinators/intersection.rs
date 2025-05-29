use crate::Sdf;
use num::Float;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialze))]
pub struct Intersection<Scalar: Float, Lhs, Rhs, const DIM: usize>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    lhs: Lhs,
    rhs: Rhs,
    phantom: PhantomData<Scalar>,
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Sdf<Scalar, DIM>
    for Intersection<Scalar, Lhs, Rhs, DIM>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.lhs
            .distance_from_slice(point)
            .max(self.rhs.distance_from_slice(point))
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> Intersection<Scalar, Lhs, Rhs, DIM>
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
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

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialze))]
pub struct IterIntersection<Scalar: Float, I, T, const DIM: usize>
where
    T: Sdf<Scalar, DIM>,
    I: Iterator<Item = T> + Clone,
{
    iter: I,
    phantom: PhantomData<Scalar>,
}

impl<Scalar: Float, I, T, const DIM: usize> Sdf<Scalar, DIM> for IterIntersection<Scalar, I, T, DIM>
where
    T: Sdf<Scalar, DIM>,
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

impl<Scalar: Float, I, T, const DIM: usize> IterIntersection<Scalar, I, T, DIM>
where
    T: Sdf<Scalar, DIM>,
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
