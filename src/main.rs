use std::collections::HashSet;
use clipboard::{ ClipboardProvider, ClipboardContext };

mod golf;

fn main() {
	let mut clipboard: ClipboardContext = match ClipboardProvider::new() {
		Ok(clipboard) => clipboard,
		Err(err) => {
			println!("Cannot access the clipboard.");
			println!("Error message: {:?}", err);
			println!("If this happens, it may mean your os isn't supported");
			return;
		},
	};
	let c_code = match clipboard.get_contents() {
		Ok(code) => code,
		Err(err) => {
			println!("Cannot read the contents of the clipboard.");
			println!("Error message: {:?}", err);
			println!("If this happens, it may mean your os isn't supported");
			return;
		},
	};

	let mut blacklist = HashSet::new();
	let mut path = match std::env::current_exe() {
		Ok(path) => path,
		Err(err) => {
			println!("Couldn't find path to the executable");
			println!("Error message: {:?}", err);
			return;
		},
	};
	path.pop();
	path.push("blacklist.txt");
	for line in 
		std::fs::read_to_string(&path)
			.expect("Cannot open 'blacklist.txt', this file is required for the tool to work")
			.lines() 
			.map(|v| v.trim())
			.filter(|v| v.len() > 0)
			.filter(|v| !v.starts_with("//"))
	{
		blacklist.insert(line.trim().to_string());
	}

	println!("Input(recieved from clipboard): ");
	for (i, line) in c_code.lines().enumerate() {
		println!("{:>3} | {}", i, line);
	}
	println!();

	println!("Golfed output: ");
	let golfed_code = golf::golfify_code(&c_code, &blacklist);

	for (i, line) in golfed_code.lines().enumerate() {
		println!("{:>3} | {}", i, line);
	}

	println!();

	println!("Bytes in golfed code: {}", golfed_code.len());

	match clipboard.set_contents(golfed_code) {
		Ok(_) => {
			println!("Copied golfed code to clipboard!");
		},
		Err(err) => {
			println!("Couldn't copy golfed code to clipboard");
			println!("Error: {:?}", err);
		}
	}
}
