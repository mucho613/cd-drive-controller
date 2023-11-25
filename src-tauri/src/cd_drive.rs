use windows::Win32::Foundation::GENERIC_READ;
use windows::Win32::Foundation::GENERIC_WRITE;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::CreateFileW;
use windows::Win32::Storage::FileSystem::FILE_ATTRIBUTE_READONLY;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;
use windows::Win32::Storage::FileSystem::FILE_SHARE_WRITE;
use windows::Win32::Storage::FileSystem::OPEN_EXISTING;
use windows_core::w;

pub fn get_file_handle() -> HANDLE {
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

        result.unwrap()
    }
}
