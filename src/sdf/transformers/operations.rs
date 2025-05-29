use crate::Sdf;
use num::Float;

use super::{Boxed, Inverted, Repeated, Rounded, Scaled, Thickened, Translated};

pub trait SdfTransformOperations<Scalar: Float, const DIM: usize>:
    Sdf<Scalar, DIM> + Sized
{
    /// Translates the SDF by a given vector.
    #[inline]
    fn translate(self, translation: &[Scalar; DIM]) -> Translated<Scalar, Self, DIM> {
        Translated::new(self, translation)
    }

    /// Scales the SDF by a given factor. There is currently no support for non-homogenous scaling.
    #[inline]
    fn scale(self, scale: Scalar) -> Scaled<Scalar, Self, DIM> {
        Scaled::new(self, scale)
    }

    /// Rounds the corners of the SDF by a given factor.
    #[inline]
    fn round(self, factor: Scalar) -> Rounded<Scalar, Self, DIM> {
        Rounded::new(self, factor)
    }

    /// Repeats the SDF to infinity based off of a certain repeat size.
    #[inline]
    fn repeat(self, repeat_spacing: impl Into<[Scalar; DIM]>) -> Repeated<Scalar, Self, DIM> {
        Repeated::new(self, repeat_spacing.into())
    }

    /// Adds thickness to the SDF. Points who's distance to the surface is less than the thickness
    /// value will be inside, others outside.
    #[inline]
    fn thickness(self, thickness: Scalar) -> Thickened<Scalar, Self, DIM> {
        Thickened::new(self, thickness)
    }

    /// Inverts the SDF. What was previously outside is now inside, and vice versa.
    #[inline]
    fn invert(self) -> Inverted<Scalar, Self, DIM> {
        Inverted::new(self)
    }

    /// Places the data stored by the SDF in the heap.
    #[inline]
    fn in_box(self) -> Boxed<Scalar, Self, DIM> {
        Boxed::new(self)
    }
}

impl<T, Scalar: Float, const DIM: usize> SdfTransformOperations<Scalar, DIM> for T where
    T: Sdf<Scalar, DIM>
{
}
