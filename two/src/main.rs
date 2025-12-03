use std::env;
use std::fs;

fn p1(id_ranges: Vec<&str>) -> i64 {
    let mut ans = 0;
    for id_range in id_ranges {
        let ids: Vec<&str> = id_range.split("-").collect();
        let start: i64 = ids[0].parse().unwrap();
        let end: i64 = ids[1].parse().unwrap();
        for id in start..=end {
            let id_str: String = id.to_string();
            let id_len: usize = id_str.chars().count();
            if id_len % 2 != 0 {
                continue;
            }
            let (left, right) = id_str.split_at(id_len/2);
            if left == right {
                ans += id;
            }
        }
    }
    ans
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./two <file_path>");
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Path should be a readable UTF-8 file");
    let id_ranges: Vec<&str> = contents.split(",").collect();

    let p1 = p1(id_ranges);

    println!("p1 password is {p1}!");
}
