#![cfg(feature = "with-image")]
#![cfg(feature = "device-test")]

use anyhow::Result;
use image::ImageFormat;
use lazy_static::lazy_static;
use realsense_rust::{prelude::*, Config, ExtendedFrame, Format, Pipeline, Resolution, StreamKind};
use std::{sync::Mutex, time::Duration};
use tokio::runtime::Runtime;
type Fallible<T> = Result<T, anyhow::Error>;

lazy_static! {
    /// this lock prevenst multiple tests control RealSense device concurrently
    static ref GLOBAL_MUTEX: Mutex<usize> = Mutex::new(0);
}

#[test]
fn async_test() -> Result<()> {
    // lock global mutex
    let mut counter = GLOBAL_MUTEX.lock().unwrap();

    // init async runtime
    let runtime = Runtime::new()?;

    runtime.block_on(async {
        // init pipeline
        let pipeline = Pipeline::new()?;
        let config = Config::new()?
            .enable_stream(StreamKind::Depth, 0, 640, 0, Format::Z16, 30)?
            .enable_stream(StreamKind::Color, 0, 640, 0, Format::Rgb8, 30)?;
        let mut pipeline = pipeline.start_async(config).await?;

        // show stream info
        let profile = pipeline.profile();
        for (idx, stream_result) in profile.streams()?.try_into_iter()?.enumerate() {
            let stream = stream_result?;
            println!("stream data {}: {:#?}", idx, stream.get_data()?);
        }

        // process frames
        for _ in 0..16 {
            let timeout = Duration::from_millis(1000);
            let frames = match pipeline.wait_async(timeout).await? {
                Some(frame) => frame,
                None => {
                    println!("timeout error");
                    continue;
                }
            };

            println!("frame number = {}", frames.number()?);

            for frame_result in frames.try_into_iter()? {
                let frame_any = frame_result?;

                match frame_any.try_extend()? {
                    ExtendedFrame::Video(frame) => {
                        let image = frame.owned_image()?;
                        image.save_with_format(
                            format!("async-video-example-{}.png", frame.number()?),
                            ImageFormat::Png,
                        )?;
                    }
                    ExtendedFrame::Depth(frame) => {
                        let Resolution { width, height } = frame.resolution()?;
                        let distance = frame.distance(width / 2, height / 2)?;
                        println!("distance = {} meter", distance);

                        let image = frame.owned_image()?;
                        image.save_with_format(
                            format!("async-depth-example-{}.png", frame.number()?),
                            ImageFormat::Png,
                        )?;
                    }
                    _ => unreachable!(),
                }
            }
        }

        Fallible::Ok(())
    })?;

    *counter += 1;
    Ok(())
}

#[test]
fn sync_test() -> Result<()> {
    // lock global mutex
    let mut counter = GLOBAL_MUTEX.lock().unwrap();

    // init pipeline
    let pipeline = Pipeline::new()?;
    let config = Config::new()?
        .enable_stream(StreamKind::Depth, 0, 640, 0, Format::Z16, 30)?
        .enable_stream(StreamKind::Color, 0, 640, 0, Format::Rgb8, 30)?;
    let mut pipeline = pipeline.start(config)?;
    let profile = pipeline.profile();

    // show stream info
    for (idx, stream_result) in profile.streams()?.try_into_iter()?.enumerate() {
        let stream = stream_result?;
        println!("stream data {}: {:#?}", idx, stream.get_data()?);
    }

    // process frames
    for _ in 0..16 {
        let timeout = Duration::from_millis(1000);
        let frames = match pipeline.wait(timeout)? {
            Some(frame) => frame,
            None => {
                println!("timeout error");
                continue;
            }
        };

        println!("frame number = {}", frames.number()?);

        for frame_result in frames.try_into_iter()? {
            let frame_any = frame_result?;

            match frame_any.try_extend()? {
                ExtendedFrame::Video(frame) => {
                    let image = frame.owned_image()?;
                    image.save_with_format(
                        format!("sync-video-example-{}.png", frame.number()?),
                        ImageFormat::Png,
                    )?;
                }
                ExtendedFrame::Depth(frame) => {
                    let Resolution { width, height } = frame.resolution()?;
                    let distance = frame.distance(width / 2, height / 2)?;
                    println!("distance = {}", distance);

                    let image = frame.owned_image()?;
                    image.save_with_format(
                        format!("sync-depth-example-{}.png", frame.number()?),
                        ImageFormat::Png,
                    )?;
                }
                _ => unreachable!(),
            }
        }
    }

    *counter += 1;
    Ok(())
}
