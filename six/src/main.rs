use std::env;
use std::fs;

enum Operator {
    Product,
    Sum,
}

fn solve(problem: &[usize], operator: Operator) -> usize {
    match operator {
        Operator::Product => problem.iter().product(),
        Operator::Sum => problem.iter().sum(),
    }
}

fn p1(contents: &str) -> usize {
    let mut lines: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

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

    let mut ans = 0;
    for mut problem in transposed {
        let operator = match problem.pop().unwrap() {
            "*" => Operator::Product,
            "+" => Operator::Sum,
            _ => unreachable!("Operator will only be * or +"),
        };
        let problem: Vec<usize> = problem
            .iter()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        ans += solve(&problem, operator);
    }
    ans
}

fn p2(contents: &str) -> usize {
    let mut lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut ans = 0;
    while !lines[0].is_empty() {
        let mut problem: Vec<usize> = vec![];
        let mut operator: Option<Operator> = None;
        while operator.is_none() {
            let mut col = vec![];
            for line in &mut lines {
                if let Some(x) = line.pop() {
                    match x {
                        '*' => operator = Some(Operator::Product),
                        '+' => operator = Some(Operator::Sum),
                        ' ' => (),
                        x => col.push(x),
                    }
                }
            }
            if !col.is_empty() {
                problem.push(col.iter().collect::<String>().parse().unwrap());
            }
        }
        ans += solve(&problem, operator.unwrap());
    }
    ans
}

fn main() {
    let file_path = env::args().nth(1).expect("Usage: six <file_path>");

    let contents = fs::read_to_string(file_path).expect("I can't read that file");

    let p1_ans = p1(&contents);
    println!("{p1_ans}");

    let p2_ans = p2(&contents);
    println!("{p2_ans}");
}
