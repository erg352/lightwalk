mod cube;
mod line;
mod plane;
mod sphere;

pub use cube::Cube;
pub use line::Line;
pub use plane::Plane;
pub use sphere::Sphere;

use num::Float;

#[inline]
pub fn sphere() -> Sphere {
    Sphere(())
}

#[inline]
pub fn cube() -> Cube {
    Cube(())
}

#[inline]
pub fn line<Scalar: Float, const DIM: usize>(direction: [Scalar; DIM]) -> Line<Scalar, DIM> {
    Line::new(direction, ())
}
