const INPUT: &str = include_str!("./input.txt");

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = INPUT.lines()
        .map(|line| {
            let mut values = line.split_whitespace().map(|v| v.parse::<i32>().unwrap());
            (values.next().unwrap(), values.next().unwrap())
        })
        .unzip();

    list_1.sort_unstable();
    list_2.sort_unstable();

    (list_1, list_2)
}

fn part_1() {
    let (list_1, list_2) = parse_input();
    let score: i32 = list_1.iter().zip(list_2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Part 1: {}", score);
}

fn part_2() {
    let (list_1, list_2) = parse_input();
    let score: i32 = list_1.iter()
        .map(|n| n * list_2.iter().filter(|&&x| x == *n).count() as i32)
        .sum();

    println!("Part 2: {}", score);
}

fn main() {
    part_1();
    part_2();
}
