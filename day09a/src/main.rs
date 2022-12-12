use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut head_location = Point { x: 0, y: 0 };
    let mut tail_location = Point { x: 0, y: 0 };

    let mut visited_locations: HashMap<Point, u32> = HashMap::new();

    let visited_times = visited_locations.get(&tail_location).unwrap_or(&0);
    visited_locations.insert(tail_location.clone(), visited_times + 1);

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            let dir = match line.get(0..1).unwrap() {
                "R" => Point { x: 1, y: 0 },
                "L" => Point { x: -1, y: 0 },
                "U" => Point { x: 0, y: 1 },
                "D" => Point { x: 0, y: -1 },
                &_ => panic!(),
            };

            let amount: i32 = line.get(2..).unwrap().parse().unwrap();
            for _i in 0..amount {
                head_location.x += dir.x;
                head_location.y += dir.y;

                let dx = head_location.x - tail_location.x;
                let dy = head_location.y - tail_location.y;

                if dx.abs() > 1 || dy.abs() > 1 {
                    tail_location.x += dx.signum();
                    tail_location.y += dy.signum();

                    let visited_times = visited_locations.get(&tail_location).unwrap_or(&0);
                    visited_locations.insert(tail_location.clone(), visited_times + 1);
                }
            }
        }
    }

    let mut total_visited = 0;
    for (_point, times) in &visited_locations {
        if times >= &1 {
            total_visited += 1;
        }
    }

    println!("{}", total_visited);
}
