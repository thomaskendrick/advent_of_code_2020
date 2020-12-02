pub struct Password {
    repeat_range: (usize, usize),
    policy_char: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    fn parse_password_line(line: &str) -> Password {
        let mut line_array = line.split(' ');
        let mut repeat_range_split = line_array.next().unwrap().split('-');

        let min: usize = repeat_range_split.next().unwrap().parse().unwrap();
        let max: usize = repeat_range_split.next().unwrap().parse().unwrap();

        let policy_char: char = line_array.next().unwrap().chars().next().unwrap();
        let password = line_array.next().unwrap();

        return Password {
            repeat_range: (min, max),
            policy_char,
            password: String::from(password),
        };
    }
    return input.lines().map(parse_password_line).collect();
}
#[aoc(day2, part1)]

fn solve_part_1(input: &Vec<Password>) -> usize {
    let input = input.clone();
    let mut valid_passwords = 0;

    for pwd in input {
        let occurences = pwd.password.matches(pwd.policy_char).count();
        if occurences >= pwd.repeat_range.0 && occurences <= pwd.repeat_range.1 {
            valid_passwords += 1;
        }
    }
    return valid_passwords;
}

#[aoc(day2, part2)]

fn solve_part_2(input: &Vec<Password>) -> usize {
    let input = input.clone();
    let mut valid_passwords = 0;

    for pwd in input {
        let first_char_valid =
            pwd.policy_char == pwd.password.chars().collect::<Vec<char>>()[pwd.repeat_range.0 - 1];
        let second_char_valid =
            pwd.policy_char == pwd.password.chars().collect::<Vec<char>>()[pwd.repeat_range.1 - 1];

        if first_char_valid ^ second_char_valid {
            valid_passwords += 1;
        }
    }
    return valid_passwords;
}
