use std::fs;



pub fn get_rounds() -> Vec<String> {
    let contents: String  = fs::read_to_string("input2").expect("File not found");
    contents.lines().map(|str| str.to_string()).collect()
}

pub fn run() {
    let mut score: i32 = 0;
    for round in get_rounds() {
        let first: char = round.chars().next().unwrap();
        let last: char = round.chars().last().unwrap();

        let both = (first, last);

        //just hard coding everything

        //adding the score for choice
        score = score + match last {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0
        };

        //score for winning, tie, and loss
        score = score + match both  {
            ('A','X') => 3,
            ('B','Y') => 3,
            ('C','Z') => 3,
            ('A','Y') => 6,
            ('B','Z') => 6,
            ('C','X') => 6,
            (_, _) => 0
        };


    }

    println!("Day two scores: {}", score);

    run2()
}

pub fn run2() {
    let mut score: i32 = 0;
    for round in get_rounds() {
        let first: char = round.chars().next().unwrap();
        let last: char = round.chars().last().unwrap();

        //choice scores if winning
        let win = match first {
            'A' => 2,
            'B' => 3,
            'C' => 1,
            _=> 0
        };

        //choice scores if tied
        let tie = match first {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0
        };

        //choice scores if losing
        let loss = match first {
            'A' => 3,
            'B' => 1,
            'C' => 2,
            _=> 0
        };

        //score for results
        score = score + match last {
            'Y' => 3 + tie,
            'Z' => 6 + win,
            'X' => 0 + loss,
            _ => 0
        }
    }

    println!("Day two part two scores: {}", score)
}