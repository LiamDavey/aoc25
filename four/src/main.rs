use std::env;
use std::fs;

#[derive(PartialEq)]
enum Cell {
    Empty,
    Roll,
}

// Grid that can be indexed by row, column
// [0][0], [0][1], [0][2]
// [1][0], [1][1], [1][2]
// [2][0], [2][1], [2][2]
struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn rows(&self) -> usize {
        self.grid.len()
    }

    fn cols(&self) -> usize {
        self.grid[0].len()
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // Up
    (1, 0),   // Down
    (0, -1),  // Left
    (0, 1),   // Right
    (-1, -1), // Up-Left
    (-1, 1),  // Up-Right
    (1, -1),  // Down-Left
    (1, 1),   // Down-Right
];

fn load_grid(file_path: &str) -> Grid {
    let contents = fs::read_to_string(file_path).expect("Path should be a readable UTF-8 file");
    let grid: Vec<Vec<Cell>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => unreachable!("Grid will only contain . or @"),
                })
                .collect()
        })
        .collect();

    Grid { grid }
}

fn check_direction(grid: &Grid, roll: (usize, usize), direction: (isize, isize)) -> bool {
    let (row, col) = roll;
    let new_row = row as isize + direction.0;
    let new_col = col as isize + direction.1;
    if new_row < 0
        || new_col < 0
        || new_row >= grid.rows() as isize
        || new_col >= grid.cols() as isize
    {
        return false;
    }

    let new_row = new_row as usize;
    let new_col = new_col as usize;

    match grid.grid[new_row][new_col] {
        Cell::Empty => false,
        Cell::Roll => true,
    }
}

fn find_rolls(grid: &Grid) -> Option<Vec<(usize, usize)>> {
    let mut row = 0;
    let mut column;
    let mut starts: Vec<(usize, usize)> = Vec::new();

    while row < grid.rows() {
        column = 0;
        while column < grid.cols() {
            if grid.grid[row][column] == Cell::Roll {
                starts.push((row, column));
            }
            column += 1;
        }
        row += 1;
    }
    if starts.is_empty() {
        None
    } else {
        Some(starts)
    }
}

fn find_removable(grid: &Grid, rolls: Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    let mut rolls_removed = vec![];
    for roll in rolls {
        let mut adjacent_rolls = 0;
        for dir in DIRECTIONS {
            if check_direction(grid, roll, dir) {
                adjacent_rolls += 1;
            }
        }
        if adjacent_rolls < 4 {
            rolls_removed.push(roll);
        }
    }
    if rolls_removed.is_empty() {
        None
    } else {
        Some(rolls_removed)
    }
}

fn remove_rolls(grid: &mut Grid, rolls: Vec<(usize, usize)>) {
    for roll in rolls {
        let (row, col) = roll;
        grid.grid[row][col] = Cell::Empty;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./four <file_path>");
    }

    let file_path = &args[1];

    let mut grid = load_grid(file_path);
    let mut first_removal = true;
    let mut p2_ans = 0;
    while let Some(rolls) = find_rolls(&grid) {
        if let Some(removeable_rolls) = find_removable(&grid, rolls) {
            let num_removeable_rolls = removeable_rolls.len();
            if first_removal {
                println!("p1 answer is {num_removeable_rolls}!");
                first_removal = false;
            }
            remove_rolls(&mut grid, removeable_rolls);
            p2_ans += num_removeable_rolls;
        } else {
            break;
        }
    }
    println!("p2 answer is {p2_ans}!");
}
