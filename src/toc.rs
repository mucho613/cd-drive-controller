use std::{
    ffi::c_void,
    mem::{transmute, self},
};
use windows::Win32::{
    Foundation::*,
    System::IO::DeviceIoControl,
};

const MAXIMUM_NUMBER_TRACKS: usize = 100;

#[repr(C)]
#[derive(Debug)]
struct TrackData {
    reserved: u8,
    control_and_adr: u8, // control: 4bit, adr: 4bit
    track_number: u8,
    reserved1: u8,
    address: [u8; 4],
}

#[repr(C)]
#[derive(Debug)]
struct CdromTOC {
    length: [u8; 2], // 2 bytes
    first_track: u8,
    last_track: u8,
    track_data: [TrackData; MAXIMUM_NUMBER_TRACKS],
}

pub fn read_toc(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x0000) << 2) | (0);

    let ret;

    unsafe {
        let mut output: CdromTOC = { mem::zeroed() };

        let output_ref = transmute::<&CdromTOC, *mut c_void>(&mut output);

        ret = DeviceIoControl(
            handle,
            command,
            None,
            mem::size_of::<CdromTOC>() as u32,
            Some(output_ref),
            mem::size_of::<CdromTOC>() as u32,
            None,
            None
        );

        let output = transmute::<*mut c_void, &CdromTOC>(output_ref);

        println!("Output: {:#?}", output.track_data[0]);
    }

    ret
}
