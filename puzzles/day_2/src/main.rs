const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| {
            line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect()
        })
        .collect()
}

fn is_safe(item: &[i32]) -> bool {
    let direction = (item[0] - item[1]) > 0;

    item.windows(2).all(|pair| {
        let diff = pair[0] - pair[1];
        (diff > 0) == direction && diff.abs() <= 3 && diff != 0
    })
}

fn count_safe_items(list: &[Vec<i32>]) -> usize {
    list.iter().filter(|item| is_safe(item)).count()
}

fn count_safe_items_with_removal(list: &[Vec<i32>]) -> usize {
    list.iter().filter(|item| {
        if is_safe(item) {
            true
        } else {
            (0..item.len()).any(|i| {
                let mut new_item = item.to_vec();
                new_item.remove(i);
                is_safe(&new_item)
            })
        }
    }).count()
}

fn part_1(input: &str) -> usize {
    let list = parse_input(input);
    count_safe_items(&list)
}

fn part_2(input: &str) -> usize {
    let list = parse_input(input);
    count_safe_items_with_removal(&list)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
                              1 2 7 8 9
                              9 7 6 2 1
                              1 3 2 4 5
                              8 6 4 4 1
                              1 3 6 7 9";


    #[test]
    fn test_part_1() {
        let part_1 = part_1(TEST_INPUT);
        assert_eq!(part_1, 2);
    }

    #[test]
    fn test_part_2() {
        let part_2 = part_2(TEST_INPUT);
        assert_eq!(part_2, 4);
    }
}
