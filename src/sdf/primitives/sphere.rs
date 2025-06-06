use crate::Sdf;
use num::Float;

/// A SDF Primitive of a sphere of radius 1 centered at the origin. Both the radius and the center
/// can be modified by scaling and translating the SDF (see
/// [this trait](`crate::prelude::SdfTransformOperations`)) for the methods needed to achieve this.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sphere;

impl<Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for Sphere {
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let value: Scalar = if DIM == 1 {
            // The squaring of the scalar followed by the square root may not get optimized out
            // when DIM == 1. The if statement is optimized out as DIM is constant with regards to
            // the function.
            point[0].abs()
        } else {
            point
                .iter()
                .map(|scalar| *scalar * *scalar)
                .fold(Scalar::zero(), |acc, e| acc + e) // .sum() isn't defined for floats.
                .sqrt()
        };

        value - Scalar::one()
    }

    #[inline]
    fn state(&self, _: &[Scalar; DIM]) {}
}
