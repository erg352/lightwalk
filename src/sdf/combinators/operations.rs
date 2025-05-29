use crate::Sdf;
use num::Float;

use super::{Difference, Intersection, Union, intersection::IterIntersection, union::IterUnion};

pub trait SdfCombinationOperations<Scalar: Float, Rhs, const DIM: usize>:
    Sdf<Scalar, DIM> + Sized
where
    Rhs: Sdf<Scalar, DIM>,
{
    #[inline]
    fn add(self, rhs: Rhs) -> Union<Scalar, Self, Rhs, DIM> {
        Union::new(self, rhs)
    }

    #[inline]
    fn mul(self, rhs: Rhs) -> Intersection<Scalar, Self, Rhs, DIM> {
        Intersection::new(self, rhs)
    }

    #[inline]
    fn sub(self, rhs: Rhs) -> Difference<Scalar, Self, Rhs, DIM> {
        Difference::new(self, rhs)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize> SdfCombinationOperations<Scalar, Rhs, DIM> for Lhs
where
    Lhs: Sdf<Scalar, DIM>,
    Rhs: Sdf<Scalar, DIM>,
{
}

pub trait SdfIterCombinationOperations<Scalar: Float, const DIM: usize>:
    Iterator<Item: Sdf<Scalar, DIM>> + Clone + Sized
{
    #[inline]
    fn union(self) -> IterUnion<Scalar, Self, <Self as Iterator>::Item, DIM> {
        IterUnion::new(self)
    }

    #[inline]
    fn intersection(self) -> IterIntersection<Scalar, Self, <Self as Iterator>::Item, DIM> {
        IterIntersection::new(self)
    }
}

impl<Scalar: Float, const DIM: usize, I> SdfIterCombinationOperations<Scalar, DIM> for I where
    I: Iterator<Item: Sdf<Scalar, DIM>> + Clone + Sized
{
}
