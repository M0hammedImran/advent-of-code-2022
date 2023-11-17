use day_02::{first_half, first_half_optimized, second_half, second_half_optimized};

fn main() {
    let contents = match std::fs::read_to_string("./src/input.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    let score_one = first_half(contents.split('\n').collect::<Vec<&str>>());
    let score_one_opt = first_half_optimized(contents.split('\n').collect::<Vec<&str>>());
    let score_two = second_half(contents.split('\n').collect::<Vec<&str>>());
    let score_two_opt = second_half_optimized(contents.split('\n').collect::<Vec<&str>>());

    println!("score_one: {score_one}");
    println!("score_one_opt: {score_one_opt}");
    println!("score_two: {score_two}");
    println!("score_two_opt: {score_two_opt}");
}
