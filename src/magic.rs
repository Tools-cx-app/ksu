const K: u32 = b'K' as u32;
pub(crate) const KSU_INSTALL_MAGIC1: u32 = 0xDEADBEEF;
pub(crate) const KSU_INSTALL_MAGIC2: u32 = 0xCAFEBABE;
#[cfg(target_env = "gnu")]
pub(crate) const KSU_IOCTL_ADD_TRY_UMOUNT: u64 = libc::_IOW::<()>(K, 18);
#[cfg(not(target_env = "gnu"))]
pub(crate) const KSU_IOCTL_ADD_TRY_UMOUNT: i32 = libc::_IOW::<()>(K, 18);