const K: u32 = b'K' as u32;
#[cfg(target_env = "gnu")]
pub(crate) const SUKISU_IOCTL_DYNAMIC_MANAGER: u64 = libc::_IOWR::<()>(K, 103);
#[cfg(not(target_env = "gnu"))]
pub(crate) const SUKISU_IOCTL_DYNAMIC_MANAGER: i32 = libc::_IOWR::<()>(K, 103);
