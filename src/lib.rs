#![doc = include_str!("../docs/lib.rs.md")]

pub mod marcher;
pub mod prelude;
pub mod sdf;

use std::ops::Deref;

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

impl<T, U, Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for T
where
    T: Deref<Target = U>,
    U: Sdf<Scalar, DIM>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.deref().distance_from_slice(point)
    }
}
