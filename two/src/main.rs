use std::env;
use std::fs;

fn p1(id_ranges: Vec<&str>) -> i64 {
    let mut ans = 0;
    for id_range in id_ranges {
        let (start, end) = id_range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for id in start..=end {
            let id_str = id.to_string();
            let id_len = id_str.chars().count();
            if id_len % 2 != 0 {
                continue;
            }
            let (left, right) = id_str.split_at(id_len / 2);
            if left == right {
                ans += id;
            }
        }
    }
    ans
}

fn find_factors(v: usize) -> Vec<usize> {
    let mut factors = vec![];
    for i in 1..v {
        if v % i == 0 {
            factors.push(i);
        }
    }
    factors
}

fn p2(id_ranges: Vec<&str>) -> i64 {
    let mut ans = 0;
    for id_range in id_ranges {
        let (start, end) = id_range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for id in start..=end {
            let id_str = id.to_string();
            let id_len = id_str.chars().count();
            let factors_len = find_factors(id_len);
            for f in factors_len {
                let mut splits = id_str.as_bytes().chunks(f);
                let first = splits.next().unwrap();
                if splits.all(|i| i == first) {
                    ans += id;
                    break;
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ans() {
        let id_ranges = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];
        assert_eq!(p1(id_ranges.clone()), 1227775554);
        assert_eq!(p2(id_ranges.clone()), 4174379265);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./two <file_path>");
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Path should be a readable UTF-8 file");
    let id_ranges: Vec<&str> = contents.split(",").collect();

    let p1_ans = p1(id_ranges.clone());
    let p2_ans = p2(id_ranges.clone());

    println!("p1 answer is {p1_ans}!");
    println!("p2 answer is {p2_ans}!");
}
