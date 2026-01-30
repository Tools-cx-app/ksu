use std::{
    fmt,
    path::{Path, PathBuf},
};

use anyhow::Result;
use rustix::path::Arg;

use crate::{errors, fd::get_fd, magic::KSU_IOCTL_ADD_TRY_UMOUNT};

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct AddTryUmountCmd {
    arg: u64,   // char ptr, this is the mountpoint
    flags: u32, // this is the flag we use for it
    mode: u8,   // denotes what to do with it 0:wipe_list 1:add_to_list 2:delete_entry
}

pub struct TryUmount {
    paths: Vec<PathBuf>,
    flags: u32,
    format_msg: Option<String>,
}

impl TryUmount {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            flags: 0,
            format_msg: None,
        }
    }

    pub fn add<S>(&mut self, p: S) -> &mut Self
    where
        S: AsRef<Path>,
    {
        self.paths.push(p.as_ref().to_path_buf());
        self
    }

    pub fn adds<I, S>(&mut self, paths: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for s in paths {
            self.add(s);
        }
        self
    }

    pub fn flags(&mut self, flags: u32) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn umount(&self) -> Result<()> {
        for p in &self.paths {
            log::debug!("{} will umount", p.display());

            let c_path = std::ffi::CString::new(p.as_str()?)?;
            let cmd = AddTryUmountCmd {
                arg: c_path.as_ptr() as u64,
                flags: self.flags,
                mode: 1,
            };

            let ret =
                unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_ADD_TRY_UMOUNT, &cmd) };

            if ret < 0 {
                return Err(errors::Error::TryUmountAddFailed {
                    path: p.as_str()?.to_string(),
                    source: std::io::Error::last_os_error(),
                }
                .into());
            }
        }

        if let Some(s) = self.format_msg.clone() {
            log::debug!("{s}");
        }

        Ok(())
    }

    pub fn del(&self) -> Result<()> {
        for p in &self.paths {
            log::debug!("{} will deleted", p.display());

            let c_path = std::ffi::CString::new(p.as_str()?)?;
            let cmd = AddTryUmountCmd {
                arg: c_path.as_ptr() as u64,
                flags: 2,
                mode: 0,
            };

            let ret =
                unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_ADD_TRY_UMOUNT, &cmd) };

            if ret < 0 {
                return Err(errors::Error::TryUmountDelFailed {
                    path: p.as_str()?.to_string(),
                    source: std::io::Error::last_os_error(),
                }
                .into());
            }
        }

        if let Some(s) = self.format_msg.clone() {
            log::debug!("{s}");
        }

        Ok(())
    }

    pub fn format_msg<C, F>(&mut self, style: F) -> &mut Self
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce(&Vec<PathBuf>) -> C,
    {
        self.format_msg = Some(style(&self.paths).to_string());
        self
    }

    pub fn wipe(&self) -> Result<()> {
        let cmd = AddTryUmountCmd {
            arg: 0,
            flags: 0,
            mode: 0,
        };

        let ret = unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_ADD_TRY_UMOUNT, &cmd) };

        if ret < 0 {
            return Err(errors::Error::TryUmountWipeFailed {
                source: std::io::Error::last_os_error(),
            }
            .into());
        };

        Ok(())
    }
}
