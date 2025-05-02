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

use num::Float;

use crate::Sdf;

pub struct Line<Scalar: Float, const DIM: usize> {
    direction: [Scalar; DIM],
}

impl<Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for Line<Scalar, DIM> {
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let dot = point
            .iter()
            .zip(self.direction.iter())
            .fold(Scalar::zero(), |acc, (&lhs, &rhs)| acc + lhs * rhs);

        (1..DIM)
            .map(|i| point[i] - self.direction[i] * dot)
            .fold(Scalar::zero(), |acc, e| acc + e * e)
            .sqrt()
    }
}

impl<Scalar: Float, const DIM: usize> Line<Scalar, DIM> {
    /// # Safety
    /// This function does not verify that the direction is normalized, which should always be
    /// true.
    #[inline]
    pub unsafe fn new_unchecked(direction: [Scalar; DIM]) -> Self {
        Self { direction }
    }

    pub fn new(mut direction: [Scalar; DIM]) -> Self {
        let norm = direction
            .iter()
            .map(|e| *e * *e)
            .fold(Scalar::zero(), |acc, e| acc + e)
            .sqrt();

        if norm == Scalar::zero() {
            panic!("Cannot define a plane with a null normal");
        }

        let inverse_norm = Scalar::one() / norm;

        for scalar in &mut direction {
            *scalar = *scalar * inverse_norm;
        }

        // Safety: We just normalized the input, so we are good to go!
        unsafe { Self::new_unchecked(direction) }
    }
}

#[inline]
pub fn line<Scalar: Float, const DIM: usize>(direction: [Scalar; DIM]) -> Line<Scalar, DIM> {
    Line::new(direction)
}
