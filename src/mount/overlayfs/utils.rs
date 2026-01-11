#[cfg(any(target_os = "linux", target_os = "android"))]
use std::{fs, os::fd::AsFd, os::unix::fs::PermissionsExt, path::Path};

#[cfg(any(target_os = "linux", target_os = "android"))]
use anyhow::{Context, Result, anyhow};
#[cfg(any(target_os = "linux", target_os = "android"))]
use rustix::{
    fs::CWD,
    mount::{
        FsMountFlags, FsOpenFlags, MountAttrFlags, MountFlags, MoveMountFlags, UnmountFlags,
        fsconfig_create, fsconfig_set_string, fsmount, fsopen, mount, move_mount, unmount,
    },
};

pub struct AutoMountExt4 {
    target: String,
    auto_umount: bool,
}

impl AutoMountExt4 {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn try_new(source: &str, target: &str, auto_umount: bool) -> Result<Self> {
        let path = Path::new(source);
        if !path.exists() {
            println!("Source path does not exist");
        } else {
            let metadata = fs::metadata(path)?;
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            if permissions.readonly() {
                println!("File permissions: {:o} (octal)", mode & 0o777);
            }
        }

        mount_ext4(source, target)?;
        Ok(Self {
            target: target.to_string(),
            auto_umount,
        })
    }

    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    pub fn try_new(_src: &str, _mnt: &str, _auto_umount: bool) -> Result<Self> {
        unimplemented!()
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn umount(&self) -> Result<()> {
        unmount(self.target.as_str(), UnmountFlags::DETACH)?;
        Ok(())
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
impl Drop for AutoMountExt4 {
    fn drop(&mut self) {
        log::info!(
            "AutoMountExt4 drop: {}, auto_umount: {}",
            self.target,
            self.auto_umount
        );
        if self.auto_umount {
            let _ = self.umount();
        }
    }
}

#[allow(dead_code)]
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn mount_image(src: &str, target: &str, _autodrop: bool) -> Result<()> {
    mount_ext4(src, target)?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn mount_ext4(source: impl AsRef<Path>, target: impl AsRef<Path>) -> Result<()> {
    let new_loopback = loopdev::LoopControl::open()?.next_free()?;
    new_loopback.with().attach(source)?;
    let lo = new_loopback.path().ok_or(anyhow!("no loop"))?;
    match fsopen("ext4", FsOpenFlags::FSOPEN_CLOEXEC) {
        Result::Ok(fs) => {
            let fs = fs.as_fd();
            fsconfig_set_string(fs, "source", lo)?;
            fsconfig_create(fs)?;
            let mount = fsmount(fs, FsMountFlags::FSMOUNT_CLOEXEC, MountAttrFlags::empty())?;
            move_mount(
                mount.as_fd(),
                "",
                CWD,
                target.as_ref(),
                MoveMountFlags::MOVE_MOUNT_F_EMPTY_PATH,
            )?;
        }
        _ => {
            mount(lo, target.as_ref(), "ext4", MountFlags::empty(), None)?;
        }
    }
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn umount_dir(src: impl AsRef<Path>) -> Result<()> {
    unmount(src.as_ref(), UnmountFlags::empty())
        .with_context(|| format!("Failed to umount {}", src.as_ref().display()))?;
    Ok(())
}
