#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisible_by: u64,
    if_true: usize,
    if_false: usize,
    inspected_amount: u64,
}

#[derive(Clone, Debug)]
enum Operation {
    Times { amount: u64 },
    Plus { amount: u64 },
    Square,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut starting_items: Vec<u64> = Vec::new();
    let mut current_operation = Operation::Square;
    let mut test_divisible_by = 0;
    let mut if_true = 0;
    let mut if_false = 0;

    let mut lcm = 1;

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
                    inspected_amount: 0,
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
                    match line.get(25..).unwrap().parse::<u64>() {
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
                    lcm *= test_divisible_by;
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
    monkeys.push(Monkey {
        items: starting_items.clone(),
        operation: current_operation.clone(),
        test_divisible_by,
        if_true,
        if_false,
        inspected_amount: 0,
    });

    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            let i = 0;
            while monkeys[m].items.len() > 0 {
                monkeys[m].items[i] = match monkeys[m].operation {
                    Operation::Square => monkeys[m].items[i] * monkeys[m].items[i],
                    Operation::Plus { amount } => monkeys[m].items[i] + amount,
                    Operation::Times { amount } => monkeys[m].items[i] * amount,
                };
                // % by LCM
                monkeys[m].items[i] %= lcm;

                let tossed = monkeys[m].items.remove(i);

                monkeys[m].inspected_amount += 1;

                let toss_to = if tossed % monkeys[m].test_divisible_by == 0 {
                    // Toss to other monkey
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys[toss_to].items.push(tossed);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected_amount);

    let answer =
        monkeys[monkeys.len() - 1].inspected_amount * monkeys[monkeys.len() - 2].inspected_amount;

    println!("{:#?}", answer);
}
