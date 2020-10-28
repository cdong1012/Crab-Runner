#[cfg(windows)]
extern crate winapi;

pub fn print_banner() {
	print!("\x1B[2J\x1B[1;1H");
	println!(
		"
   ____ ____      _    ____  ____  _   _ _   _ _   _ _____ ____  
  / ___|  _ \\    / \\  | __ )|  _ \\| | | | \\ | | \\ | | ____|  _ \\ 
 | |   | |_) |  / _ \\ |  _ \\| |_) | | | |  \\| |  \\| |  _| | |_) |
 | |___|  _ <  / ___ \\| |_) |  _ <| |_| | |\\  | |\\  | |___|  _ < 
  \\____|_| \\_\\/_/   \\_\\____/|_| \\_\\\\___/|_| \\_|_| \\_|_____|_| \\_\
																 
"
	);
	println!(
		"                            _~^~^~_
			\\) /  o o  \\ (/
			  '_   u   _'
			  \\ '-----' /\n"
	);
}

pub fn print_help() {
	println!("Usage: crab_runner.exe <shellcode file>");
	println!("Optional arguments:");
	println!("	--offset <offset>\tThe shellcode offset to start at");
	println!("	--debug\t\t\tVerbose mode");
	println!("	--version\t\tPrint the version\n");
}
