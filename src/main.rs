use std::collections::HashSet;

mod golf;

fn main() {
	let c_code = r#"
	#include<stdio.h>

	void main() {
		int my_variable = 0;
		int more_variable = 1245;

		printf("%d\n", my_variable);
	}
	"#;

	let mut blacklist = HashSet::new();
	for line in 
		std::fs::read_to_string("blacklist.txt")
			.expect("Cannot open 'blacklist.txt', this file is required for the tool to work")
			.lines() 
			.map(|v| v.trim())
			.filter(|v| v.len() > 0)
			.filter(|v| !v.starts_with("//"))
	{
		blacklist.insert(line.trim().to_string());
	}

	let golfed_code = golf::golfify_code(c_code, &blacklist);
	println!("{}", golfed_code);
}
