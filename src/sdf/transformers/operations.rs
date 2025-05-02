// Copyright © 2025 Ambre Guyot
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
