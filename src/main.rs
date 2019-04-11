extern crate kmp;

/**
\brief a simple program to search exact match strings in reference text file
		strings stored in file *query*, reference referred as *ref*
		usage: {command} *ref* *query*
*/


use std::env;
use kmp::kmp::search;


fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		usage(&args);
		return;
	}

	let txt = read_file(&args[1]);
	let pat = read_file(&args[2]);

	let pat = String::from(pat);
	let queries: Vec<&str> = pat.split('\n').collect();
	let mut locs = vec![];

	for query in &queries {
		locs.push(search(query, &txt, true));
	}

	for (q, l) in queries.iter().zip(locs.iter()) {
		print!("{}: ", q);
		for loc in l {
			print!("\t{}", loc);
		}
		println!();
	}
}

fn usage(args: &Vec<String>) {
	assert!(args.len() > 0, "Cannot access command line arguments");
	eprintln!("Usage:\n\t{command} {{ref}} {{query}}", command = args[0]);
}

fn read_file(path: &str) -> String {
	use std::fs;
	fs::read_to_string(path).expect(&format!("{} {}", "Error reading file:", path))
}