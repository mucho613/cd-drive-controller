mod control;
mod read_q_channel;
mod toc;

use std::{io, time};

use control::{play_cdrom_msf, stop_cdrom};
use windows::{
    Win32::{
        Foundation::*,
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
    }, core::w,
};

use crate::{control::{
    eject_cdrom, load_cdrom, pause_cdrom, resume_cdrom, seek_cdrom_msf,
}, toc::read_toc, read_q_channel::read_q_channel};

fn main() {
    println!("Please input command");

    let mut buffer = String::new(); // 入力用のバッファ
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let command = buffer.trim();

    unsafe {
        let result = CreateFileW(
            w!("\\\\.\\D:"),
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
                play_cdrom_msf(handle).unwrap();
            }
            "stop" => {
                println!("Stop");
                stop_cdrom(handle).unwrap();
            }
            "toc" => {
                println!("Read TOC");
                read_toc(handle).unwrap();
            }
            "seek" => {
                println!("Seek to top");
                seek_cdrom_msf(handle).unwrap();
            }
            "pause" => {
                println!("Pause");
                pause_cdrom(handle).unwrap();
            }
            "eject" => {
                println!("Eject disc");
                eject_cdrom(handle).unwrap();
            }
            "load" => {
                println!("Load disc");
                load_cdrom(handle).unwrap();
            }
            "resume" => {
                println!("Resume");
                resume_cdrom(handle).unwrap();
            }
            "read" => {
                println!("Read Q Channel");

                let one_second = time::Duration::from_millis(1000);
                loop {
                    read_q_channel(handle).unwrap();
                    std::thread::sleep(one_second);
                }
            }
            _ => {
                println!("Unknown command.");
            }
        }

        CloseHandle(handle).unwrap();
    }
}
