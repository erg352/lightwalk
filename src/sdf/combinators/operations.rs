use crate::{Sdf, SdfState};
use num::Float;

use super::{Difference, Intersection, Union, intersection::IterIntersection, union::IterUnion};

pub trait SdfCombinationOperations<Scalar: Float, Rhs, const DIM: usize, State: SdfState>:
    Sdf<Scalar, DIM, State> + Sized
where
    Rhs: Sdf<Scalar, DIM, State>,
{
    #[inline]
    fn add(self, rhs: Rhs) -> Union<Scalar, Self, Rhs, DIM, State> {
        Union::new(self, rhs)
    }

    #[inline]
    fn mul(self, rhs: Rhs) -> Intersection<Scalar, Self, Rhs, DIM, State> {
        Intersection::new(self, rhs)
    }

    #[inline]
    fn sub(self, rhs: Rhs) -> Difference<Scalar, Self, Rhs, DIM, State> {
        Difference::new(self, rhs)
    }
}

impl<Scalar: Float, Lhs, Rhs, const DIM: usize, State: SdfState>
    SdfCombinationOperations<Scalar, Rhs, DIM, State> for Lhs
where
    Lhs: Sdf<Scalar, DIM, State>,
    Rhs: Sdf<Scalar, DIM, State>,
{
}

pub trait SdfIterCombinationOperations<Scalar: Float, const DIM: usize, State: SdfState>:
    Iterator<Item: Sdf<Scalar, DIM, State>> + Clone + Sized
{
    #[inline]
    fn union(self) -> IterUnion<Scalar, Self, <Self as Iterator>::Item, DIM, State> {
        IterUnion::new(self)
    }

    #[inline]
    fn intersection(self) -> IterIntersection<Scalar, Self, <Self as Iterator>::Item, DIM, State> {
        IterIntersection::new(self)
    }
}

impl<Scalar: Float, const DIM: usize, I, State: SdfState>
    SdfIterCombinationOperations<Scalar, DIM, State> for I
where
    I: Iterator<Item: Sdf<Scalar, DIM, State>> + Clone + Sized,
{
}
