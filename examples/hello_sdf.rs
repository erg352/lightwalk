use std::f32::consts::PI;

// The prelude contains all the types and traits we need to use lightweight and is recommended to
// avoid an overabundance of 'use' statements.
use lightwalk::prelude::*;

#[derive(Clone, Copy)]
enum Color {
    White,
}

fn main() {
    // We can define SDFs using primitives like spheres and cubes, and transforming them using a
    // bunch of operations such as scaling and translating. Some such operations might limit the
    // 'scope' of the SDF. As an example, we are translating by a 2D vector, and as such, the SDF
    // will be in 2D. If we translated with a 3D Vector, we would have had a 3D SDF and so on.
    let s = sphere().scale(4.).translate(&[1., 2.]);
    let c = cube()
        .scale(2.)
        .round(0.3)
        .rotate_2d(PI / 4.)
        .translate(&[-3., 4.]);

    // We can compose multiple SDFs together so long as they are in the same dimension (here, they
    // are both 2D), the same scalar type and the same state (if any).
    let combined = sphere()
        .translate(&[0.1, 0.4])
        .add(cube().rotate_2d(5_f32.to_radians()))
        .mul(line([-1.0, 1.0]).thickness(0.1).translate(&[0.1, 0.2]))
        .bind(Color::White);

    // We can sample the SDF at a point in space by calling the distance function (or others, like
    // distance_ref) to get the distance at said point to the shape we described earlier.
    let (distance_to_scene, _state) = combined.distance_and_state([-2., -2.]);

    println!("Distance to scene: {distance_to_scene}");
}
