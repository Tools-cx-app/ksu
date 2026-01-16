use anyhow::Result;

use crate::{
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

impl Features {
    pub fn new() -> Self {
        Self {
            feature_id: None,
            value: None,
        }
    }

    pub fn get(&self) -> Result<(u64, bool)> {
        let Some(feature_id) = self.feature_id else {
            return Err(anyhow::anyhow!("No set feature_id!!"));
        };

        let mut cmd = GetFeatureCmd {
            feature_id,
            value: 0,
            supported: 0,
        };

        let ret =
            unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_GET_FEATURE, &raw mut cmd) };
        if ret < 0 {
            return Err(anyhow::anyhow!(
                "Failed to get feature_id {}, Err: {}",
                feature_id,
                std::io::Error::last_os_error()
            ));
        }

        Ok((cmd.value, cmd.supported != 0))
    }

    pub fn set(&self) -> Result<()> {
        let Some(feature_id) = self.feature_id else {
            return Err(anyhow::anyhow!("No set feature_id!!"));
        };
        let Some(value) = self.value else {
            return Err(anyhow::anyhow!("No set value!!"));
        };

        let cmd = SetFeatureCmd { feature_id, value };

        let ret = unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_SET_FEATURE, &cmd) };
        if ret < 0 {
            return Err(anyhow::anyhow!(
                "Failed to set feature_id {} to {}, Err: {}",
                feature_id,
                value,
                std::io::Error::last_os_error()
            ));
        }

        Ok(())
    }
}
