use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::iter::Peekable;
use std::str::Chars;

const RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic:

    enum Item {
        Int { value: i64 },
        List { items: Box<Vec<Item>> },
    }

    fn item_from_line(line: &String) -> Item {
        fn read_int(chars: &mut Peekable<Chars>) -> Item {
            let mut value: i64 = 0;
            loop {
                match chars.peek().unwrap() {
                    ',' => break,
                    ']' => break,
                    _ => {
                        let c = chars.next().unwrap();
                        value = value * RADIX as i64 + c.to_digit(RADIX).unwrap() as i64
                    }
                }
            }
            Item::Int { value }
        }
        fn read_list(chars: &mut Peekable<Chars>) -> Item {
            chars.next(); // Read '['
            let mut items = Vec::<Item>::new();
            loop {
                match chars.peek().unwrap() {
                    ',' => {
                        chars.next();
                        ()
                    }
                    ']' => {
                        chars.next();
                        return Item::List { items: Box::new(items) }
                    },
                    '[' => items.push(read_list(chars)),
                    _ => items.push(read_int(chars)),
                }
            }
        }
        fn read_item(chars: &mut Peekable<Chars>) -> Option<Item> {
            match chars.peek() {
                Some(',') => None,
                Some(']') => None,
                Some('[') => Some(read_list(chars)),
                _ => Some(read_int(chars)),
            }
        }
        let mut chars = line.chars().peekable();
        read_item(&mut chars).unwrap()
    }

    fn in_order(left: &Item, right: &Item) -> i32 {
        match (left, right) {
            (Item::Int { value: left_value }, Item::Int { value: right_value }) => {
                if left_value < right_value {
                    -1
                } else if left_value > right_value {
                    1
                } else {
                    0
                }
            }
            (Item::List { items: left_items }, Item::List { items: right_items }) => {
                let left_items = &*left_items;
                let right_items = &*right_items;
                let mut pos: usize = 0;
                let mut res: i32 = 0;
                while res == 0 {
                    if pos < left_items.len() && pos < right_items.len() {
                        res = in_order(&left_items[pos], &right_items[pos]);
                    } else if pos >= left_items.len() && pos >= right_items.len() {
                        break;
                    } else if pos >= left_items.len() {
                        res = -1;
                    } else {
                        res = 1;
                    }
                    pos += 1;
                }
                res
            }
            (Item::Int { value: left_value }, ..) => {
                let single = Box::new(vec![Item::Int { value: *left_value }]);
                in_order(&Item::List { items: single }, right)
            }
            (.., Item::Int { value: right_value }) => {
                let single = Box::new(vec![Item::Int { value: *right_value }]);
                in_order(left, &Item::List { items: single })
            }
        }
    }

    // Iterate over lines for part 1
    let mut line_no: usize = 0;
    let mut ordered_index_sum: u32 = 0;
    while line_no < lines.len() {
        let index = line_no as u32 / 3 + 1;
        let left = item_from_line(&lines[line_no]);
        let right = item_from_line(&lines[line_no + 1]);
        if in_order(&left, &right) == -1 {
            ordered_index_sum += index;
        }

        line_no += 3;
    }

    println!("Ordered index sum: {}", ordered_index_sum);

    // Iterate over lines for part 2
    // Instead of sorting all packets, we're just going to compare all packets to [[2]] and [[6]]
    // to find their positions if they were sorted.
    let item2 = item_from_line(&String::from("[[2]]"));
    let item6 = item_from_line(&String::from("[[6]]"));
    let mut item2_rank = 1;
    let mut item6_rank = 2; // because item2 is before it
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let line_item = item_from_line(&line);
        if in_order(&line_item, &item2) == -1 {
            item2_rank += 1;
        }
        if in_order(&line_item, &item6) == -1 {
            item6_rank += 1;
        }
    }
    println!("Decoder key: {}", item2_rank * item6_rank);
}