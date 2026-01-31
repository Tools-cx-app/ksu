use anyhow::Result;

use crate::{
    errors,
    fd::get_fd,
    magic::{KSU_IOCTL_GET_FEATURE, KSU_IOCTL_SET_FEATURE},
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GetFeatureCmd {
    feature_id: u32,
    value: u64,
    supported: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct SetFeatureCmd {
    feature_id: u32,
    value: u64,
}

pub struct Features {
    feature_id: Option<u32>,
    value: Option<u64>,
}

impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}

impl Features {
    pub fn new() -> Self {
        Self {
            feature_id: None,
            value: None,
        }
    }

    /// Get feature value and support status from kernel
    /// Returns (value, supported)
    pub fn get(&self) -> Result<(u64, bool)> {
        let Some(feature_id) = self.feature_id else {
            return Err(errors::Error::MissingFeatureId.into());
        };

        let mut cmd = GetFeatureCmd {
            feature_id,
            value: 0,
            supported: 0,
        };

        let ret =
            unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_GET_FEATURE, &raw mut cmd) };
        if ret < 0 {
            return Err(errors::Error::GetFeatureFailed {
                id: feature_id,
                source: std::io::Error::last_os_error(),
            }
            .into());
        }

        Ok((cmd.value, cmd.supported != 0))
    }

    /// Set feature value in kernel
    pub fn set(&self) -> Result<()> {
        let Some(feature_id) = self.feature_id else {
            return Err(errors::Error::MissingFeatureId.into());
        };
        let Some(value) = self.value else {
            return Err(errors::Error::MissingFeatureValue.into());
        };

        let cmd = SetFeatureCmd { feature_id, value };

        let ret = unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_SET_FEATURE, &cmd) };
        if ret < 0 {
            return Err(errors::Error::SetFeatureFailed {
                id: feature_id,
                value,
                source: std::io::Error::last_os_error(),
            }
            .into());
        }

        Ok(())
    }
}
