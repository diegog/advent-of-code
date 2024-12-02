const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = input.lines()
        .map(|line| {
            let mut values = line.split_whitespace().map(|v| v.parse::<i32>().unwrap());
            (values.next().unwrap(), values.next().unwrap())
        })
        .unzip();

    list_1.sort_unstable();
    list_2.sort_unstable();

    (list_1, list_2)
}

fn part_1(input: &str) -> i32 {
    let (list_1, list_2) = parse_input(input);
    list_1.iter().zip(list_2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_2(input: &str) -> i32 {
    let (list_1, list_2) = parse_input(input);
    list_1.iter()
        .map(|n| n * list_2.iter().filter(|&&x| x == *n).count() as i32)
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
                              4   3
                              2   5
                              1   3
                              3   9
                              3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 31);
    }
}
