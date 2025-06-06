use num::Float;

use crate::Sdf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    #[inline]
    fn state(&self, _: &[Scalar; DIM]) {}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn illegal_line_1d() {
        let _line = Line::new([0.0]);
    }

    #[test]
    #[should_panic]
    fn illegal_line_2d() {
        let _line = Line::new([0.0, 0.0]);
    }

    #[test]
    #[should_panic]
    fn illegal_line_3d() {
        let _line = Line::new([0.0, 0.0, 0.0]);
    }

    #[test]
    fn distance_1d() {
        for j in -100..100 {
            if j == 0 {
                continue;
            }

            let line = Line::new([j as f32]);

            for i in -100..100 {
                let f = i as f32;
                assert_eq!(line.distance([f]), 0.0);
            }
        }
    }
}
