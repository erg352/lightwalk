use lightwalk::prelude::*;

#[unsafe(no_mangle)]
#[inline(never)]
fn build_sdf(scale: f32, angle: f32, translation: [f32; 2]) -> impl Sdf<f32, 2> {
    Cube.scale(scale).rotate_2d(angle).translate(&translation)
}

#[inline(never)]
fn calculate_distance(sdf: impl Sdf<f32, 2>, position: [f32; 2]) -> f32 {
    sdf.distance(position)
}

fn main() {
    let scale = 0.4;
    let angle = 0.3;
    let position = [0.5, 0.6];
    let sdf = build_sdf(scale, angle, position);
    println!("{}", calculate_distance(sdf, [1., 2.]));
}
