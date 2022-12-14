use std::collections::HashMap;

enum Cell {
    Rock,
    Sand,
}

type Point = (i32, i32);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if let Some(s) = args.get(1) {
        s
    } else {
        "input.txt"
    };

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut grid: HashMap<Point, Cell> = HashMap::new();
    let mut max = 0;

    for line in lines {
        if let Ok(line) = line {
            let positions: Vec<(i32, i32)> = line
                .trim()
                .split(" -> ")
                .map(|s| {
                    let mut split = s.split(",");
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    )
                })
                .collect();

            for i in 1..positions.len() {
                let from = positions[i - 1];
                let to = positions[i];
                let vec = (to.0 - from.0, to.1 - from.1);

                for i in 0..=vec.0 {
                    for j in 0..=vec.1 {
                        let location = (from.0 + i, from.1 + j);
                        grid.insert(location.clone(), Cell::Rock);
                        if location.1 > max {
                            max = location.1;
                        }
                    }
                }
            }
        }
    }

    let sand_source = (500, 0);
    let mut sand_grain = (500, 0);
    loop {
        let (x, y) = sand_grain;
        let down = (x, y + 1);
        let downleft = (x - 1, y + 1);
        let downright = (x + 1, y + 1);
        if let None = grid.get(&down) {
            move_grain(&mut sand_grain, &mut grid, down);
        } else if let None = grid.get(&downleft) {
            move_grain(&mut sand_grain, &mut grid, downleft);
        } else if let None = grid.get(&downright) {
            move_grain(&mut sand_grain, &mut grid, downright);
        } else {
            sand_grain = sand_source;
        }
        if sand_
    }
}

fn move_grain(sand_grain: &mut Point, grid: &mut HashMap<Point, Cell>, location: Point) {
    grid.remove(&sand_grain);
    grid.insert(location, Cell::Rock);
    *sand_grain = location;
}
