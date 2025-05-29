use crate::Sdf;
use num::Float;

/// Adds thickness to the SDF. All points who's absolute distance to the surface of the SDF are
/// less than the specified thickness are "inside" the new SDF, and all other points are "outside".
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialze))]
pub struct Thickened<Scalar: Float, T: Sdf<Scalar, DIM>, const DIM: usize> {
    inner: T,
    thickness: Scalar,
}

impl<Scalar: Float, T, const DIM: usize> Sdf<Scalar, DIM> for Thickened<Scalar, T, DIM>
where
    T: Sdf<Scalar, DIM>,
{
    fn distance_from_slice(&self, point: &[Scalar; DIM]) -> Scalar {
        let base_sdf = self.inner.distance_from_slice(point);

        base_sdf.abs() - self.thickness
    }
}

impl<Scalar: Float, T: Sdf<Scalar, DIM>, const DIM: usize> Thickened<Scalar, T, DIM> {
    pub fn new(inner: T, thickness: Scalar) -> Self {
        Self { inner, thickness }
    }
}
