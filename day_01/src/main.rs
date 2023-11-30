fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    *parse_input(input).last().unwrap_or(&0u32)
}

fn part2(input: &str) -> u32 {
    parse_input(input).iter().rev().take(3).sum()
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut vec_of_sums = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.parse::<u32>()
                        .expect("should only contain positiv numbers!")
                })
                .sum()
        })
        .collect::<Vec<u32>>();
    vec_of_sums.sort_unstable();
    vec_of_sums
}

#[test]
fn case1() {
    let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    assert_eq!(part1(input), 24000);
}

#[test]
fn case2() {
    let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    assert_eq!(part2(input), 45000);
}
