#[cfg(windows)]
extern crate winapi;

use std::env;
use std::ffi::CString;

use winapi::ctypes::c_void;
use winapi::um::winuser::{SystemParametersInfoA, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER, SPI_GETDESKWALLPAPER};


// TODO: refactor some of this into lib.rs


fn help(command: Option<&str>) {
	match command {
		Some(cmd) => {
			match cmd {
				"set" => {
					println!("Usage: dsktpbg set [args]\n");
					println!("Commands:");
					println!("\tset default           - sets the desktop wallpaper to the default windows wallpaper");
					println!("\tset file <path>       - sets the desktop wallpaper to the specified file");
					println!("\tset color <r> <g> <b> - sets the desktop wallpaper to the specified color");
					println!("\tset color <hexcode>   - sets the desktop wallpaper to the specified color");
				}
				"get" => {
					println!("Usage: dsktpbg get");
					println!("\tget - prints the current desktop wallpaper");
				}
				"clear" => {
					println!("Usage: dsktpbg clear");
					println!("\tclear - clears the desktop wallpaper");
				}
				"help" => {
					println!("Usage: dsktpbg help [command]");
					println!("\thelp - prints general help message");
					println!("\thelp [command] - prints help for the specified command");
				}
				_ => println!("Unknown command: '{}'", cmd),
			}
		},
		None => {
			// TODO: finalize this help text
			println!("Usage: dsktpbg [args]");
			println!("\nCommands:");
			
			println!("\thelp [command]        - displays help for the specified command");
			
			println!("\tclear                 - clears the desktop wallpaper");
			
			println!("\tget                   - gets the file path of the current desktop wallpaper");
			
			println!("\tset default           - sets the desktop wallpaper to the default windows wallpaper");
			println!("\tset file <path>       - sets the desktop wallpaper to the specified file");
			println!("\tset color <r> <g> <b> - sets the desktop wallpaper to the specified color");
			println!("\tset color <hexcode>   - sets the desktop wallpaper to the specified color");
		},
	}
}

fn set_desktop_wallpaper(path: &str) -> Result<(), String> {
	let path = CString::new(path).map_err(|e| e.to_string())?;
	
	let success = unsafe {
		SystemParametersInfoA(
			SPI_SETDESKWALLPAPER,
			0,
			path.as_ptr() as *mut c_void,
			SPIF_UPDATEINIFILE,
		)
	};
	
	if success == 0 { // TODO: is there a more idiomatic way to do this?
		let err_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
		return Err(format!("Failed to set wallpaper (winapi error {err_code})"));
	}
	
	Ok(())
}

fn get_wallpaper_path() -> Result<String, String> {
	// oh god this reminds me of C
	const BUFFER_SIZE: usize = 2048; // this is probably long enough right
	
	let path: Vec<u8> = vec![0; BUFFER_SIZE];
	
	let success = unsafe {
		SystemParametersInfoA(
			SPI_GETDESKWALLPAPER,
			BUFFER_SIZE as u32, // at least it doesnt let you buffer overflow
			path.as_ptr() as *mut c_void, // i love passing in raw buffer pointers for them to get modified in place so much! such a good and totally not antiquated design pattern!
			0,
		)
	};
	
	if success == 0 {
		let err_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
		return Err(format!("Failed to get wallpaper path (winapi error {err_code})"));
	}
	
	String::from_utf8(path).map_err(|e| e.to_string())
}


fn set_command(args: Vec<String>) -> Result<(), String> {
	if args.is_empty() {
		return Err(String::from("No arguments provided to set background to"));
	}
	
	match args[0].as_str() {
		"default" => {
			set_desktop_wallpaper("C:\\Windows\\Web\\Wallpaper\\Windows\\img0.jpg")
		}
		"file" => {
			if args.len() == 2 {
				set_desktop_wallpaper(&args[1])
			} else {
				Err(String::from("Usage: dsktpbg set file <path>"))
			}
		}
		"color" => {
			Err(String::from("setting to color not implemented yet"))
			// TODO: there is some way to do this, since you can set the wallpaper to a color in settings
			// But there would need to be a contingency for if this is not windows 8 and above (i think)
		}
		other_cmd => {
			Err(format!("Unknown set command '{other_cmd}'"))
		}
	}
}


fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 {
		println!("Expected a command - use 'dsktpbg help' for more info");
		return;
	}
	
	match args[1].as_str() {
		"help" => {
			if args.len() == 3 {
				help(Some(&args[2]));
			} else {
				help(None);
			}
		}
		"clear" => {
			set_desktop_wallpaper("").unwrap();
		}
		"get" => {
			match get_wallpaper_path() {
				Ok(path) => {
					println!("{}", path);
				},
				Err(e) => {
					println!("{}", e);
				}
			}
		}
		"set" => {
			set_command(args[2..].to_vec()).unwrap();
		}
		_ => {
			println!("Unknown command '{}'", args[1]);
		}
	}
}
