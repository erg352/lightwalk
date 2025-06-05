use crate::Sdf;
use num::Float;

/// A SDF Primitive of a hyperplane with a given normal. Points who's dot product with the normal
/// is positive will be "outside" the SDF shape and the other points will be "Inside". The
/// hyperplane passes by the origin of the World.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Plane<Scalar: Float, const DIM: usize> {
    normal: [Scalar; DIM],
}

impl<Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for Plane<Scalar, DIM> {
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        // Simply perform the dot product between the normal of plane and the point.
        point
            .iter()
            .zip(self.normal.iter())
            .fold(Scalar::zero(), |acc, e| acc + *e.0 * *e.1)
    }

    fn state(&self, _: &[Scalar; DIM]) {}
}

impl<Scalar: Float, const DIM: usize> Plane<Scalar, DIM> {
    /// # Safety
    /// This function does not verify that the normal is normalized, which should always be
    /// true.
    pub unsafe fn new_unchecked(normal: [Scalar; DIM]) -> Self {
        Self { normal }
    }

    pub fn new(mut normal: [Scalar; DIM]) -> Self {
        let norm = normal
            .iter()
            .map(|e| *e * *e)
            .fold(Scalar::zero(), |acc, e| acc + e)
            .sqrt();

        if norm == Scalar::zero() {
            panic!("Cannot define a plane with a null normal");
        }

        let inverse_norm = Scalar::one() / norm;

        for scalar in &mut normal {
            *scalar = *scalar * inverse_norm;
        }

        // Safety: We just normalized the input, so we are good to go!
        unsafe { Self::new_unchecked(normal) }
    }
}
