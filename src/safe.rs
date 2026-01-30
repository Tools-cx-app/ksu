use anyhow::Result;

use crate::{errors, fd::get_fd, magic::KSU_IOCTL_CHECK_SAFEMODE};

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CheckSafemodeCmd {
    in_safe_mode: u8,
}

/// Check is safe mode
pub fn safemode() -> Result<bool> {
    let mut cmd = CheckSafemodeCmd { in_safe_mode: 0 };
    let ret = unsafe {
        libc::ioctl(
            get_fd() as libc::c_int,
            KSU_IOCTL_CHECK_SAFEMODE,
            &raw mut cmd,
        )
    };

    if ret < 0 {
        return Err(errors::Error::GetSafeModeFailed {
            source: std::io::Error::last_os_error(),
        }
        .into());
    }
    Ok(cmd.in_safe_mode != 0)
}
