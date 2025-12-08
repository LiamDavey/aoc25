use std::env;
use std::fs;

#[derive(Debug)]
struct Battery {
    pos: usize,
    joltage: u8,
}

fn p1(input: &[u8]) -> usize {
    let mut ans = 0;
    for bank in input.split(|&b| b == b'\n') {
        let mut first_max = Battery { pos: 0, joltage: 0 };
        for (pos, jolt) in bank.iter().enumerate().take(bank.len() - 1).rev() {
            if *jolt >= first_max.joltage {
                first_max.joltage = *jolt;
                first_max.pos = pos;
            }
        }
        let second_min_pos = first_max.pos+1; // 2nd battery must be to right of 1st battery
        let mut second_max = Battery { pos: 0, joltage: 0 };
        for (pos, jolt) in bank.split_at(second_min_pos).1.iter().enumerate() {
            if *jolt >= second_max.joltage {
                second_max.joltage = *jolt;
                second_max.pos = pos+second_min_pos;
            }
        }
        let first_digit = (first_max.joltage - b'0') as usize;
        let second_digit = (second_max.joltage - b'0') as usize;
        let bank_jolts = first_digit * 10 + second_digit;
        ans += bank_jolts;
    }
    ans
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./three <file_path>");
    }

    let file_path = &args[1];
    let contents = fs::read(file_path).expect("I can't read that file");
    //let contents = fs::read_to_string(file_path).expect("Path should be a readable UTF-8 file");

    let p1_ans = p1(&contents);

    println!("p1 answer is {p1_ans}!");
}
