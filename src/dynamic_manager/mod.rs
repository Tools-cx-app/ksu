mod errors;
mod magic;

use anyhow::Result;

use crate::{dynamic_manager::magic::SUKISU_IOCTL_DYNAMIC_MANAGER, fd::get_fd};

#[repr(C)]
#[derive(Clone, Copy)]
struct DynamicManageCmd {
    operation: u32,
    size: u32,
    hash: [u8; 64],
}

pub struct DynamicManage {
    size: Option<u32>,
    hash: Option<[u8; 64]>,
}

impl Default for DynamicManage {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicManage {
    pub fn new() -> Self {
        Self {
            size: None,
            hash: None,
        }
    }

    pub fn size<U>(&mut self, size: U) -> &mut Self
    where
        U: Into<u32>,
    {
        self.size = Some(size.into());
        self
    }

    pub fn hash(&mut self, hash: [u8; 64]) -> &mut Self {
        self.hash = Some(hash);
        self
    }

    pub fn set(&self) -> Result<(), errors::Error> {
        let Some(size) = self.size else {
            return Err(errors::Error::MissingSize);
        };
        let Some(hash) = self.hash else {
            return Err(errors::Error::MissingHash);
        };

        let cmd = DynamicManageCmd {
            operation: 0,
            size,
            hash,
        };

        let ret =
            unsafe { libc::ioctl(get_fd() as libc::c_int, SUKISU_IOCTL_DYNAMIC_MANAGER, &cmd) };
        if ret < 0 {
            return Err(errors::Error::Set {
                source: std::io::Error::last_os_error(),
            });
        }
        Ok(())
    }

    pub fn get(&self) -> Result<(u32, [u8; 64]), errors::Error> {
        let mut cmd = DynamicManageCmd {
            operation: 1,
            size: 0,
            hash: [0u8; 64],
        };

        let ret = unsafe {
            libc::ioctl(
                get_fd() as libc::c_int,
                SUKISU_IOCTL_DYNAMIC_MANAGER,
                &raw mut cmd,
            )
        };
        if ret < 0 {
            return Err(errors::Error::Get {
                source: std::io::Error::last_os_error(),
            });
        }
        Ok((cmd.size, cmd.hash))
    }

    pub fn clear(&self) -> Result<(), errors::Error> {
        let cmd = DynamicManageCmd {
            operation: 2,
            size: 0,
            hash: [0u8; 64],
        };

        let ret =
            unsafe { libc::ioctl(get_fd() as libc::c_int, SUKISU_IOCTL_DYNAMIC_MANAGER, &cmd) };
        if ret < 0 {
            return Err(errors::Error::Clear {
                source: std::io::Error::last_os_error(),
            });
        }

        Ok(())
    }
}
