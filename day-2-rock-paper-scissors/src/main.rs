use std::fs;

#[derive(PartialEq)]
enum RoundResult {
    WIN,
    LOSS,
    DRAW,
}

fn calculate_round_score(move_played: char, result: &RoundResult) -> usize {
    let selected_score = move_played as usize - 'X' as usize + 1;
    let outcome_score: usize = match *result {
        RoundResult::WIN => 6,
        RoundResult::DRAW => 3,
        _ => 0,
    };
    return selected_score + outcome_score;
}

fn calculate_required_move(opponents_move: char, desired_result: &RoundResult) -> char {
    let counter_move: char;
    if (*desired_result == RoundResult::WIN && opponents_move == 'C')
        || (*desired_result == RoundResult::DRAW && opponents_move == 'A')
        || (*desired_result == RoundResult::LOSS && opponents_move == 'B')
    {
        counter_move = 'X';
    } else if (*desired_result == RoundResult::WIN && opponents_move == 'A')
        || (*desired_result == RoundResult::DRAW && opponents_move == 'B')
        || (*desired_result == RoundResult::LOSS && opponents_move == 'C')
    {
        counter_move = 'Y';
    } else {
        counter_move = 'Z';
    }
    return counter_move;
}

fn main() {
    let part_1_score: usize = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| {
            let opponents_move = line.chars().nth(0).unwrap();
            let counter_move = line.chars().nth(2).unwrap();

            let result: RoundResult;
            if (opponents_move == 'A' && counter_move == 'Y')
                || (opponents_move == 'B' && counter_move == 'Z')
                || (opponents_move == 'C' && counter_move == 'X')
            {
                result = RoundResult::WIN;
            } else if (opponents_move == 'A' && counter_move == 'X')
                || (opponents_move == 'B' && counter_move == 'Y')
                || (opponents_move == 'C' && counter_move == 'Z')
            {
                result = RoundResult::DRAW;
            } else {
                result = RoundResult::LOSS;
            }

            return calculate_round_score(counter_move, &result);
        })
        .sum();

    let part_2_score: usize = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| {
            let opponents_move = line.chars().nth(0).unwrap();
            let encoded_result = line.chars().nth(2).unwrap();

            let result: RoundResult = match encoded_result {
                'X' => RoundResult::LOSS,
                'Y' => RoundResult::DRAW,
                _ => RoundResult::WIN,
            };

            let counter_move = calculate_required_move(opponents_move, &result);
            return calculate_round_score(counter_move, &result);
        })
        .sum();

    // Part 1, Answer = 12458
    println!("{}", part_1_score);

    // Part 2, Answer = 12683
    println!("{}", part_2_score);
}

// Rock     A, X
// Paper    B, Y
// Scissors C, Z
