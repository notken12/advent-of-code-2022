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

    let mut rope: Vec<Point> = Vec::new();
    for _ in 0..10 {
        rope.push(Point { x: 0, y: 0 });
    }

    let mut visited_locations: HashMap<Point, u32> = HashMap::new();

    let visited_times = visited_locations.get(&rope[rope.len() - 1]).unwrap_or(&0);
    visited_locations.insert(rope[rope.len() - 1].clone(), visited_times + 1);

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
            for _ in 0..amount {
                rope[0].x += dir.x;
                rope[0].y += dir.y;

                for i in 1..rope.len() {
                    let dx = rope[i - 1].x - rope[i].x;
                    let dy = rope[i - 1].y - rope[i].y;

                    if dx.abs() > 1 || dy.abs() > 1 {
                        rope[i].x += dx.signum();
                        rope[i].y += dy.signum();

                        if i == rope.len() - 1 {
                            let visited_times = visited_locations.get(&rope[i]).unwrap_or(&0);
                            visited_locations.insert(rope[i].clone(), visited_times + 1);
                        }
                    }
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
