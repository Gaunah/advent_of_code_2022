fn main() {
    let input = include_str!("../input.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    // A|B|C = Opponent's choice
    // X|Y|Z = My choice
    //
    // A|X = Rock
    // B|Y = Paper
    // C|Z = Scissors
    //
    // Rock => 1 Point
    // Paper => 2 Points
    // Scissors => 3 Points
    //
    // Lost => 0 Points
    // Draw => 3 Points
    // Win => 6 Points

    input.lines().fold(0, |total, line| {
        total
            + match line {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,

                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,

                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => 0,
            }
    })
}

fn part2(input: &str) -> u32 {
    // A|B|C = Opponent's choice
    // X|Y|Z = Outcome
    //
    // A = Rock
    // B = Paper
    // C = Scissors
    //
    // X = lose
    // Y = draw
    // Z = win
    //
    // Rock => 1 Point
    // Paper => 2 Points
    // Scissors => 3 Points
    //
    // Lose => 0 Points
    // Draw => 3 Points
    // Win => 6 Points

    input.lines().fold(0, |total, line| {
        total
            + match line {
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,

                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,

                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => 0,
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 15);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 12);
    }
}
