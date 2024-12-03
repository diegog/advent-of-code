use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> i32 {
    let r = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let instructions: Vec<&str> = r.captures_iter(input)
        .map(|c| c.get(0).unwrap().as_str())
        .collect();

    instructions.iter().map(|i| {
        let mut values = i[4..i.len() - 1].split(',').map(|v| v.parse::<i32>().unwrap());
        values.next().unwrap() * values.next().unwrap()
    }).sum()
}

fn part_2(input: &str) -> i32 {
    let r = Regex::new(r"(mul\([0-9]+,[0-9]+\))|(do\(\))|(don't\(\))").unwrap();
    let mut mul_enabled = true;
    r.captures_iter(input)
        .map(|c| {
            if c.get(1).is_some() {
                let i = c.get(1).unwrap().as_str();
                let mut values = i[4..i.len() - 1].split(',').map(|v| v.parse::<i32>().unwrap());
                if mul_enabled { return values.next().unwrap() * values.next().unwrap() }
            }
            if c.get(2).is_some() {
                mul_enabled = true;
            }
            if c.get(3).is_some() {
                mul_enabled = false;
            }
            0
        }).sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_2_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_1_INPUT), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_2_INPUT), 48);
    }
}
