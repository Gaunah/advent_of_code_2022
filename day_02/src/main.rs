fn main() {
    let input = include_str!("../input.txt");
    println!("Answer to part1: {}", compute_final_score(input));
}

fn compute_final_score(input: &str) -> u32 {
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

    input.lines().fold(0, |mut total, line| {
        total += match line {
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
        };
        total
    })
}

#[test]
fn case1() {
    let input = "A Y\nB X\nC Z";
    assert_eq!(compute_final_score(input), 15);
}
