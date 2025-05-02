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

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
                .fold(Scalar::zero(), |acc, e| acc + e)
                .sqrt()
        };

        value - Scalar::one()
    }
}
