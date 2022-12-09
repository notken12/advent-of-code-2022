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

    let mut most = 0;
    let mut current = 0;

    for line in lines {
        if let Ok(line) = line {
            match line.trim().parse::<u32>() {
                Ok(calories) => {
                    current += calories;
                    if current > most {
                        most = current;
                    }
                }
                Err(_e) => {
                    current = 0;
                }
            }
        }
    }

    println!("{}", most);
}
