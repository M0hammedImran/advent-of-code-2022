use std::fs;

fn main() {
    let contents = match fs::read_to_string("./src/input.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    let mut contents = contents
        .split("\n\n")
        .map(|text| {
            text.split('\n')
                .map(|s| s.parse::<i32>().unwrap())
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .collect::<Vec<i32>>();

    contents.sort_by(|a, b| b.cmp(a));

    println!("Result:1_1:\n{:?}", contents);

    println!("Result:1_2:\n{:?}", contents[0] + contents[1] + contents[2]);
}
