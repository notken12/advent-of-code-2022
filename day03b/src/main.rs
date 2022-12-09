use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut sum: u32 = 0;

    loop {
        if let (Some(Ok(sack1)), Some(Ok(sack2)), Some(Ok(sack3))) = (lines.next(), lines.next(), lines.next()) {
            let mut present = HashMap::<char, bool>::new();

            for c in sack1.chars() {
                present.insert(c, true);
            }

            for c in sack2.chars() {
                if present.get(&c) != Some(&true) {
                    present.insert(c, false);
                }
            }

            for c in sack3.bytes() {
                let as_char = char::from_u32(c as u32).unwrap();
                if let (Some(_), Some(_)) = (sack1.find(as_char), sack2.find(as_char)) {
                    let value:u32 = if c >= 97 {
                        // Lowercase
                        (c - 97 + 1).into()
                    } else {
                        (c - 65 + 27).into()
                    };

                    sum += value;
                    break;
                }
            }
        } else {
            break;
        }
    }

    println!("{sum}");
}
