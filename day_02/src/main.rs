use std::fs;

fn main() {
    let contents = match fs::read_to_string("./src/input.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    let score = second_half(contents.split('\n').collect::<Vec<&str>>());

    println!("contents: {:?}", score);
}

pub fn first_half(input: Vec<&str>) -> i32 {
    let mut score = 0;

    input.into_iter().for_each(|play| {
        let plays = play.split(' ').collect::<Vec<&str>>();
        let opponent = plays[0];
        let me = plays[1];
        if opponent == "A" && me == "Y" {
            score += 8;
        } else if opponent == "A" && me == "Z" {
            score += 3;
        } else if opponent == "A" && me == "X" {
            score += 4;
        } else if opponent == "B" && me == "X" {
            score += 1;
        } else if opponent == "B" && me == "Y" {
            score += 5;
        } else if opponent == "B" && me == "Z" {
            score += 9;
        } else if opponent == "C" && me == "X" {
            score += 7;
        } else if opponent == "C" && me == "Y" {
            score += 2;
        } else if opponent == "C" && me == "Z" {
            score += 6;
        }
    });
    return score;
}

fn second_half(input: Vec<&str>) -> i32 {
    let mut score = 0;

    input.into_iter().for_each(|play| {
        let plays = play.split(' ').collect::<Vec<&str>>();
        let opponent = plays[0];
        let me = plays[1];
        if opponent == "A" && me == "Y" {
            score += 4;
        } else if opponent == "A" && me == "Z" {
            score += 8;
        } else if opponent == "A" && me == "X" {
            score += 3;
        } else if opponent == "B" && me == "X" {
            score += 1;
        } else if opponent == "B" && me == "Y" {
            score += 5;
        } else if opponent == "B" && me == "Z" {
            score += 9;
        } else if opponent == "C" && me == "X" {
            score += 2;
        } else if opponent == "C" && me == "Y" {
            score += 6;
        } else if opponent == "C" && me == "Z" {
            score += 7;
        }
    });
    return score;
}
