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
