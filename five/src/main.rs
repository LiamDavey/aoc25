use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/actual.txt").expect("Path should be a readable UTF-8 file");
    let (fresh, avail) = contents.split_once("\n\n").expect("Expected blank line");
    let fresh_ranges: Vec<(usize, usize)> = fresh
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').expect("Expected ranges; e.g. '1-2'");
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let mut p1_ans = 0;
    for item in avail.lines() {
        let item: usize = item.parse().unwrap();
        let is_fresh = fresh_ranges.iter().any(|i| i.0 <= item && i.1 >= item);
        if is_fresh {
            p1_ans += 1;
        }
    }
    println!("p1 answer is {p1_ans}");
}
