use std::{ffi::c_void, mem::transmute};
use windows::{
    w,
    Win32::{
        Foundation::*,
        System::{Ioctl::IOCTL_DISK_EJECT_MEDIA, IO::DeviceIoControl},
    },
};

struct CdromPlayAudioMsf {
    start_minutes: u8,
    start_seconds: u8,
    start_frames: u8,
    end_minutes: u8,
    end_seconds: u8,
    end_frames: u8,
}

pub fn play_cdrom_msf(handle: HANDLE) {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0006) << 2) | (0);

    unsafe {
        let input = CdromPlayAudioMsf {
            start_minutes: 0,
            start_seconds: 0,
            start_frames: 0,
            end_minutes: 30,
            end_seconds: 0,
            end_frames: 0,
        };

        let input = transmute::<&CdromPlayAudioMsf, *const c_void>(&input);

        DeviceIoControl(handle, command, Some(input), 6, None, 0, None, None);
    }
}

pub fn eject_cdrom(handle: HANDLE) -> BOOL {
    let command = IOCTL_DISK_EJECT_MEDIA;

    let ret;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    return ret;
}

pub fn stop_cdrom(handle: HANDLE) -> BOOL {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0002) << 2) | (0);

    let ret: BOOL;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    return ret;
}
