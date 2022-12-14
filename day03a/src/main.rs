use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut sum: u32 = 0;

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();
            let comp1 = line.get(0..line.len() / 2).unwrap();
            let comp2 = line.get(line.len() / 2..line.len()).unwrap();

            let mut present = HashMap::<char, bool>::new();

            for c in comp1.chars() {
                present.insert(c, true);
            }

            for c in comp2.bytes() {
                if present.get(&c.into()) == Some(&true) {
                    let value:u32 = if c >= 97 {
                        // Lowercase
                        (c - 97 + 1).into()
                    } else {
                        (c - 65 + 27).into()
                    };
                    // println!("{:?}, {}", char::from_u32(c as u32), value);

                    sum += value;
                    break;
                }
            }
        }
    }

    println!("{sum}");
}
