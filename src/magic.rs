const K: u32 = b'K' as u32;
pub(crate) const KSU_INSTALL_MAGIC1: u32 = 0xDEADBEEF;
pub(crate) const KSU_INSTALL_MAGIC2: u32 = 0xCAFEBABE;
#[cfg(target_env = "gnu")]
pub(crate) const KSU_IOCTL_GET_INFO: u64 = libc::_IOR::<()>(K, 2);
#[cfg(not(target_env = "gnu"))]
pub(crate) const KSU_IOCTL_GET_INFO: i32 = libc::_IOR::<()>(K, 2);

#[cfg(target_env = "gnu")]
pub(crate) const KSU_IOCTL_GET_FEATURE: u64 = libc::_IOWR::<()>(K, 13);
#[cfg(not(target_env = "gnu"))]
pub(crate) const KSU_IOCTL_GET_FEATURE: i32 = libc::_IOWR::<()>(K, 13);

#[cfg(not(target_env = "gnu"))]
pub(crate) const KSU_IOCTL_SET_FEATURE: i32 = libc::_IOW::<()>(K, 14);
#[cfg(target_env = "gnu")]
pub(crate) const KSU_IOCTL_SET_FEATURE: u64 = libc::_IOW::<()>(K, 14);

#[cfg(target_env = "gnu")]
pub(crate) const KSU_IOCTL_NUKE_EXT4_SYSFS: u64 = libc::_IOW::<()>(K, 17);
#[cfg(not(target_env = "gnu"))]
pub(crate) const KSU_IOCTL_NUKE_EXT4_SYSFS: i32 = libc::_IOW::<()>(K, 17);

#[cfg(target_env = "gnu")]
pub(crate) const KSU_IOCTL_ADD_TRY_UMOUNT: u64 = libc::_IOW::<()>(K, 18);
#[cfg(not(target_env = "gnu"))]
pub(crate) const KSU_IOCTL_ADD_TRY_UMOUNT: i32 = libc::_IOW::<()>(K, 18);
