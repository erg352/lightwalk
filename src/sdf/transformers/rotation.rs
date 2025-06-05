use std::marker::PhantomData;

use crate::{Sdf, SdfState, prelude::SdfMapStateOperation, sdf::state::SdfBindStateOperation};
use glam::{DQuat, DVec3, Quat, Vec3};
use num::Float;

/// Rotates a 2D SDF based off of a certain angle given in radiants. This struct should not be used
/// directly, instead it is recommended to use the function
/// [rotate_2d](crate::sdf::transformers::SdfRotation2dOperations::rotate_2d) defined on any 2D
/// SDFs that wraps it with this type.
///
/// # Example:
///
/// ```rust
/// use lightwalk::prelude::*;
///
/// // A 2D cube rotated by 0.4 radiants.
/// let cube = Cube.rotate_2d(0.4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotated2d<Scalar: Float, T, State: SdfState>
where
    T: Sdf<Scalar, 2, State>,
{
    inner: T,

    // NOTE: We Store Sine and Cosine instead of the angle to only perform the trigonometry once instead
    // of on each call to distance. This should SIGNIFICANTLY improve performance, as the call to
    // the trig function is the major bottleneck of 2D rotation.
    //
    // NOTE: Place Sine before Cosine as ___sincosf_stret (found in Apple Silicon) returns Sine in s0 and
    // Cosine in s1. If we have Cosine be placed before Sine, we may need 3 fmov operations to
    // swap the position of the data.
    //
    // TODO: This optimisation is done assuming other operating systems use a similar convention.
    // We should further investigate on Linux and Windows and on different CPU architectures.
    sin: Scalar,
    cos: Scalar,
    _marker: PhantomData<State>,
}

impl<Scalar: Float, T, State: SdfState> Sdf<Scalar, 2, State> for Rotated2d<Scalar, T, State>
where
    T: Sdf<Scalar, 2, State>,
{
    #[inline]
    fn distance_from_slice(&self, point: &[Scalar; 2]) -> Scalar {
        // We need to rotate the input by the inverse of the specified rotation, hence the - sign
        // being on the Y coordinate's sine and not the X coordinate's sine.
        let point = [
            self.cos * point[0] + self.sin * point[1],
            -self.sin * point[0] + self.cos * point[1],
        ];

        self.inner.distance_from_slice(&point)
    }

    #[inline]
    fn state(&self, point: &[Scalar; 2]) -> State {
        self.inner.state(point)
    }
}

