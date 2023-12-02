fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            left.chars()
                .find_map(|c| {
                    if right.contains(c) {
                        if c.is_lowercase() {
                            Some(c as u32 - 96) // a-z -> 1-26
                        } else {
                            Some(c as u32 - 38) // A-Z -> 27-52
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or(0)
        })
        .sum()
}

#[test]
fn case1() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part1(input), 157)
}
