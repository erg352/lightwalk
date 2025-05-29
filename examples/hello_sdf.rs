use std::f32::consts::PI;

// The prelude contains all the types and traits we need to use lightweight and is recommended to
// avoid an overabundance of 'use' statements.
use lightwalk::{prelude::*, sdf::shader::GpuSdf};

fn main() {
    // We can define SDFs using primitives like spheres and cubes, and transforming them using a
    // bunch of operations such as scaling and translating. Some such operations might limit the
    // 'scope' of the SDF. As an example, we are translating by a 2D vector, and as such, the SDF
    // will be in 2D. If we translated with a 3D Vector, we would have had a 3D SDF and so on.
    let sphere = Sphere.scale(4.).translate(&[1., 2.]);
    let cube = Cube
        .scale(2.)
        .round(0.3)
        .rotate_2d(PI / 4.)
        .translate(&[-3., 4.]);

    // SDFs can be created from primitives without thickness, but it may be wise to specify a
    // thickness!
    let line = line([-1., 1.]).thickness(0.4);

    // We can compose multiple SDFs together so long as they are in the same dimension (here, they
    // are both 2D) and have the same scalar type. (Or float type. Here, both are f32s.)
    let combined = sphere.add(cube).add(line);

    // We can sample the SDF at a point in space by calling the distance function (or others, like
    // distance_ref) to get the distance at said point to the shape we described earlier.
    let distance_to_scene = combined.distance([-2., -2.]);

    println!("Distance to scene: {distance_to_scene}");

    let sdf = Plane::new([-1f32, 1f32]);

    println!("{}", sdf.gpu_fn_name());
}
