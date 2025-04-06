use std::hint::black_box;

use lightwalk::{Sdf, prelude::*};

#[inline(never)]
fn inner_fn(value: f32) -> f32 {
    Sphere.distance(&[value, value * 2.0])
}

fn main() {
    let value = 1.0;
    let value = black_box(inner_fn(black_box(value)));
    println!("{value}");
}
