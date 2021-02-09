//! Defines the device types.

use crate::{
    common::*,
    error::{ErrorChecker, Result},
    kind::Rs2CameraInfo,
    sensor::{
        ColorSensor, DepthSensor, DepthStereoSensor, FishEyeSensor, L500DepthSensor, MotionSensor,
        PoseSensor, SoftwareSensor, Tm2Sensor,
    },
    sensor_list::SensorList,
};

/// Represents a device instance.
#[derive(Debug)]
pub struct Device {
    pub(crate) ptr: NonNull<sys::rs2_device>,
}

impl Device {
    /// Discover available sensors on device.
    pub fn sensors(&self) -> Result<SensorList> {
        let list = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = sys::rs2_query_sensors(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            SensorList::from_raw(ptr)
        };
        Ok(list)
    }

    pub fn hardware_reset(&self) -> Result<()> {
        unsafe {
            let mut checker = ErrorChecker::new();
            sys::rs2_hardware_reset(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
        }
        Ok(())
    }

    pub fn first_color_sensor(self) -> Result<Option<ColorSensor>> {
        self.sensors()?.first_color_sensor()
    }

    pub fn first_depth_sensor(self) -> Result<Option<DepthSensor>> {
        self.sensors()?.first_depth_sensor()
    }

    pub fn first_depth_stereo_sensor(self) -> Result<Option<DepthStereoSensor>> {
        self.sensors()?.first_depth_stereo_sensor()
    }

    pub fn first_fish_eye_sensor(self) -> Result<Option<FishEyeSensor>> {
        self.sensors()?.first_fish_eye_sensor()
    }

    pub fn first_l500_depth_sensor(self) -> Result<Option<L500DepthSensor>> {
        self.sensors()?.first_l500_depth_sensor()
    }

    pub fn first_motion_sensor(self) -> Result<Option<MotionSensor>> {
        self.sensors()?.first_motion_sensor()
    }

    pub fn first_pose_sensor(self) -> Result<Option<PoseSensor>> {
        self.sensors()?.first_pose_sensor()
    }

    pub fn first_software_sensor(self) -> Result<Option<SoftwareSensor>> {
        self.sensors()?.first_software_sensor()
    }

    pub fn first_tm2_sensor(self) -> Result<Option<Tm2Sensor>> {
        self.sensors()?.first_tm2_sensor()
    }

    pub fn name(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::Name)
    }

    pub fn serial_number(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::SerialNumber)
    }

    pub fn recommended_firmware_version(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::RecommendedFirmwareVersion)
    }

    pub fn physical_port(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::PhysicalPort)
    }

    pub fn debug_op_code(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::DebugOpCode)
    }

    pub fn advanced_mode(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::AdvancedMode)
    }

    pub fn product_id(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::ProductId)
    }

    pub fn camera_locked(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::CameraLocked)
    }

    pub fn usb_type_descriptor(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::UsbTypeDescriptor)
    }

    pub fn product_line(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::ProductLine)
    }

    pub fn asic_serial_number(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::AsicSerialNumber)
    }

    pub fn firmware_update_id(&self) -> Result<Option<&str>> {
        self.info(Rs2CameraInfo::FirmwareUpdateId)
    }

    pub fn info(&self, kind: Rs2CameraInfo) -> Result<Option<&str>> {
        if !self.is_info_supported(kind)? {
            return Ok(None);
        }

        let ptr = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = sys::rs2_get_device_info(
                self.ptr.as_ptr(),
                kind as sys::rs2_camera_info,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            ptr
        };

        // TODO: deallicate this CStr?
        let string = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };
        Ok(Some(string))
    }

    pub fn is_info_supported(&self, kind: Rs2CameraInfo) -> Result<bool> {
        let val = unsafe {
            let mut checker = ErrorChecker::new();
            let val = sys::rs2_supports_device_info(
                self.ptr.as_ptr(),
                kind as sys::rs2_camera_info,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            val
        };
        Ok(val != 0)
    }

    pub fn into_raw(self) -> *mut sys::rs2_device {
        let ptr = self.ptr;
        mem::forget(self);
        ptr.as_ptr()
    }

    pub unsafe fn from_raw(ptr: *mut sys::rs2_device) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_device(self.ptr.as_ptr());
        }
    }
}

unsafe impl Send for Device {}
