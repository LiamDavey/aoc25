use std::fs;

fn sort_ranges(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut sorted = vec![];
    let mut ranges = ranges.clone();
    ranges.sort_by_key(|r| r.0);
    let mut last_end: usize = 0;
    for (start, end) in ranges {
        if end <= last_end {
            continue;
        }
        if start <= last_end {
            let new_start = last_end + 1;
            sorted.push((new_start, end));
        } else {
            sorted.push((start, end));
        }
        last_end = end;
    }
    sorted
}

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
    let fresh_ranges = sort_ranges(&fresh_ranges);
    let mut p1_ans = 0;
    for item in avail.lines() {
        let item: usize = item.parse().unwrap();
        let is_fresh = fresh_ranges.iter().any(|i| i.0 <= item && i.1 >= item);
        if is_fresh {
            p1_ans += 1;
        }
    }
    println!("p1 answer is {p1_ans}");
    let mut p2_ans = 0;
    for (start, end) in fresh_ranges {
        p2_ans += end - start + 1;
    }
    println!("p2 answer is {p2_ans}");
}
