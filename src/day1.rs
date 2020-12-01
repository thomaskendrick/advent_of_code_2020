
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    return input.lines().map(|line| line.parse().unwrap()).collect();
}

#[aoc(day1, part1)]

fn solve_part_1(input: &Vec<usize>) -> usize {
    for x in input {
        for y in input {
            if (x + y == 2020){
                return x * y;
            }
        }
    }
    panic!("I couldn't find it!");
}

#[aoc(day1, part2)]

fn solve_part_2(input: &Vec<usize>) -> usize {
    for x in input {
        for y in input {
            for z in input {
                if (x + y + z == 2020){
                    return x * y * z;
                }
            }
        }
    }
    panic!("I couldn't find it!");
}
