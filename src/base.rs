//! Common types and functions.

use crate::kind::{Format, StreamKind};
use image::{Bgr, Bgra, ConvertBuffer, DynamicImage, ImageBuffer, Luma, Rgb, Rgba};
use nalgebra::{Quaternion, Translation3, Unit, UnitQuaternion, Vector3};
use std::{
    convert::{AsMut, AsRef},
    ops::{Deref, DerefMut},
    time::Duration,
};

pub const DEFAULT_TIMEOUT: Duration =
    Duration::from_millis(realsense_sys::RS2_DEFAULT_TIMEOUT as u64);

/// Represents a pose detected by sensor.
#[derive(Debug)]
pub struct PoseData(pub realsense_sys::rs2_pose);

impl PoseData {
    pub fn translation(&self) -> Translation3<f32> {
        let realsense_sys::rs2_vector { x, y, z } = self.0.translation;
        Translation3::new(x, y, z)
    }

    pub fn velocity(&self) -> Vector3<f32> {
        let realsense_sys::rs2_vector { x, y, z } = self.0.velocity;
        Vector3::new(x, y, z)
    }

    pub fn acceleration(&self) -> Vector3<f32> {
        let realsense_sys::rs2_vector { x, y, z } = self.0.acceleration;
        Vector3::new(x, y, z)
    }

    pub fn rotation(&self) -> UnitQuaternion<f32> {
        let realsense_sys::rs2_quaternion { x, y, z, w } = self.0.rotation;
        Unit::new_unchecked(Quaternion::new(w, x, z, y))
    }

    pub fn angular_velocity(&self) -> Vector3<f32> {
        let realsense_sys::rs2_vector { x, y, z } = self.0.angular_velocity;
        Vector3::new(x, y, z)
    }

    pub fn angular_acceleration(&self) -> Vector3<f32> {
        let realsense_sys::rs2_vector { x, y, z } = self.0.angular_acceleration;
        Vector3::new(x, y, z)
    }

    pub fn tracker_confidence(&self) -> u32 {
        self.0.tracker_confidence as u32
    }

    pub fn mapper_confidence(&self) -> u32 {
        self.0.mapper_confidence as u32
    }
}

impl Deref for PoseData {
    type Target = realsense_sys::rs2_pose;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PoseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<realsense_sys::rs2_pose> for PoseData {
    fn as_ref(&self) -> &realsense_sys::rs2_pose {
        &self.0
    }
}

impl AsMut<realsense_sys::rs2_pose> for PoseData {
    fn as_mut(&mut self) -> &mut realsense_sys::rs2_pose {
        &mut self.0
    }
}

unsafe impl Send for PoseData {}
unsafe impl Sync for PoseData {}

/// Contains width and height of a frame.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

/// Image type returned by [Frame::color_image](Frame::color_image).
#[derive(Debug, Clone)]
pub enum Rs2Image<'a> {
    Bgr8(ImageBuffer<Bgr<u8>, &'a [u8]>),
    Bgra8(ImageBuffer<Bgra<u8>, &'a [u8]>),
    Rgb8(ImageBuffer<Rgb<u8>, &'a [u8]>),
    Rgba8(ImageBuffer<Rgba<u8>, &'a [u8]>),
    Luma16(ImageBuffer<Luma<u16>, &'a [u16]>),
}

impl<'a> From<Rs2Image<'a>> for DynamicImage {
    fn from(from: Rs2Image<'a>) -> DynamicImage {
        match from {
            Rs2Image::Bgr8(image) => DynamicImage::ImageBgr8(image.convert()),
            Rs2Image::Bgra8(image) => DynamicImage::ImageBgra8(image.convert()),
            Rs2Image::Rgb8(image) => DynamicImage::ImageRgb8(image.convert()),
            Rs2Image::Rgba8(image) => DynamicImage::ImageRgba8(image.convert()),
            Rs2Image::Luma16(image) => DynamicImage::ImageLuma16(image.convert()),
        }
    }
}

/// Represents the specification of a stream.
#[derive(Debug)]
pub struct StreamProfileData {
    pub stream: StreamKind,
    pub format: Format,
    pub index: usize,
    pub unique_id: i32,
    pub framerate: i32,
}
