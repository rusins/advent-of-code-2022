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
    let lines = BufReader::new(&file).lines().map(|r| r.unwrap());

    // Logic
    let mut current_calories: i64 = 0;
    let mut max_calories: Vec<i64> = vec![0, 0, 0];
    for line in lines {
        if line.is_empty() {
            current_calories = 0;
        } else {
            let calories: i64 = line.parse().unwrap();
            current_calories += calories;
            if current_calories > max_calories[0] {
                max_calories[0] = current_calories;
                if max_calories[0] > max_calories[1] {
                    max_calories[0] = max_calories[1];
                    max_calories[1] = current_calories;
                    if max_calories[1] > max_calories[2] {
                        max_calories[1] = max_calories[2];
                        max_calories[2] = current_calories;
                    }
                }
            }
        }
    }

    print!("Max calories on an elf: {}\n", max_calories[2]);
    print!("Total calories of top 3 elves: {}\n", max_calories[0] + max_calories[1] + max_calories[2])
}