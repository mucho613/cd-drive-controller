// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cd_drive;
mod control;
mod read_q_channel;
mod toc;

use cd_drive::get_file_handle;
use read_q_channel::{CdDriveStatus, PlayTime};
use toc::Toc;
use windows::Win32::Foundation::CloseHandle;

use control::{play_cdrom_msf, stop_cdrom};

use crate::{
    control::{eject_cdrom, load_cdrom, pause_cdrom, resume_cdrom, seek_cdrom_msf},
    read_q_channel::read_q_channel,
    toc::read_toc,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn command(command: &str) -> () {
    exec_ioctl(command);
}

#[tauri::command]
fn get_toc() -> Toc {
    let handle = get_file_handle();
    let toc = read_toc(handle).unwrap();
    unsafe {
        let _ = CloseHandle(handle);
    }
    toc
}

#[tauri::command]
fn get_play_time() -> CdDriveStatus {
    let handle = get_file_handle();
    let cd_drive_status = read_q_channel(handle).unwrap();
    unsafe {
        let _ = CloseHandle(handle);
    }
    cd_drive_status
}

#[tauri::command]
fn play(play_time: PlayTime) -> () {
    let handle = get_file_handle();
    play_cdrom_msf(handle, play_time).unwrap();
    unsafe {
        let _ = CloseHandle(handle);
    }
}

fn exec_ioctl(command: &str) {
    let handle = get_file_handle();

    match command {
        "stop" => {
            println!("Stop");
            stop_cdrom(handle).unwrap();
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

    unsafe {
        let _ = CloseHandle(handle);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            play,
            command,
            get_play_time,
            get_toc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
