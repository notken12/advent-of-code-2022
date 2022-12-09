use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();

    let lines = io::BufReader::new(file).lines();

    let mut current = 0;

    let mut amounts = Vec::<u32>::new();

    for line in lines {
        if let Ok(line) = line {
            match line.trim().parse::<u32>() {
                Ok(calories) => {
                    current += calories;
                }
                Err(_e) => {
                    amounts.push(current);
                    current = 0;
                }
            }
        }
    }

    // Descending sort
    amounts.sort_by(|a, b| b.cmp(a));

    let top3 = amounts[0] + amounts[1] + amounts[2];
    println!("{top3}");
}
