#[cfg(windows)]
extern crate winapi;

mod help;
mod launcher;
use help::{print_banner, print_help};
use launcher::{run_shellcode, launch_thread, run_shellcode_dump};
use std::ffi::CString;

fn main() {
    let mut offset: i64 = 0;
    let version = "1.0";
    let mut dump = false;
    print_banner();
    let argv = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    if argv.len() < 2 {
        print_help();
        return;
    }
    let file_name: CString = argv[1].clone();

    let mut i = 2;
    while i < argv.len() {
        if argv[i] == CString::new("--offset").unwrap() && i + 1 < argv.len() {
            i += 1;
            let offset_string: String = argv[i].clone().into_string().unwrap();
            if offset_string.as_bytes()[0] == '0' as u8 && offset_string.as_bytes()[1] == 'x' as u8
            {
                let mut hex_string: String = String::default();
                for i in 2..offset_string.len() {
                    hex_string.push(offset_string.as_bytes()[i] as char);
                }
                offset = i64::from_str_radix(hex_string.as_str(), 16).unwrap();
            } else {
                match offset_string.parse::<i64>() {
                    Ok(off) => {
                        offset = off;
                    }
                    Err(_) => {
                        println!("[!!] Invalid offset - {:?}", offset_string);
                        println!("[!!] Please enter a valid number in decimal or hexadecimal...\n");
                        return;
                    }
                }
            }
        } else if argv[i] == CString::new("--version").unwrap() {
            println!("[x] The current version of Crabrunner is {}", version);
        } else if argv[i] == CString::new("--dump").unwrap() {
            dump = true;
        }  else if argv[i] == CString::new("--help").unwrap() {
            print_help();
            return;
        } else {
            println!("[!!] Invalid flag - {:?}\n", argv[i]);
            print_help();
            return;
        }
        i += 1;
    }
    println!("[x] File name: {:?}", file_name);
    println!("[x] Shellcode offset: 0x{:x}", offset);
    let mut successful = false;
    unsafe {
        let (buffer, size) = match dump {
            true => {
                run_shellcode_dump(file_name, offset as u64)
            },
            false => {
                run_shellcode(file_name, offset)
            }
        };

        if buffer != std::ptr::null_mut() && size != 0 {
            successful = launch_thread(buffer, size as usize, offset as u64) == 0;
        }
    }
    if successful {
        println!("[x] Finish launching shellcode... Have fun!");
    } else {
        println!("[!!] Could not run this shellcode...");
    }
    
}
