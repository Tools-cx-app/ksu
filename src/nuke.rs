use std::{
    fmt,
    path::{Path, PathBuf},
};

use anyhow::Result;
use rustix::path::Arg;

use crate::{errors, fd::get_fd, magic::KSU_IOCTL_NUKE_EXT4_SYSFS};

#[repr(C)]
struct NukeExt4SysfsCmd {
    arg: u64,
}

pub struct NukeExt4Sysfs {
    paths: Vec<PathBuf>,
    format_msg: Option<String>,
}

impl NukeExt4Sysfs {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
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

    pub fn format_msg<C, F>(&mut self, style: F) -> &mut Self
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce(&Vec<PathBuf>) -> String,
    {
        self.format_msg = Some(style(&self.paths));
        self
    }

    pub fn execute(&self) -> Result<()> {
        for p in &self.paths {
            log::debug!("{} will umount", p.display());

            let c_path = std::ffi::CString::new(p.as_str()?)?;
            let cmd = NukeExt4SysfsCmd {
                arg: c_path.as_ptr() as u64,
            };

            let ret =
                unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_NUKE_EXT4_SYSFS, &cmd) };

            if ret < 0 {
                return Err(errors::Error::NukeFailed {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nuke() {
        let nuke = NukeExt4Sysfs::new().add("/test").execute();

        assert!(nuke.is_ok());
    }
}
