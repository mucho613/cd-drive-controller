use std::{
    ffi::c_void,
    mem::{self, transmute},
};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::{
    Ioctl::{IOCTL_DISK_EJECT_MEDIA, IOCTL_DISK_LOAD_MEDIA},
    IO::DeviceIoControl,
};

#[repr(C)]
struct CdromPlayAudioMsf {
    start_minutes: u8,
    start_seconds: u8,
    start_frames: u8,
    end_minutes: u8,
    end_seconds: u8,
    end_frames: u8,
}

#[repr(C)]
struct CdromSeekAudioMsf {
    m: u8,
    s: u8,
    f: u8,
}
pub fn play_cdrom_msf(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0006) << 2) | (0);

    let input: CdromPlayAudioMsf = CdromPlayAudioMsf {
        start_minutes: 0,
        start_seconds: 0,
        start_frames: 0,
        end_minutes: 80,
        end_seconds: 0,
        end_frames: 0,
    };

    let ret;

    unsafe {
        let input = transmute::<&CdromPlayAudioMsf, *const c_void>(&input);

        ret = DeviceIoControl(
            handle,
            command,
            Some(input),
            mem::size_of::<CdromPlayAudioMsf>() as u32,
            None,
            0,
            None,
            None,
        );
    }

    ret
}

pub fn seek_cdrom_msf(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0001) << 2) | (0);

    let ret;

    unsafe {
        let input = CdromSeekAudioMsf { m: 9, s: 27, f: 0 };

        let input = transmute::<&CdromSeekAudioMsf, *const c_void>(&input);

        ret = DeviceIoControl(
            handle,
            command,
            Some(input),
            std::mem::size_of::<CdromPlayAudioMsf>() as u32,
            None,
            0,
            None,
            None,
        );
    }

    ret
}

pub fn eject_cdrom(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = IOCTL_DISK_EJECT_MEDIA;

    let ret;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    ret
}

pub fn load_cdrom(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = IOCTL_DISK_LOAD_MEDIA;

    let ret;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    ret
}

pub fn stop_cdrom(handle: HANDLE) -> Result<(), windows_core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0002) << 2) | (0);

    let ret;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    ret
}

pub fn pause_cdrom(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0003) << 2) | (0);

    let ret;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    ret
}

pub fn resume_cdrom(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0004) << 2) | (0);

    let ret;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    ret
}
