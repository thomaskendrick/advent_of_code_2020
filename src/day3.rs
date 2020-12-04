pub enum Cell {
    Tree,
    Empty,
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        match s {
            "." => Cell::Empty,
            "#" => Cell::Tree,
            _ => panic!(format!("bad character {}", s)),
        }
    }
}

pub type SkiSlope = Vec<Vec<Cell>>;

fn check_slope(input: &SkiSlope, x_delta: usize, y_delta: usize) -> usize {
    let mut tree_count = 0;
    let mut x_offset = 0;

    for r in 1..input.len() {
        if r % y_delta > 0 {
            continue;
        }
        let row = &input[r];
        x_offset += x_delta;
        x_offset %= row.len();
        if let Cell::Tree = row[x_offset] {
            tree_count += 1;
        }
    }
    tree_count
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> SkiSlope {
    input
        .lines()
        .map(|line| {
            return line
                .split("")
                .filter(|line| !line.is_empty())
                .map(|c| Cell::from(c))
                .collect();
        })
        .collect()
}

#[aoc(day3, part1)]

fn solve_part_1(input: &SkiSlope) -> usize {
    let x_delta = 3;
    let y_delta = 1;

    check_slope(input, 3, 1)
}
#[aoc(day3, part2)]

fn solve_part_2(input: &SkiSlope) -> usize {
    return check_slope(input, 1, 1)
        * check_slope(input, 3, 1)
        * check_slope(input, 5, 1)
        * check_slope(input, 7, 1)
        * check_slope(input, 1, 2);
}
