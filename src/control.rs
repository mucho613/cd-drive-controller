use std::{
    ffi::c_void,
    mem::{self, transmute, MaybeUninit},
};
use windows::Win32::{
    Foundation::*,
    System::{
        Ioctl::{IOCTL_DISK_EJECT_MEDIA, IOCTL_DISK_LOAD_MEDIA},
        IO::DeviceIoControl,
    },
};

// #define CTL_CODE( DeviceType, Function, Method, Access ) (
//     ((DeviceType) << 16) | ((Access) << 14) | ((Function) << 2) | (Method)
// )
//
// // Define the method codes for how buffers are passed for I/O and FS controls
//

// #define METHOD_BUFFERED                 0
// #define METHOD_IN_DIRECT                1
// #define METHOD_OUT_DIRECT               2
// #define METHOD_NEITHER                  3

// //
// // Define the access check value for any access
// //
// //
// // The FILE_READ_ACCESS and FILE_WRITE_ACCESS constants are also defined in
// // ntioapi.h as FILE_READ_DATA and FILE_WRITE_DATA. The values for these
// // constants *MUST* always be in sync.
// //

// #define FILE_ANY_ACCESS                 0
// #define FILE_READ_ACCESS          ( 0x0001 )    // file & pipe
// #define FILE_WRITE_ACCESS         ( 0x0002 )    // file & pipe

const MAXIMUM_NUMBER_TRACKS: usize = 100;

struct CdromPlayAudioMsf {
    start_minutes: u8,
    start_seconds: u8,
    start_frames: u8,
    end_minutes: u8,
    end_seconds: u8,
    end_frames: u8,
}

struct TrackData {
    reserved: u8,
    control: u8, // 4
    adr: u8,     // 4
    track_number: u8,
    reserved1: u8,
    address: u32,
}

struct CdromTOC {
    length: u16, // 2 bytes
    first_track: u8,
    last_track: u8,
    track_data: [TrackData; MAXIMUM_NUMBER_TRACKS],
}

struct CdromSeekAudioMsf {
    m: u8,
    s: u8,
    f: u8,
}

pub fn read_toc(handle: HANDLE) -> BOOL {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0000) << 2) | (0);

    let ret: BOOL;

    unsafe {
        let output: CdromTOC = MaybeUninit::uninit().assume_init();

        let output = transmute::<&CdromTOC, *mut c_void>(&output);

        ret = DeviceIoControl(handle, command, None, 0, Some(output), 0, None, None);

        let output = transmute::<*mut c_void, &CdromTOC>(output);

        println!("First track: {}", (*output).first_track);
        println!("Last track: {}", (*output).last_track);
    }

    return ret;
}

pub fn play_cdrom_msf(handle: HANDLE) {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0006) << 2) | (0);

    unsafe {
        let input = CdromPlayAudioMsf {
            start_minutes: 0,
            start_seconds: 0,
            start_frames: 0,
            end_minutes: 75,
            end_seconds: 0,
            end_frames: 0,
        };

        let input = transmute::<&CdromPlayAudioMsf, *const c_void>(&input);

        DeviceIoControl(handle, command, Some(input), 6, None, 94, None, None);
    }
}

pub fn seek_cdrom_msf(handle: HANDLE) {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0001) << 2) | (0);

    unsafe {
        let input = CdromSeekAudioMsf { m: 0, s: 0, f: 0 };

        let input = transmute::<&CdromSeekAudioMsf, *const c_void>(&input);

        DeviceIoControl(handle, command, Some(input), 3, None, 0, None, None);
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

pub fn load_cdrom(handle: HANDLE) -> BOOL {
    let command = IOCTL_DISK_LOAD_MEDIA;

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

pub fn pause_cdrom(handle: HANDLE) -> BOOL {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0003) << 2) | (0);

    let ret: BOOL;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    return ret;
}

pub fn resume_cdrom(handle: HANDLE) -> BOOL {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0004) << 2) | (0);

    let ret: BOOL;

    unsafe {
        ret = DeviceIoControl(handle, command, None, 0, None, 0, None, None);
    }

    return ret;
}