impl<Scalar: Float, T, State: SdfState> Rotated2d<Scalar, T, State>
where
    T: Sdf<Scalar, 2, State>,
{
    #[inline]
    pub fn new(inner: T, rotation: Scalar) -> Self {
        Self {
            inner,
            cos: rotation.cos(),
            sin: rotation.sin(),
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, T, U, State: SdfState> SdfBindStateOperation<Scalar, 2, State>
    for Rotated2d<Scalar, T, ()>
where
    T: SdfBindStateOperation<Scalar, 2, State, Output = U> + 'static,
    U: Sdf<Scalar, 2, State> + 'static,
{
    type Output = Rotated2d<Scalar, U, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Rotated2d {
            inner: self.inner.bind(s),
            cos: self.cos,
            sin: self.sin,
            _marker: PhantomData,
        }
    }
}

impl<Scalar: Float, T, U, InState: SdfState, OutState: SdfState>
    SdfMapStateOperation<Scalar, 2, InState, OutState> for Rotated2d<Scalar, T, InState>
where
    T: SdfMapStateOperation<Scalar, 2, InState, OutState, Output = U> + 'static,
    U: Sdf<Scalar, 2, OutState> + 'static,
{
    type Output = Rotated2d<Scalar, U, OutState>;

    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Rotated2d {
            inner: self.inner.map_state(f),
            cos: self.cos,
            sin: self.sin,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotated3d<T: Sdf<f32, 3, State>, State: SdfState> {
    inner: T,
    inverse_rotation: Quat,
    _marker: PhantomData<State>,
}

impl<T: Sdf<f32, 3, State>, State: SdfState> Sdf<f32, 3, State> for Rotated3d<T, State> {
    #[inline]
    fn distance_from_slice(&self, point: &[f32; 3]) -> f32 {
        let point = (self.inverse_rotation * Vec3::from_array(*point)).to_array();
        self.inner.distance_from_slice(&point)
    }

    #[inline]
    fn state(&self, point: &[f32; 3]) -> State {
        self.inner.state(point)
    }
}

impl<T: Sdf<f32, 3, State>, State: SdfState> Rotated3d<T, State> {
    #[inline]
    pub fn new(inner: T, rotation: Quat) -> Self {
        Self {
            inner,
            inverse_rotation: rotation.inverse(),
            _marker: PhantomData,
        }
    }
}

impl<T, U, State: SdfState> SdfBindStateOperation<f32, 3, State> for Rotated3d<T, ()>
where
    T: SdfBindStateOperation<f32, 3, State, Output = U> + 'static,
    U: Sdf<f32, 3, State> + 'static,
{
    type Output = Rotated3d<U, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        Rotated3d {
            inner: self.inner.bind(s),
            inverse_rotation: self.inverse_rotation,
            _marker: PhantomData,
        }
    }
}

impl<T, U, InState: SdfState, OutState: SdfState> SdfMapStateOperation<f32, 3, InState, OutState>
    for Rotated3d<T, InState>
where
    T: SdfMapStateOperation<f32, 3, InState, OutState, Output = U> + 'static,
    U: Sdf<f32, 3, OutState> + 'static,
{
    type Output = Rotated3d<U, OutState>;

    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        Rotated3d {
            inner: self.inner.map_state(f),
            inverse_rotation: self.inverse_rotation,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DRotated3d<T: Sdf<f64, 3, State>, State: SdfState> {
    inner: T,
    inverse_rotation: DQuat,
    _marker: PhantomData<State>,
}

impl<T: Sdf<f64, 3, State>, State: SdfState> Sdf<f64, 3, State> for DRotated3d<T, State> {
    #[inline]
    fn distance_from_slice(&self, point: &[f64; 3]) -> f64 {
        let point = (self.inverse_rotation * DVec3::from_array(*point)).to_array();
        self.inner.distance_from_slice(&point)
    }

    #[inline]
    fn state(&self, point: &[f64; 3]) -> State {
        self.inner.state(point)
    }
}

impl<T: Sdf<f64, 3, State>, State: SdfState> DRotated3d<T, State> {
    #[inline]
    pub fn new(inner: T, rotation: DQuat) -> Self {
        Self {
            inner,
            inverse_rotation: rotation.inverse(),
            _marker: PhantomData,
        }
    }
}

impl<T, U, State: SdfState> SdfBindStateOperation<f64, 3, State> for DRotated3d<T, ()>
where
    T: SdfBindStateOperation<f64, 3, State, Output = U> + 'static,
    U: Sdf<f64, 3, State> + 'static,
{
    type Output = DRotated3d<U, State>;

    #[inline]
    fn bind(self, s: State) -> Self::Output {
        DRotated3d {
            inner: self.inner.bind(s),
            inverse_rotation: self.inverse_rotation,
            _marker: PhantomData,
        }
    }
}

impl<T, U, InState: SdfState, OutState: SdfState> SdfMapStateOperation<f64, 3, InState, OutState>
    for DRotated3d<T, InState>
where
    T: SdfMapStateOperation<f64, 3, InState, OutState, Output = U> + 'static,
    U: Sdf<f64, 3, OutState> + 'static,
{
    type Output = DRotated3d<U, OutState>;

    fn map_state(self, f: impl FnOnce(InState) -> OutState) -> Self::Output {
        DRotated3d {
            inner: self.inner.map_state(f),
            inverse_rotation: self.inverse_rotation,
            _marker: PhantomData,
        }
    }
}

pub trait SdfRotation2dOperations<Scalar: Float, State: SdfState>:
    Sdf<Scalar, 2, State> + Sized
{
    #[inline]
    fn rotate_2d(self, rotation: Scalar) -> Rotated2d<Scalar, Self, State> {
        Rotated2d::new(self, rotation)
    }
}

pub trait SdfRotation3dOperations<State: SdfState>: Sdf<f32, 3, State> + Sized {
    #[inline]
    fn rotate(self, rotation: Quat) -> Rotated3d<Self, State> {
        Rotated3d::new(self, rotation)
    }
}

pub trait SdfDRotation3dOperations<State: SdfState>: Sdf<f64, 3, State> + Sized {
    #[inline]
    fn rotate_64(self, rotation: DQuat) -> DRotated3d<Self, State> {
        DRotated3d::new(self, rotation)
    }
}

impl<T, Scalar: Float, State: SdfState> SdfRotation2dOperations<Scalar, State> for T where
    Self: Sdf<Scalar, 2, State> + Sized
{
}

impl<T, State: SdfState> SdfRotation3dOperations<State> for T where Self: Sdf<f32, 3, State> + Sized {}
impl<T, State: SdfState> SdfDRotation3dOperations<State> for T where Self: Sdf<f64, 3, State> + Sized
{}
