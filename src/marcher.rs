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

pub struct Ray<Scalar: Float, const DIM: usize> {
    pub origin: [Scalar; DIM],
    pub direction: [Scalar; DIM],
}

pub struct Marcher<Scalar: Float, T, const DIM: usize>
where
    T: Sdf<Scalar, DIM>,
{
    pub max_ray_distance: Scalar,
    pub surface_distance: Scalar,
    pub max_iter_count: u32,

    pub surface: T,
}

pub struct Collision<Scalar: Float> {
    pub distance: Scalar,
}

impl<Scalar: Float, T, const DIM: usize> Marcher<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    pub fn march(&self, ray: &Ray<Scalar, DIM>) -> Option<Collision<Scalar>> {
        let mut total_distance = Scalar::zero();
        let mut position = ray.origin;

        for _ in 0..self.max_iter_count {
            if total_distance > self.max_ray_distance {
                return None;
            }

            let distance = self.surface.distance(&position);

            if distance < self.surface_distance {
                return Some(Collision {
                    distance: total_distance,
                });
            }

            total_distance = total_distance + distance;

            for (i, axis) in position.iter_mut().enumerate() {
                *axis = *axis + ray.direction[i];
            }
        }

        None
    }
}
