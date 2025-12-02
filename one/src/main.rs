use std::env;
use std::fs;

fn p1(contents: &String) -> i32 {
    let mut zeros = 0;
    let mut pos = 50;
    for line in contents.lines() {
        let steps: i32 = line[1..].trim().parse().unwrap();
        match line.chars().nth(0).unwrap() {
            'L' => pos = (pos - steps).rem_euclid(100),
            'R' => pos = (pos + steps).rem_euclid(100),
            _ => ()
        }
        if pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn p2(contents: &String) -> i32 {
    let mut zeros = 0;
    let mut pos = 50;
    for line in contents.lines() {
        println!("{line}");
        let mut crosses = 0;
        let steps: i32 = line[1..].trim().parse().unwrap();
        match line.chars().nth(0).unwrap() {
            'L' => {
                crosses = (pos - steps).div_euclid(100).abs();
                pos = (pos - steps).rem_euclid(100);
            },
            'R' => {
                crosses = (pos + steps).div_euclid(100).abs();
                pos = (pos + steps).rem_euclid(100);
            },
            _ => ()
        }
        zeros += crosses;
        println!("pos={pos}");
        println!("zeros={zeros}");
    }
    zeros
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./one <file_path>");
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Path should be a readable UTF-8 file");

    let p1 = p1(&contents);
    let p2 = p2(&contents);

    println!("p1 password is {p1}!");
    println!("p2 password is {p2}!");
}
