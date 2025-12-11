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

fn p2(input: &[u8]) -> u128 {
    let mut ans = 0;
    for bank in input.split(|&b| b == b'\n') {
        let mut largest_dozen: Vec<Battery> = vec![];
        let mut next_option = 0;
        while largest_dozen.len() < 12 {
            let last_option = bank.len() - 12 + largest_dozen.len();
            let options = &bank[next_option..=last_option];
            let (pos, jolt) = options.iter().enumerate().rev().max_by_key(|x| x.1).unwrap();
            largest_dozen.push(Battery { pos: pos+next_option, joltage: *jolt });
            next_option += pos+1;
        }
        let mut bank_volts = 0;
        for battery in largest_dozen {
            bank_volts = bank_volts * 10 + u128::from(battery.joltage - b'0');
        }
        ans += bank_volts;
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

    let p1_ans = p1(&contents);
    let p2_ans = p2(&contents); 

    println!("p1 answer is {p1_ans}!");
    println!("p2 answer is {p2_ans}!");
}
