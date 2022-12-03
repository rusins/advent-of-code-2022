use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines().map(|e| e.unwrap());

    let mut score = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let opponent = match parts[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!(),
        };
        let you = match parts[1] {
            "X" => (opponent + 1) % 3 + 1,
            "Y" => opponent,
            "Z" => opponent % 3 + 1,
            _ => panic!(),
        };
        score += you;
        let adj = (opponent - you + 2) % 3;
        match adj {
            1 => score += 6,
            2 => score += 3,
            _ => (),
        }
    }

    println!("{}", score)
}