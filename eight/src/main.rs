use std::env;
use std::fs;
use std::vec;

#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn dist(&self, other: &Point) -> f64 {
        let sum_of_squares = ((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f64;
        sum_of_squares.sqrt()
    }
}

fn read_to_points(contents: &str) -> Vec<Point> {
    contents
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|n| n.parse::<isize>().unwrap());
            Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
                z: nums.next().unwrap(),
            }
        })
        .collect()
}

fn p1(points: &[Point], iterations: usize) -> usize {
    let mut options: Vec<(usize, usize, f64)> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].dist(&points[j]);
            options.push((i, j, dist));
        }
    }
    options.sort_by(|a, b| a.2.total_cmp(&b.2));
    let options: Vec<(usize, usize, f64)> = options.into_iter().take(iterations).collect();

    let mut circuits: Vec<Vec<usize>> = vec![];
    for (idx, other_idx, _dist) in options {
        let circuit_idx = circuits.iter().position(|c| c.contains(&idx));
        let other_circuit_idx = circuits.iter().position(|c| c.contains(&other_idx));
        match (circuit_idx, other_circuit_idx) {
            (None, None) => circuits.push(vec![idx, other_idx]),

            (Some(ci), None) => circuits[ci].push(other_idx),

            (None, Some(oi)) => circuits[oi].push(idx),

            (Some(ci), Some(oi)) if ci != oi => {
                // remove larger index, so keep index doesn't change
                let (keep, drop) = if ci < oi { (ci, oi) } else { (oi, ci) };
                let mut new_circuit = circuits.remove(drop);
                circuits[keep].append(&mut new_circuit);
            }

            _ => {} // already in same circuit
        }
    }

    let mut lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    lengths.sort_by(|a, b| b.cmp(a));
    lengths[..3].iter().product()
}

fn p2(points: &[Point]) -> isize {
    let mut options: Vec<(usize, usize, f64)> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].dist(&points[j]);
            options.push((i, j, dist));
        }
    }
    options.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut circuits: Vec<Vec<usize>> = vec![];
    for i in 0..points.len() {
        circuits.push(vec![i]);
    }

    let mut ans = 0;
    for (idx, other_idx, _dist) in options {
        let circuit_idx = circuits.iter().position(|c| c.contains(&idx)).unwrap();
        let other_circuit_idx = circuits.iter().position(|c| c.contains(&other_idx)).unwrap();
            if circuit_idx != other_circuit_idx {
                // remove larger index, so keep index doesn't change
                let (keep, drop) = if circuit_idx < other_circuit_idx { (circuit_idx, other_circuit_idx) } else { (other_circuit_idx, circuit_idx) };
                let mut new_circuit = circuits.remove(drop);
                circuits[keep].append(&mut new_circuit);
            }
        if circuits.len() == 1 {
            ans = points[idx].x * points[other_idx].x;
            break;
        }
    }
    ans
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Usage: eight <file_path>");

    let contents = fs::read_to_string(&file_path).expect("I can't read that file");

    let points = read_to_points(&contents);

    if file_path.contains("example.txt") {
        let p1_ans = p1(&points, 10);
        println!("{p1_ans}");
    } 
    else if file_path.contains("actual.txt") {
        let p1_ans = p1(&points, 1000);
        println!("{p1_ans}");
    }
    else {
        println!("Unknown input");
    }

    let p2_ans = p2(&points);
    println!("{p2_ans}");
}
