use std::{
    ffi::c_void,
    mem::{self, transmute},
};
use windows::Win32::{Foundation::HANDLE, System::IO::DeviceIoControl};

use crate::read_q_channel::PlayTime;

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
pub struct CdromTOC {
    length: [u8; 2], // 2 bytes
    first_track: u8,
    last_track: u8,
    track_data: [TrackData; MAXIMUM_NUMBER_TRACKS],
}

#[derive(serde::Serialize)]
pub struct Toc {
    first_track_number: u8,
    last_track_number: u8,
    track_data: Vec<PlayTime>,
}

pub fn read_toc(handle: HANDLE) -> Result<Toc, windows::core::Error> {
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
            None,
        );

        let cdrom_toc = transmute::<*mut c_void, &CdromTOC>(output_ref);

        let tracks: Vec<PlayTime> = cdrom_toc
            .track_data
            .iter()
            .map(|track| PlayTime {
                minutes: track.address[1],
                seconds: track.address[2],
                frames: track.address[3],
            })
            .collect();

        Ok(Toc {
            first_track_number: cdrom_toc.first_track,
            last_track_number: cdrom_toc.last_track,
            track_data: tracks,
        })
    }
}
