use std::{fs::File, io::Write, ops::Range};

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

            let distance = self.surface.distance_from_slice(&position);

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

    pub fn trace_to_ppm(&self, mut file: File, res: usize) -> std::io::Result<()> {
        file.set_len(0)?;
        {
            let header = format!("P6\n{0}{0}\n255\n", res);
            file.write_all(header.as_bytes())?;
        }

        for [_x, _y] in iter_pairs(&[0..res, 0..res]) {
            #[allow(unused)]
            let ray = Ray {
                origin: todo!(),
                direction: todo!(),
            };

            let color = match self.march(&ray) {
                Some(_collision) => [255; 3],
                None => [0; 3],
            };

            file.write_all(&color)?;
        }

        Ok(())
    }
}

fn iter_pairs(range: &[Range<usize>; 2]) -> impl Iterator<Item = [usize; 2]> {
    // clones are cheap 16 byte stack copies
    range[1]
        .clone()
        .flat_map(|x| range[0].clone().map(move |y| [x, y]))
}
