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
    let mut max_y = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_x = i32::MAX;

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

                let i_range = if vec.0 > 0 {0..=vec.0} else {vec.0..=0};
                let j_range = if vec.1 > 0 {0..=vec.1} else {vec.1..=0};

                for j in j_range {
                    for i in i_range.clone() {
                        let location = (from.0 + i, from.1 + j);
                        grid.insert(location.clone(), Cell::Rock);
                        if location.1 > max_y {
                            max_y = location.1;
                        }
                        if location.1 < min_y {
                            min_y = location.1;
                        }
                        if location.0 > max_x {
                            max_x = location.0;
                        }
                        if location.0 < min_x {
                            min_x = location.0;
                        }
                    }
                }
            }
        }
    }

    let sand_source = (500, 0);
    let mut sand_grain = (500, 0);
    let mut answer = 0;
    loop {
        let (x, y) = sand_grain;
        let down = (x, y + 1);
        let downleft = (x - 1, y + 1);
        let downright = (x + 1, y + 1);

        if y + 1 == max_y + 2 {
            // On infinite floor
            // Sand comes to rest
            sand_grain = sand_source;
            grid.insert(sand_grain, Cell::Sand);
            answer += 1;
        } else {
            if let None = grid.get(&down) {
                move_grain(&mut sand_grain, &mut grid, down);
            } else if let None = grid.get(&downleft) {
                move_grain(&mut sand_grain, &mut grid, downleft);
            } else if let None = grid.get(&downright) {
                move_grain(&mut sand_grain, &mut grid, downright);
            } else {
                if sand_grain == sand_source {
                    // Sand source blocked
                    answer += 1;
                    break;
                }
                
                sand_grain = sand_source;
                grid.insert(sand_grain, Cell::Sand);
                answer += 1;
            }
        }

        
    }

    for y in min_y-20..=max_y+10 {
        for x in min_x..=max_x {
            if x == sand_source.0 && y == min_y - 10 {
                print!("X")
            } else {
                match grid.get(&(x,y)) {
                    Some(c) => match c {
                        Cell::Rock => print!("#"),
                        Cell::Sand => {
                            print!("o")
                        }
                    },
                    None => print!(" ")
                }
            }
            
        }
        println!();
    }

    println!("{}", answer);
}

fn move_grain(sand_grain: &mut Point, grid: &mut HashMap<Point, Cell>, location: Point) {
    grid.remove(&sand_grain);
    grid.insert(location, Cell::Sand);
    *sand_grain = location;
}
