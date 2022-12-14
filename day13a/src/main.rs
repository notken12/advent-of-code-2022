use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Item {
    Number(i32),
    Array(Vec<Item>),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut pair_index = 1;
    let mut a: Vec<Vec<Item>> = Vec::new();
    let mut answer = 0;

    for line in lines {
        if let Ok(line) = line {
            if line.len() == 0 {
                // compare
                if let Order::Right = compare(&a[0], &a[1]) {
                    answer += pair_index;
                }

                // clear array
                a.clear();
                pair_index += 1;
            } else {
                let line = line.trim();

                let v: Vec<Item> = serde_json::from_str(&line).unwrap();
                a.push(v);
            }
        }
    }

    println!("{}", answer);
}

enum Order {
    Right,
    Wrong,
    Neutral,
}

fn compare(v1: &Vec<Item>, v2: &Vec<Item>) -> Order {
    for i in 0..=v1.len() {
        // If left runs out, it's in the right order
        if i >= v1.len() && i < v2.len() {
            return Order::Right;
        }

        // If right runs out, it's not in the right order
        if i < v1.len() && i >= v2.len() {
            return Order::Wrong;
        }
        if i >= v1.len() {
            break;
        }

        let mut comp = vec![v1[i].clone(), v2[i].clone()];

        for j in 0..2 {
            if let (Item::Number(n), Item::Array(_)) = (&comp[j], &comp[(j + 1) % 2]) {
                comp[j] = Item::Array(vec![Item::Number(*n)]);
            }
        }

        match (&comp[0], &comp[1]) {
            (Item::Array(v1), Item::Array(v2)) => {
                let recurse = compare(v1, v2);
                if let Order::Neutral = recurse {
                    continue;
                } else {
                    return recurse;
                }
            }
            (Item::Number(n1), Item::Number(n2)) => {
                // If left side is smaller, it's in the right order
                if n1 < n2 {
                    return Order::Right;
                } else if n1 > n2 {
                    // Not in right order
                    return Order::Wrong;
                } else {
                    // The same
                    continue;
                }
            }
            _ => panic!(),
        }
    }
    Order::Neutral
}
