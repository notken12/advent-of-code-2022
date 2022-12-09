use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut sum = 0;

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();
            let comp1 = line.get(0..line.len() / 2).unwrap();
            let comp2 = line.get(line.len() / 2..).unwrap();

            let mut present = HashMap::<char, bool>::new();

            for char in comp1.chars() {
                present.insert(char, true);
            }

            for char in comp2.bytes() {
                if present.get(&char.into()) == Some(&true) {
                    let value = if char < 97 {
                        // Uppercase

                    }
                }
            }
        }
    }
}
