use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

const PART: u32 = 1;
// 20 for part 1, 10000 for part 2
const ROUNDS: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic:
    enum Operation {
        Square,
        Add { value: u64 },
        Multiply { value: u64 },
    }

    fn apply_operation(op: &Operation, old_value: u64) -> u64 {
        match op {
            Operation::Square => old_value * old_value,
            Operation::Add { value } => old_value + value,
            Operation::Multiply { value } => old_value * value,
        }
    }

    struct Monkey {
        items: Vec<u64>,
        operation: Operation,
        divisible_test_value: u64,
        test_true_monkey: usize,
        test_false_monkey: usize,
    }

    // Read input
    let mut monkeys = Vec::<Monkey>::new();
    let mut line_no: usize = 0;
    while line_no < lines.len() {
        line_no += 1;
        let starting_items: Vec<u64> = lines[line_no].trim().strip_prefix("Starting items: ").unwrap()
            .split(", ").map(|s| s.parse::<u64>().unwrap()).collect();
        line_no += 1;
        let operation_fn: Vec<&str> = lines[line_no].trim().strip_prefix("Operation: new = ").unwrap()
            .split(" ").collect();
        let operation = match operation_fn[1] {
            "+" => Operation::Add { value: operation_fn[2].parse().unwrap() },
            "*" => match operation_fn[2] {
                "old" => Operation::Square,
                value => Operation::Multiply { value: value.parse().unwrap() }
            },
            _ => panic!(),
        };
        line_no += 1;
        let divisible_test_value: u64 = lines[line_no].trim().strip_prefix("Test: divisible by ")
            .unwrap().parse().unwrap();
        line_no += 1;
        let test_true_monkey: usize = lines[line_no].trim().strip_prefix("If true: throw to monkey ")
            .unwrap().parse().unwrap();
        line_no += 1;
        let test_false_monkey: usize = lines[line_no].trim().strip_prefix("If false: throw to monkey ")
            .unwrap().parse().unwrap();
        line_no += 2;
        monkeys.push(Monkey {
            items: starting_items,
            operation,
            divisible_test_value,
            test_true_monkey,
            test_false_monkey,
        });
    }

    // Simulate the process
    let mut divisor_multiple = 1;
    for monkey in &monkeys {
        divisor_multiple *= monkey.divisible_test_value;
    }

    let mut items_inspected = vec![0; monkeys.len()];
    for _round in 0..ROUNDS {
        for m in 0..monkeys.len() {
            for item in monkeys[m].items.clone() {
                items_inspected[m] += 1;
                let worry_level: u64 = if PART == 1 {
                    apply_operation(&monkeys[m].operation, item) / 3
                } else {
                    apply_operation(&monkeys[m].operation, item) % divisor_multiple
                };
                let recipient = if worry_level % monkeys[m].divisible_test_value == 0 {
                    monkeys[m].test_true_monkey
                } else {
                    monkeys[m].test_false_monkey
                };
                monkeys[recipient].items.push(worry_level);
            }
            monkeys[m].items.clear();
        }
    }

    let monkey_business: u64 = {
        let mut s = items_inspected.clone();
        s.sort();
        s.reverse();
        s[0] * s[1]
    };
    println!("Monkey business: {}", monkey_business);
}