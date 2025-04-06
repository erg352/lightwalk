use crate::Sdf;
use num::Float;

pub struct Cube;

impl<Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for Cube {
    #[inline]
    fn distance(&self, point: &[Scalar; DIM]) -> Scalar {
        point
            .iter()
            .map(|axis| axis.abs() - Scalar::from(0.5).unwrap())
            .reduce(|acc, e| acc.max(e))
            .unwrap_or(Scalar::zero())
    }
}
