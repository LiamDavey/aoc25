use std::collections::HashSet;
use std::env;
use std::fs;

fn p1(contents: &str) -> usize {
    let mut ans = 0;
    let mut lines = contents.lines();
    let start_pos = lines
        .next()
        .expect("Bad input")
        .find('S')
        .expect("No start found");
    let mut beams = HashSet::new();
    beams.insert(start_pos);
    for line in lines {
        let splitters: Vec<usize> = line
            .chars()
            .enumerate()
            .filter(|(_i, c)| *c == '^')
            .map(|(i, _c)| i)
            .collect();
        let mut new_beams = HashSet::new();
        for &beam in &beams {
            if splitters.contains(&beam) {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                ans += 1;
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }
    ans
}

fn main() {
    let file_path = env::args().nth(1).expect("Usage: seven <file_path>");

    let contents = fs::read_to_string(file_path).expect("I can't read that file");

    let p1_ans = p1(&contents);
    println!("{p1_ans}");

    // let p2_ans = p2(&contents);
    // println!("{p2_ans}");
}
