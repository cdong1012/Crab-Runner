#[cfg(windows)]
extern crate winapi;

mod help;
mod launcher;
use help::{print_banner, print_help};
use launcher::{load_file, run_shellcode};
use std::ffi::CString;

fn main() {
    let mut offset: i64 = 0;
    let mut debug: bool = false;
    let version = "1.0";
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
        } else if argv[i] == CString::new("--debug").unwrap() {
            debug = true;
        } else if argv[i] == CString::new("--version").unwrap() {
            println!("[x] The current version of Crabrunner is {}", version);
        } else {
            println!("[!!] Invalid flag - {:?}\n", argv[i]);
            print_help();
            return;
        }
        i += 1;
    }
    println!("[x] File name: {:?}", file_name);
    println!("[x] Shellcode offset: 0x{:x}", offset);
    println!("[x] Debug: {}", debug);
    unsafe {
        let (buffer, size) = load_file(file_name, offset);
        run_shellcode(buffer, size as usize, 0);
    }
}
