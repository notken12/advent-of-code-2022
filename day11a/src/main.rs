#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test_divisible_by: u32,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug)]
enum Operation {
    Times { amount: u32 },
    Plus { amount: u32 },
    Square,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut starting_items: Vec<u32> = Vec::new();
    let mut current_operation = Operation::Square;
    let mut test_divisible_by = 0;
    let mut if_true = 0;
    let mut if_false = 0;

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim_end();

            if line.len() < 5 {
                monkeys.push(Monkey {
                    items: starting_items.clone(),
                    operation: current_operation.clone(),
                    test_divisible_by,
                    if_true,
                    if_false,
                });
                continue;
            }

            match line.get(0..5).unwrap() {
                "Monke" => {}
                "  Sta" => {
                    starting_items = line
                        .get(18..)
                        .unwrap()
                        .split(", ")
                        .map(|x| x.parse().unwrap())
                        .collect();
                }
                "  Ope" => {
                    let sign = line.get(23..24).unwrap();
                    match line.get(25..).unwrap().parse::<u32>() {
                        Ok(amount) => {
                            current_operation = match sign {
                                "*" => Operation::Times { amount },
                                "+" => Operation::Plus { amount },
                                &_ => panic!(),
                            };
                        }
                        Err(_) => current_operation = Operation::Square,
                    }
                }
                "  Tes" => {
                    test_divisible_by = line.get(21..).unwrap().parse().unwrap();
                }
                "    I" => {
                    if line.get(7..8).unwrap() == "t" {
                        if_true = line.get(29..).unwrap().parse().unwrap();
                    } else {
                        if_false = line.get(30..).unwrap().parse().unwrap();
                    }
                }
                &_ => panic!(),
            }
        }
    }

    println!("{:#?}", monkeys);
}
