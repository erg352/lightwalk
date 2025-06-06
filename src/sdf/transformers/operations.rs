use std::{rc::Rc, sync::Arc};

use crate::{Sdf, SdfState};
use num::Float;

use super::{Inverted, Repeated, Rounded, Scaled, Thickened, Translated};

pub trait SdfTransformOperations<Scalar: Float, const DIM: usize, State: SdfState>:
    Sdf<Scalar, DIM, State> + Sized
{
    /// Translates the SDF by a given vector.
    #[inline]
    fn translate(self, translation: &[Scalar; DIM]) -> Translated<Scalar, Self, DIM, State> {
        Translated::new(self, translation)
    }

    /// Scales the SDF by a given factor. There is currently no support for non-homogenous scaling.
    #[inline]
    fn scale(self, scale: Scalar) -> Scaled<Scalar, Self, DIM, State> {
        Scaled::new(self, scale)
    }

    /// Rounds the corners of the SDF by a given factor.
    #[inline]
    fn round(self, factor: Scalar) -> Rounded<Scalar, Self, DIM, State> {
        Rounded::new(self, factor)
    }

    /// Repeats the SDF to infinity based off of a certain repeat size.
    #[inline]
    fn repeat(
        self,
        repeat_spacing: impl Into<[Scalar; DIM]>,
    ) -> Repeated<Scalar, Self, DIM, State> {
        Repeated::new(self, repeat_spacing.into())
    }

    /// Adds thickness to the SDF. Points who's distance to the surface is less than the thickness
    /// value will be inside, others outside.
    #[inline]
    fn thickness(self, thickness: Scalar) -> Thickened<Scalar, Self, DIM, State> {
        Thickened::new(self, thickness)
    }

    /// Inverts the SDF. What was previously outside is now inside, and vice versa.
    #[inline]
    fn invert(self) -> Inverted<Scalar, Self, DIM, State> {
        Inverted::new(self)
    }

    /// Places the data stored by the SDF in the heap.
    #[inline]
    fn in_box(self) -> Box<Self> {
        Box::new(self)
    }

    /// Places the data stored by the SDF in the heap as a shared pointer.
    #[inline]
    fn in_rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    /// Places the data stored by the SDF in the heap as a thread-safe shared pointer.
    #[inline]
    fn in_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl<T, Scalar: Float, const DIM: usize, State: SdfState> SdfTransformOperations<Scalar, DIM, State>
    for T
where
    T: Sdf<Scalar, DIM, State>,
{
}
