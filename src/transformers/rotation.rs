// Copyright © 2025 Ambre Guyot
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::Sdf;
use glam::{DQuat, DVec3, Quat, Vec3};
use num::Float;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Rotated2d<Scalar: Float, T>
where
    T: Sdf<Scalar, 2>,
{
    inner: T,
    inverse_rotation: Scalar,
}

impl<Scalar: Float, T> Sdf<Scalar, 2> for Rotated2d<Scalar, T>
where
    T: Sdf<Scalar, 2>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; 2]) -> Scalar {
        let cos_angle = self.inverse_rotation.cos();
        let sin_angle = self.inverse_rotation.sin();
        let point = [
            cos_angle * point[0] - sin_angle * point[1],
            sin_angle * point[0] + cos_angle * point[1],
        ];

        self.inner.distance_from_slice(&point)
    }
}

impl<Scalar: Float, T> Rotated2d<Scalar, T>
where
    T: Sdf<Scalar, 2>,
{
    #[inline]
    pub fn new(inner: T, rotation: Scalar) -> Self {
        Self {
            inner,
            inverse_rotation: -rotation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotated3d<T: Sdf<f32, 3>> {
    inner: T,
    inverse_rotation: Quat,
}

impl<T: Sdf<f32, 3>> Sdf<f32, 3> for Rotated3d<T> {
    #[inline]
    fn distance_from_slice(&self, point: &[f32; 3]) -> f32 {
        let point = (self.inverse_rotation * Vec3::from_array(*point)).to_array();
        self.inner.distance_from_slice(&point)
    }
}

impl<T: Sdf<f32, 3>> Rotated3d<T> {
    #[inline]
    pub fn new(inner: T, rotation: Quat) -> Self {
        Self {
            inner,
            inverse_rotation: rotation.inverse(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DRotated3d<T: Sdf<f64, 3>> {
    inner: T,
    inverse_rotation: DQuat,
}

impl<T: Sdf<f64, 3>> Sdf<f64, 3> for DRotated3d<T> {
    #[inline]
    fn distance_from_slice(&self, point: &[f64; 3]) -> f64 {
        let point = (self.inverse_rotation * DVec3::from_array(*point)).to_array();
        self.inner.distance_from_slice(&point)
    }
}

impl<T: Sdf<f64, 3>> DRotated3d<T> {
    #[inline]
    pub fn new(inner: T, rotation: DQuat) -> Self {
        Self {
            inner,
            inverse_rotation: rotation.inverse(),
        }
    }
}

pub trait SdfRotation2dOperations<Scalar: Float>: Sdf<Scalar, 2> + Sized {
    #[inline]
    fn rotate_2d(self, rotation: Scalar) -> Rotated2d<Scalar, Self> {
        Rotated2d::new(self, rotation)
    }
}

pub trait SdfRotation3dOperations: Sdf<f32, 3> + Sized {
    #[inline]
    fn rotate(self, rotation: Quat) -> Rotated3d<Self> {
        Rotated3d::new(self, rotation)
    }
}

pub trait SdfDRotation3dOperations: Sdf<f64, 3> + Sized {
    #[inline]
    fn rotate_64(self, rotation: DQuat) -> DRotated3d<Self> {
        DRotated3d::new(self, rotation)
    }
}

impl<T, Scalar: Float> SdfRotation2dOperations<Scalar> for T where Self: Sdf<Scalar, 2> + Sized {}
impl<T> SdfRotation3dOperations for T where Self: Sdf<f32, 3> + Sized {}
impl<T> SdfDRotation3dOperations for T where Self: Sdf<f64, 3> + Sized {}
