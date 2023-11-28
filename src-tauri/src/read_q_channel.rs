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

#[derive(serde::Serialize)]
pub struct CdDriveStatus {
    status: String,
    track_number: u8,
    index_number: u8,
    absolute_play_time: PlayTime,
    track_relative_play_time: PlayTime,
}

#[derive(serde::Serialize)]
pub struct PlayTime {
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
}

pub fn read_q_channel(handle: HANDLE) -> Result<CdDriveStatus, windows::core::Error> {
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

        let status = match output.header.audio_status {
            AUDIO_STATUS_NOT_SUPPORTED => "Not supported",
            AUDIO_STATUS_IN_PROGRESS => "Playing",
            AUDIO_STATUS_PAUSED => "Paused",
            AUDIO_STATUS_PLAY_COMPLETE => "Play complete",
            AUDIO_STATUS_PLAY_ERROR => "Play error",
            AUDIO_STATUS_NO_STATUS => "No status",
            _ => "Unknown status",
        }
        .to_string();

        Ok(CdDriveStatus {
            status: status,
            track_number: output.track_number,
            index_number: output.index_number,
            absolute_play_time: PlayTime {
                minutes: output.absolute_address[1],
                seconds: output.absolute_address[2],
                frames: output.absolute_address[3],
            },
            track_relative_play_time: PlayTime {
                minutes: output.track_relative_address[1],
                seconds: output.track_relative_address[2],
                frames: output.track_relative_address[3],
            },
        })
    }
}
