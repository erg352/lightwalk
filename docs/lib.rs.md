`lightwalk` is a simple raymarching library used to expressively define SDFs in a efficient manner.

At it's core, this crate defines a trait called [`Sdf`]. In the context of documentation, an SDF is any value or type that implements this trait.
    SDFs are split into three categories: _primitives_, _transformers_ and _combinators_.
    - A _primitive_ is a simple mathematical shape we can trivially find an SDF for (a sphere, a cube...). They are always centered at the origin, and their
    rotation and scale are whichever is most natural for the given shape. For example, a sphere's radius or a cube's side length both equal one, and the cube
    has no rotation.
    - A _transformer_ is a simple function we can apply to transform a SDF into another. Transformers can be as simple as translations or rotations, but can also
    be a little more complex, such as mirrors or spatial repeaters.
    - A _combinator_ combines multiple SDFs into a single SDF. Operations such as _unions_ and _intersections_ fall into this category.
