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

    // Logic:
    let mut reg_x: Vec<i64> = vec!(1);
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        match split[0] {
            "noop" => reg_x.push(*reg_x.last().unwrap()),
            "addx" => {
                let delta: i64 = split[1].parse().unwrap();
                reg_x.push(*reg_x.last().unwrap());
                reg_x.push(reg_x.last().unwrap() + delta);
            },
            _ => ()
        }
    }

    // Part 1
    let mut signal_strength: i64 = 0;
    let mut cycle: i64 = 20;
    while cycle < reg_x.len() as i64 {
        signal_strength += cycle * reg_x[(cycle - 1) as usize];
        cycle += 40;
    }

    println!("Signal strength sum: {}", signal_strength);

    // Part 2
    if reg_x.len() >= 240 {
        for row in 0..6 {
            for col in 0..40 {
                let cycle = row * 40 + col;
                if (reg_x[cycle as usize] - col).abs() <= 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}