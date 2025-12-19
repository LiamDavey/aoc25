use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./six <file_path>");
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("I can't read that file");

    let mut lines: Vec<Vec<&str>> = contents.lines().map(|line| line.split_whitespace().collect()).collect();

    let mut transposed = vec![];
    while !lines[0].is_empty() {
        let mut col = vec![];
        for line in &mut lines {
            if let Some(x) = line.pop() {
                col.push(x);
            }
        }
        transposed.push(col);
    }

    let mut p1_ans = 0;
    for mut problem in transposed {
        let operator = problem.pop().unwrap();
        let ans: usize = match operator {
            "*" => problem.iter().map(|num| num.parse::<usize>().unwrap()).product(),
            "+" => problem.iter().map(|num| num.parse::<usize>().unwrap()).sum(),
            _ => unreachable!("Operator will only be * or +"),
            
        };
        p1_ans += ans;
    }
    println!("{p1_ans}");
}