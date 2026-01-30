use anyhow::Result;

use crate::{errors, fd::get_fd, magic::KSU_IOCTL_SET_SEPOLICY};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SetSepolicyCmd {
    pub cmd: u64,
    pub arg: u64,
}

/// Set selinux policy
pub fn set_sepolicy(cmd: &SetSepolicyCmd) -> Result<()> {
    let mut cmd = *cmd;
    let ret = unsafe {
        libc::ioctl(
            get_fd() as libc::c_int,
            KSU_IOCTL_SET_SEPOLICY,
            &raw mut cmd,
        )
    };

    if ret < 0 {
        return Err(errors::Error::SetSepolicyFailed {
            source: std::io::Error::last_os_error(),
        }
        .into());
    }

    Ok(())
}
