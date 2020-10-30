#[cfg(windows)]
extern crate winapi;
use std::ffi::CString;
use std::io::Read;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::fileapi::{CreateFileA, GetFileSize, ReadFile, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
use winapi::um::processthreadsapi::{CreateThread, ResumeThread};
use winapi::um::winbase::{CREATE_SUSPENDED, CreateFileMappingA, CopyFileA};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_NORMAL, GENERIC_READ, HANDLE, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
    PAGE_EXECUTE_READWRITE, GENERIC_ALL, FILE_SHARE_WRITE
};
use winapi::um::memoryapi::{FILE_MAP_ALL_ACCESS, FILE_MAP_EXECUTE, MapViewOfFile};

// Make sure to VirtualFree this return value
pub unsafe fn run_shellcode(file_name: CString, offset: i64) -> (LPVOID, u32) {
    let file: HANDLE = CreateFileA(
        file_name.as_ptr(),
        GENERIC_READ,
        0,
        null_mut(),
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        null_mut(),
    );
    if file == INVALID_HANDLE_VALUE {
        println!("[!!] Error trying to open shellcode file...");
        return (null_mut(), 0);
    }
    let file_size: DWORD = GetFileSize(file, null_mut());
    if offset > file_size as i64 {
        println!("[!!] Offset can not be greater than file size...");
        return (null_mut(), 0);
    }
    let buffer: LPVOID = VirtualAlloc(
        null_mut(),
        file_size as usize,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );

    if buffer == null_mut() {
        CloseHandle(file);
        println!("[!!] Error trying to allocate buffer...");
        return (null_mut(), 0);
    }

    if ReadFile(file, buffer, file_size, null_mut(), null_mut()) == 0 {
        CloseHandle(file);
        VirtualFree(buffer, file_size as usize, MEM_RELEASE);
        println!("[!!] Error trying to read data into buffer...");
        return (null_mut(), 0);
    }


    println!("[x] Allocate successfully!");
    println!("[x] Memory base: 0x{:x}", buffer as u64);
    println!("[x] Offset: 0x{:x}", offset);
    println!("[x] Entry point: 0x{:x}", buffer as i64 + offset);

    CloseHandle(file);
    return (buffer, file_size);
}

pub unsafe fn launch_thread(code_base: LPVOID, file_size: usize, offset: u64) -> u32 {
    let code_entry: unsafe extern "system" fn(lpThreadParameter: LPVOID) -> DWORD =
        std::mem::transmute((code_base as u64 + offset) as LPVOID);

    println!("[x] Starting new thread to run shellcode");
    let mut thread_id: DWORD = 0;
    let thread: HANDLE = CreateThread(
        null_mut(),
        0,
        Some(code_entry),
        null_mut(),
        CREATE_SUSPENDED,
        &mut thread_id,
    );

    if thread == null_mut() {
        VirtualFree(code_base, file_size, MEM_RELEASE);
        println!("[!!] Error while creating thread...");
        return 1;
    }

    println!("[x] Started thread {} in suspended state!", thread_id);

    println!("\n\nEnter something to begin ");
    let _input: Option<i32> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
    ResumeThread(thread);
    0
}

pub unsafe fn run_shellcode_dump(file_name: CString, offset: u64) -> (LPVOID, u32) {
    let mut dump_file_name: String = file_name.clone().into_string().unwrap();
    dump_file_name.push_str("_dump.out");
    let dump_file_name: CString = CString::new(dump_file_name).unwrap();
    CopyFileA(
        file_name.as_ptr(),
        dump_file_name.as_ptr(),
        0
    );
    let file: HANDLE = CreateFileA(
        dump_file_name.as_ptr(),
        GENERIC_ALL,
        FILE_SHARE_WRITE,
        null_mut(),
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        null_mut(),
    );

    if file == INVALID_HANDLE_VALUE {
        println!("[!!] Error trying to open shellcode file...");
        return (null_mut(), 0);
    }

    let file_mapping:HANDLE = CreateFileMappingA(
        file,
        null_mut(),
        PAGE_EXECUTE_READWRITE,
        0,
        0,
        null_mut()
    );

    let file_size: DWORD = GetFileSize(file, null_mut());
    if offset > file_size as u64 {
        println!("[!!] Offset can not be greater than file size...");
        return (null_mut(), 0);
    }

    let code_base: LPVOID = MapViewOfFile(file_mapping, FILE_MAP_ALL_ACCESS | FILE_MAP_EXECUTE, 0, 0,0);

    if code_base == null_mut() {
        CloseHandle(file_mapping);
        CloseHandle(file);
        println!("[!!] Can't map file into memory...");
        return (null_mut(), 0);
    }
    CloseHandle(file);
    return (code_base, file_size);
}