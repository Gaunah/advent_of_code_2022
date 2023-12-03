fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            left.chars()
                .find_map(|c| {
                    if right.contains(c) {
                        Some(calculate_item_prio(c))
                    } else {
                        None
                    }
                })
                .unwrap_or(0)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let [first, second, third] = chunk else {
                panic!("The chunk has less then three elements!");
            };

            first
                .chars()
                .find_map(|c| {
                    if second.contains(c) && third.contains(c) {
                        Some(calculate_item_prio(c))
                    } else {
                        None
                    }
                })
                .unwrap_or(0)
        })
        .sum()
}

fn calculate_item_prio(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96 // a-z -> 1-26
    } else {
        c as u32 - 38 // A-Z -> 27-52
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}
