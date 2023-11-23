use std::{
    ffi::c_void,
    mem::{self, transmute},
};

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::IO::DeviceIoControl;

const IOCTL_CDROM_SUB_Q_CHANNEL: u8 = 0x00;
const IOCTL_CDROM_CURRENT_POSITION: u8 = 0x01;
const IOCTL_CDROM_MEDIA_CATALOG: u8 = 0x02;
const IOCTL_CDROM_TRACK_ISRC: u8 = 0x03;

const AUDIO_STATUS_NOT_SUPPORTED: u8 = 0x00;
const AUDIO_STATUS_IN_PROGRESS: u8 = 0x11;
const AUDIO_STATUS_PAUSED: u8 = 0x12;
const AUDIO_STATUS_PLAY_COMPLETE: u8 = 0x13;
const AUDIO_STATUS_PLAY_ERROR: u8 = 0x14;
const AUDIO_STATUS_NO_STATUS: u8 = 0x15;

#[repr(C)]
#[derive(Debug)]
struct CdromSubQDataFormat {
    format: u8,
    track: u8,
}

#[repr(C)]
#[derive(Debug)]
struct SubQHeader {
    reserved: u8,
    audio_status: u8,
    data_length: [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
struct SubQCurrentPosition {
    header: SubQHeader,
    format_code: u8,
    control_and_adr: u8, // control: 4bit, adr: 4bit
    track_number: u8,
    index_number: u8,
    absolute_address: [u8; 4],
    track_relative_address: [u8; 4],
}

pub fn read_q_channel(handle: HANDLE) -> Result<(), windows::core::Error> {
    let command = ((0x00000002) << 16) | ((0x0001) << 14) | ((0x000b) << 2) | (0);

    let ret;

    let input = CdromSubQDataFormat {
        format: IOCTL_CDROM_CURRENT_POSITION,
        track: 0,
    };

    unsafe {
        let mut output: SubQCurrentPosition = { mem::zeroed() };

        let input_ref = transmute::<&CdromSubQDataFormat, *const c_void>(&input);

        let output_ref = transmute::<&SubQCurrentPosition, *mut c_void>(&mut output);

        ret = DeviceIoControl(
            handle,
            command,
            Some(input_ref),
            std::mem::size_of::<CdromSubQDataFormat>() as u32,
            Some(output_ref),
            std::mem::size_of::<SubQCurrentPosition>() as u32,
            None,
            None,
        );

        let output = transmute::<*mut c_void, &SubQCurrentPosition>(output_ref);

        match output.header.audio_status {
            AUDIO_STATUS_NOT_SUPPORTED => println!("Not supported"),
            AUDIO_STATUS_IN_PROGRESS => println!("In progress"),
            AUDIO_STATUS_PAUSED => println!("Paused"),
            AUDIO_STATUS_PLAY_COMPLETE => println!("Play complete"),
            AUDIO_STATUS_PLAY_ERROR => println!("Play error"),
            AUDIO_STATUS_NO_STATUS => println!("No status"),
            _ => println!("Unknown status"),
        }
        println!(
            "Track: {}, Index: {}",
            output.track_number, output.index_number
        );
        println!(
            "Absolute time: {:>2}:{:02}.{:02}",
            output.absolute_address[1], output.absolute_address[2], output.absolute_address[3],
        );
        println!(
            "Track relative time: {:>2}:{:02}.{:02}",
            output.track_relative_address[1],
            output.track_relative_address[2],
            output.track_relative_address[3],
        );
    }

    ret
}
