use anyhow::Result;

use crate::{errors, fd::get_fd, magic::KSU_IOCTL_MANAGE_MARK};

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct ManageMarkCmd {
    operation: u32,
    pid: i32,
    result: u32,
}

/// Mark Manager
pub struct MarkManager {
    pid: Option<i32>,
}

impl MarkManager {
    pub fn new() -> Self {
        Self { pid: None }
    }

    /// set pid
    pub fn pid(&mut self, pid: i32) -> &mut Self {
        self.pid = Some(pid);
        self
    }

    ///Mark a process (pid=0 marks all processes)
    pub fn set(&self) -> Result<()> {
        let Some(pid) = self.pid else {
            return Err(errors::Error::MarkissingPid.into());
        };

        let cmd = ManageMarkCmd {
            operation: 2,
            pid,
            result: 0,
        };

        let ret = unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_MANAGE_MARK, &cmd) };
        if ret < 0 {
            return Err(errors::Error::MarkSetFailed {
                pid,
                source: std::io::Error::last_os_error(),
            }
            .into());
        }

        Ok(())
    }

    /// Get mark status for a process (pid=0 returns total marked count)
    pub fn get(&self) -> Result<u32> {
        let Some(pid) = self.pid else {
            return Err(errors::Error::MarkissingPid.into());
        };

        let mut cmd = ManageMarkCmd {
            operation: 1,
            pid,
            result: 0,
        };

        let ret =
            unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_MANAGE_MARK, &raw mut cmd) };
        if ret < 0 {
            return Err(errors::Error::MarkGetFailed {
                pid,
                source: std::io::Error::last_os_error(),
            }
            .into());
        }

        Ok(cmd.result)
    }

    /// Unmark a process (pid=0 unmarks all processes)
    pub fn unset(&self) -> Result<()> {
        let Some(pid) = self.pid else {
            return Err(errors::Error::MarkissingPid.into());
        };

        let cmd = ManageMarkCmd {
            operation: 3,
            pid,
            result: 0,
        };

        let ret = unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_MANAGE_MARK, &cmd) };
        if ret < 0 {
            return Err(errors::Error::MarkUnSetFailed {
                pid,
                source: std::io::Error::last_os_error(),
            }
            .into());
        }

        Ok(())
    }

    /// Refresh mark for all running processes
    pub fn refresh(&self) -> Result<()> {
        let cmd = ManageMarkCmd {
            operation: 4,
            pid: 0,
            result: 0,
        };

        let ret = unsafe { libc::ioctl(get_fd() as libc::c_int, KSU_IOCTL_MANAGE_MARK, &cmd) };
        if ret < 0 {
            return Err(errors::Error::MarkRefreshFailed {
                source: std::io::Error::last_os_error(),
            }
            .into());
        }

        Ok(())
    }
}
