use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut file = std::fs::File::open(filename).unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut last_4: Vec<char> = Vec::new();
    let mut chars = text.chars();

    for i in 0.. {
        let c = chars.next();
        if let Some(c) = c {
            last_4.insert(last_4.len(), c);
            if last_4.len() > 4 {
                last_4.remove(0);
            }

            if last_4.len() == 4 {
                let mut repeating = false;
                'outer: for i in 0..last_4.len() - 1 {
                    for j in i + 1..last_4.len() {
                        if last_4[i] == last_4[j] {
                            repeating = true;
                            break 'outer;
                        }
                    }
                }

                if !repeating {
                    println!("{}", i + 1);
                    break;
                }
            }
        } else {
            break;
        }
    }
}
