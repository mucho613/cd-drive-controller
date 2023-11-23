// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod control;
mod read_q_channel;
mod toc;

use windows::Win32::{
    Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE},
    Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    },
};

use control::{play_cdrom_msf, stop_cdrom};
use windows_core::w;

use crate::{
    control::{eject_cdrom, load_cdrom, pause_cdrom, resume_cdrom, seek_cdrom_msf},
    read_q_channel::read_q_channel,
    toc::read_toc,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    command(name);
    format!("{} OK.", name)
}

fn command(command: &str) {
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
                read_q_channel(handle).unwrap();
            }
            _ => {
                println!("Unknown command.");
            }
        }

        let _ = CloseHandle(handle);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
