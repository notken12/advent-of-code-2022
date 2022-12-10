use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();

    for line in lines {
        if let Ok(line) = line {
            if let Some("m") = line.get(0..1) {
                let words: Vec<&str> = line.split(" ").collect();

                let quantity = words[1].parse::<u32>().unwrap();
                let from = words[3].parse::<u32>().unwrap();
                let to = words[5].parse::<u32>().unwrap();

                for _ in 0..quantity {
                    let item = stacks.get_mut(&from).unwrap().remove(0);
                    stacks.get_mut(&to).unwrap().insert(0, item);
                }
            } else {
                for i in (1..line.len()).step_by(4) {
                    if let Some(c) = line.get(i..=i) {
                        let c = c.chars().next().unwrap();
                        if c == ' ' {
                            continue;
                        }

                        let stack_idx = (i as u32 - 1) / 4 + 1;
                        if c.is_numeric() {
                            if stacks.get(&stack_idx) == None {
                                stacks.insert(stack_idx, Vec::new());
                            }
                        } else {
                            if let Some(stack) = stacks.get_mut(&stack_idx) {
                                stack.push(c);
                            } else {
                                stacks.insert(stack_idx, vec![c]);
                            }
                        }
                    } else {
                        unreachable!();
                    }
                }
            }
        }
    }

    for i in 1.. {
        let stack = stacks.get(&i);
        if let Some(stack) = stack {
            print!("{}", stack[0]);
        } else {
            break;
        }
    }

    println!();
}
