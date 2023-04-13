mod control;

use control::play_cdrom_msf;
use windows::{
    w,
    Win32::{
        Foundation::*,
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
    },
};

fn main() {
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

        play_cdrom_msf(handle);

        CloseHandle(handle);
    }
}
