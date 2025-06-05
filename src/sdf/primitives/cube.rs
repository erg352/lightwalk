use crate::Sdf;
use num::Float;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cube;

impl<Scalar: Float, const DIM: usize> Sdf<Scalar, DIM> for Cube {
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        point
            .iter()
            .map(|axis| axis.abs() - Scalar::from(0.5).unwrap())
            .reduce(|acc, e| acc.max(e))
            .unwrap_or(Scalar::zero())
    }

    #[inline]
    fn state(&self, _: &[Scalar; DIM]) {}
}
