//! Marker types and traits for [Frame](crate::frame::Frame).

use crate::{common::*, kind::Rs2Extension};

/// The marker trait for frame kinds.
pub trait FrameKind {}

/// The marker traits for frame kinds except [Any](Any).
pub trait NonAnyFrameKind
where
    Self: FrameKind,
{
    const EXTENSION: Rs2Extension;
}

#[derive(Debug)]
pub struct Composite;

impl FrameKind for Composite {}
impl NonAnyFrameKind for Composite {
    const EXTENSION: Rs2Extension = Rs2Extension::CompositeFrame;
}

#[derive(Debug)]
pub struct Any;

impl FrameKind for Any {}

#[derive(Debug)]
pub struct Video;

impl FrameKind for Video {}
impl NonAnyFrameKind for Video {
    const EXTENSION: Rs2Extension = Rs2Extension::VideoFrame;
}

#[derive(Debug)]
pub struct Motion;

impl FrameKind for Motion {}
impl NonAnyFrameKind for Motion {
    const EXTENSION: Rs2Extension = Rs2Extension::MotionFrame;
}

#[derive(Debug)]
pub struct Depth;

impl FrameKind for Depth {}
impl NonAnyFrameKind for Depth {
    const EXTENSION: Rs2Extension = Rs2Extension::DepthFrame;
}

#[derive(Debug)]
pub struct Disparity;

impl FrameKind for Disparity {}
impl NonAnyFrameKind for Disparity {
    const EXTENSION: Rs2Extension = Rs2Extension::DisparityFrame;
}

#[derive(Debug)]
pub struct Pose;

impl FrameKind for Pose {}
impl NonAnyFrameKind for Pose {
    const EXTENSION: Rs2Extension = Rs2Extension::PoseFrame;
}

#[derive(Debug)]
pub struct Points;

impl FrameKind for Points {}
impl NonAnyFrameKind for Points {
    const EXTENSION: Rs2Extension = Rs2Extension::Points;
}
