mod control;

use std::io;

use control::{play_cdrom_msf, stop_cdrom};
use windows::{
    w,
    Win32::{
        Foundation::*,
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
    },
};

use crate::control::{
    eject_cdrom, load_cdrom, pause_cdrom, read_toc, resume_cdrom, seek_cdrom_msf,
};

fn main() {
    println!("Please input command");

    let mut buffer = String::new(); // 入力用のバッファ
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let command = buffer.trim();

    unsafe {
        let result = CreateFileW(
            w!("\\\\.\\F:"),
            GENERIC_READ.0 | GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_READONLY,
            None,
        );

        let handle = result.unwrap();

        match command {
            "play" => {
                println!("Start playing...");
                play_cdrom_msf(handle);
            }
            "stop" => {
                println!("Stop");
                stop_cdrom(handle);
            }
            "toc" => {
                println!("Read TOC");
                read_toc(handle);
            }
            "seek" => {
                println!("Seek to top");
                seek_cdrom_msf(handle);
            }
            "pause" => {
                println!("Pause");
                pause_cdrom(handle);
            }
            "eject" => {
                println!("Eject disc");
                eject_cdrom(handle);
            }
            "load" => {
                println!("Load disc");
                load_cdrom(handle);
            }
            "resume" => {
                println!("Resume");
                resume_cdrom(handle);
            }
            _ => {
                println!("Unknown command.");
            }
        }

        CloseHandle(handle);
    }
}
