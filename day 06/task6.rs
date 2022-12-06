use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

// Number of characters to look back when checking
// For part 1 it's 4, for part 2 it's 14
const REP: usize = 14;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let mut file = File::open(file_name).unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();

    // Logic
    let chars: Vec<char> = line.chars().collect();
    let mut pos = REP - 1;
    loop {
        if pos >= chars.len() {
            break;
        }
        let mut set = HashSet::new();
        let mut duplicate = false;
        for i in pos - (REP - 1) ..= pos {
            if set.contains(&chars[i]) {
                duplicate = true;
            } else {
                set.insert(&chars[i]);
            }
        }
        if duplicate {
            pos = pos + 1;
        } else {
            break;
        }
    }
    println!("{}", pos + 1);
}