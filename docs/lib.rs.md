`lightwalk` is a simple raymarching library used to expressively define SDFs in a efficient manner.

At it's core, this crate defines a trait called [`Sdf`]. In the context of documentation, an SDF is any value or type that implements this trait.
    SDFs are split into three categories: _primitives_, _transformers_ and _combinators_.
- A _primitive_ is a simple mathematical shape we can trivially find an SDF for (a sphere, a cube...). They are always centered at the origin, and their
    rotation and scale are whichever is most natural for the given shape. For example, a sphere's radius or a cube's side length both equal one, and the cube
    has no rotation.
- A _transformer_ is a simple function we can apply to transform a SDF into another. Transformers can be as simple as translations or rotations, but can also
    be a little more complex, such as mirrors or spatial repeaters.
- A _combinator_ combines multiple SDFs into a single SDF. Operations such as _unions_ and _intersections_ fall into this category.

```rust
use lightwalk::prelude::*;

// Expressively create SDFs using a combination of primitives,
// combinators and transformers.
let sdf = Sphere.translate(&[1.0, 2.0, 3.0]).add(Cube.round(0.5));

// We can grab the distance from any point in the world.
let distance = sdf.distance(&[0.5, 0.3, 0.8]);

// Create a marcher to perform ray marching on the SDF.
let marcher = Marcher {
    max_ray_distance: 100.0,
    surface_distance: 0.01,
    max_iter_count: 100,
    surface: sdf,
};

let ray = Ray {
    origin: [0.5, 0.3, 0.8],
    direction: [-1.0, 0.0, 0.0],
};

// perform the ray marching!
if let Some(collision) = marcher.march(&ray) {
    println!("Distance from origin: {}", collision.distance);
}

```
