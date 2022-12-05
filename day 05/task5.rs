use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic
    // Read initial state
    let layout_end = {
        let mut current_line_no = 0;
        for (line_no, line) in lines.iter().enumerate() {
            current_line_no = line_no;
            if line.is_empty() {
                break;
            }
        }
        current_line_no
    };
    let stack_count: i64 = lines[layout_end - 1].split(" ").collect::<Vec<&str>>().last().unwrap().parse().unwrap();
    let mut stacks: Vec<Vec<char>> = (0..stack_count).map(|_| Vec::new()).collect();
    for line_no in (0 .. layout_end - 1).rev() {
        let mut line_it = lines[line_no].chars().enumerate();
        loop {
            match line_it.next() {
                Some((pos, c)) if c == '[' => {
                    let (_, crate_value) = line_it.next().unwrap();
                    stacks[pos / 4].push(crate_value);
                },
                Some(_) => (),
                None => break,
            }
        }
    }

    // Read and execute instructions
    for line_no in layout_end + 1 .. lines.len() {
        let line = &lines[line_no];
        let split: Vec<&str> = line.split(" ").collect();
        let count: u32 = split[1].parse().unwrap();
        let from: usize = split[3].parse().unwrap();
        let to: usize = split[5].parse().unwrap();

        for _ in 0..count {
            let crate_value = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(crate_value);
        }
    }

    for stack in stacks.iter() {
        match stack.last() {
            Some(value) => print!("{}", value),
            None => (),
        }
    }
    println!();
}