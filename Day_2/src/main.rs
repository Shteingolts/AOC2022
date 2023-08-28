fn parse_input(path: &str) -> String {
    let input: String = std::fs::read_to_string(path).expect("Unable to read file");
    input
}

fn solve_round_part_1(my_pick: &str, opponent_pick: &str) -> u32 {
    // A, X -> Rock (+1 point)
    // B, Y -> Paper (+2 points)
    // C, Z -> Scissors (+3 points)
    // 0 for a loss, 3 for a draw, 6 for a win

    let result = match opponent_pick {
        // Rock
        "A" => match my_pick {
            "X" => 4, //Rock -> draw
            "Y" => 8, //Paper -> win
            "Z" => 3, //Scissors -> loss
            _ => 0,
        },
        // Paper
        "B" => match my_pick {
            "X" => 1, //loss
            "Y" => 5, //draw
            "Z" => 9, //win
            _ => 0,
        },
        // Scissors
        "C" => match my_pick {
            "X" => 7, //win
            "Y" => 2, //loss
            "Z" => 6, //draw
            _ => 0,
        },
        _ => 0,
    };
    assert_ne!(result, 0);
    result
}

fn solve_part_1(input: &str) -> u32 {
    let rounds_played: Vec<&str> = input
        .lines()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut answer: u32 = 0;

    for round in rounds_played.iter() {
        let picks: Vec<&str> = round
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        answer += solve_round_part_1(picks[1], picks[0]);
    }
    answer
}

fn solve_round_part_2(my_pick: &str, opponent_pick: &str) -> u32 {
    // A, X -> Rock (+1 point)
    // B, Y -> Paper (+2 points)
    // C, Z -> Scissors (+3 points)
    // 0 for a loss, 3 for a draw, 6 for a win

    let result = match opponent_pick {
        // Rock
        "A" => match my_pick {
            "X" => 3, //Loss -> Scissors
            "Y" => 4, //Draw -> Rock
            "Z" => 8, //Win -> Paper
            _ => 0,
        },
        // Paper
        "B" => match my_pick {
            "X" => 1, //Loss -> Rock
            "Y" => 5, //Draw -> Paper
            "Z" => 9, //Win -> Scissors
            _ => 0,
        },
        // Scissors
        "C" => match my_pick {
            "X" => 2, //Loss -> Paper
            "Y" => 6, //Draw -> Scissors
            "Z" => 7, //Win -> Rock
            _ => 0,
        },
        _ => 0,
    };
    assert_ne!(result, 0);
    result
}

fn solve_part_2(input: &str) -> u32 {
    let rounds_played: Vec<&str> = input
        .lines()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut answer: u32 = 0;

    for round in rounds_played.iter() {
        let picks: Vec<&str> = round
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        answer += solve_round_part_2(picks[1], picks[0]);
    }
    answer
}

fn main() {
    let input = parse_input("data/input.txt");
    let answer = solve_part_1(&input);
    println!("Part 1: {answer}");
    let answer = solve_part_2(&input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part_1() {
        let input: &str = "A Y
        B X
        C Z";
        let answer = solve_part_1(&input);
        assert_eq!(answer, 15);
    }
    #[test]
    fn it_works_part_2() {
        let input = "A Y
        B X
        C Z";
        let answer = solve_part_2(&input);
        assert_eq!(answer, 12);
    }
}
