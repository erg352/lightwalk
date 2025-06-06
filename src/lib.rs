#![doc = include_str!("../docs/lib.rs.md")]

pub mod marcher;
pub mod prelude;
pub mod sdf;

use std::ops::Deref;

use num::Float;

/// Base trait used to define SDFs. See traits like [`sdf::combinators::SdfCombinationOperations`] and
/// [`sdf::transformers::SdfTransformOperations`] for additional features.
pub trait Sdf<Scalar: Float, const DIM: usize, State = ()> {
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

    fn state(&self, point: &[Scalar; DIM]) -> State;

    fn distance_ref<'a>(&self, point: impl Into<&'a [Scalar; DIM]>) -> Scalar
    where
        Scalar: 'a,
    {
        self.distance_from_slice(point.into())
    }

    fn distance(&self, point: impl Into<[Scalar; DIM]>) -> Scalar {
        self.distance_from_slice(&point.into())
    }

    /// Returns the distance and the state of the SDF at a given point at once. This is the
    /// prefered method for fetching both pieces of data, as further optimisations can be performed
    /// when done concurrently. See implementation of combiner SDFs for more information.
    #[inline]
    fn distance_and_state(&self, point: impl Into<[Scalar; DIM]>) -> (Scalar, State) {
        let point = &point.into();
        (self.distance_from_slice(point), self.state(point))
    }

    /// Returns the gradient of the SDF at a given point. The derivatives are calculated with
    /// finite differences, hence the need for epsilon.
    #[inline]
    fn gradient(&self, point: impl Into<[Scalar; DIM]>, epsilon: Scalar) -> [Scalar; DIM] {
        let point = point.into();

        std::array::from_fn(|i| {
            let mut point = point;
            point[i] = point[i] + epsilon;

            self.distance_from_slice(&point)
        })
    }

    /// Returns the normal of the SDF at a given point (the normalized gradient of the field.) For
    /// more information about epslion, see the gradient method on this trait.
    #[inline]
    fn normal(&self, point: impl Into<[Scalar; DIM]>, epsilon: Scalar) -> [Scalar; DIM] {
        let gradient = self.gradient(point, epsilon);

        let norm = gradient
            .iter()
            .map(|e| e.powi(2))
            .fold(Scalar::zero(), |acc, e| acc + e)
            .sqrt();

        let inv_norm = Scalar::one() / norm;
        std::array::from_fn(|i| gradient[i] * inv_norm)
    }
}

pub trait SdfState: Clone {}

impl<T> SdfState for T where T: Clone {}

impl<T, U, Scalar: Float, const DIM: usize, State: SdfState> Sdf<Scalar, DIM, State> for T
where
    T: Deref<Target = U>,
    U: Sdf<Scalar, DIM, State> + 'static,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        self.deref().distance_from_slice(point)
    }

    fn state(&self, point: &[Scalar; DIM]) -> State {
        self.deref().state(point)
    }
}
