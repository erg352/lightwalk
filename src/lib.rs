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

#![doc = include_str!("../docs/lib.rs.md")]

pub mod marcher;
pub mod prelude;
pub mod sdf;

use num::Float;

/// Base trait used to define SDFs. See traits like [`sdf::combinators::SdfCombinationOperations`] and
/// [`sdf::transformers::SdfTransformOperations`] for additional features.
pub trait Sdf<Scalar: Float, const DIM: usize> {
    /// Evaluates the distance from the point passed in as a parameter to the object described by
    /// the SDF. The distance is positive if outside of the object, negative if inside, and 0 on the
    /// surface.
    /// # Example
    /// ```rust
    /// use lightwalk::prelude::*;
    ///
    /// let sphere = Sphere.translate(&[1.0, 2.0, 3.0]);
    ///
    /// let distance = sphere.distance(&[3.0, 2.0, 1.0]);
    /// ```
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar;

    fn distance_ref<'a>(&self, point: impl Into<&'a [Scalar; DIM]>) -> Scalar
    where
        Scalar: 'a,
    {
        self.distance_from_slice(point.into())
    }

    fn distance(&self, point: impl Into<[Scalar; DIM]>) -> Scalar {
        self.distance_from_slice(&point.into())
    }
}

impl<'a, T, Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for &T
where
    T: Sdf<Scalar, DIM>,
{
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        (*self).distance_from_slice(point)
    }
}
