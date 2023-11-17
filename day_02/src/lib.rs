#![feature(test)]
extern crate test;

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

    score
}

pub fn first_half_optimized(input: Vec<&str>) -> i32 {
    let mut score = 0;

    let lookup_table = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];

    input.into_iter().for_each(|play| {
        let plays = play.split(' ').collect::<Vec<&str>>();
        let opponent = plays[0].as_bytes()[0] - b'A';
        let me = plays[1].as_bytes()[0] - b'X';
        score += lookup_table[opponent as usize][me as usize];
    });

    score
}

pub fn second_half(input: Vec<&str>) -> i32 {
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

    score
}

pub fn second_half_optimized(input: Vec<&str>) -> i32 {
    let mut score = 0;

    let lookup_table = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];

    input.into_iter().for_each(|play| {
        let plays = play.split(' ').collect::<Vec<&str>>();
        let opponent = plays[0].as_bytes()[0] - b'A';
        let me = plays[1].as_bytes()[0] - b'X';
        score += lookup_table[opponent as usize][me as usize];
    });

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[bench]
    fn bench_second_half_optimized(b: &mut Bencher) {
        let contents = match std::fs::read_to_string("./src/input.txt") {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        b.iter(|| second_half_optimized(contents.split('\n').collect::<Vec<&str>>()));
    }

    #[bench]
    fn bench_second_half(b: &mut Bencher) {
        let contents = match std::fs::read_to_string("./src/input.txt") {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        b.iter(|| second_half(contents.split('\n').collect::<Vec<&str>>()));
    }

    #[bench]
    fn bench_first_half(b: &mut Bencher) {
        let contents = match std::fs::read_to_string("./src/input.txt") {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        b.iter(|| first_half(contents.split('\n').collect::<Vec<&str>>()));
    }

    #[bench]
    fn bench_first_half_optimized(b: &mut Bencher) {
        let contents = match std::fs::read_to_string("./src/input.txt") {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        b.iter(|| first_half_optimized(contents.split('\n').collect::<Vec<&str>>()));
    }
}
