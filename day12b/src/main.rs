use std::{
    collections::{BinaryHeap, HashMap},
};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    row: usize,
    col: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut end_location = Point { row: 0, col: 0 };

    let mut row = 0;
    for line in lines {
        if let Ok(line) = line {
            map.push(Vec::new());
            let line = line.trim();

            let mut col = 0;
            for c in line.chars() {
                if c == 'S' {
                    map[row].push('a' as u8 - 97);
                } else if c == 'E' {
                    end_location = Point { row, col };
                    map[row].push('z' as u8 - 97);
                } else {
                    map[row].push(c as u8 - 97);
                }
                // map[row].push(0);
                col += 1;
            }
            row += 1;
        }
    }

    let mut answer = usize::MAX;

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] != 'a' as u8 - 97 {
                continue;
            }

            let steps = a_star(
                Point { row, col },
                end_location.clone(),
                |p| distance(p, &end_location) as i32,
                |p| {
                    let mut neighbors = Vec::new();
                    for (i, j) in DIRECTIONS {
                        let nrow = p.row as isize + j;
                        let ncol = p.col as isize + i;
                        if ncol >= 0
                            && ncol < map[0].len() as isize
                            && nrow >= 0
                            && nrow < map.len() as isize
                        {
                            let nrow = nrow as usize;
                            let ncol = ncol as usize;

                            if map[nrow][ncol] as i8 - map[p.row][p.col] as i8 <= 1 {
                                neighbors.push(Point {
                                    row: nrow,
                                    col: ncol,
                                });
                            }
                        }
                    }
                    neighbors
                },
            );

            if steps.len() > 0 && steps.len() - 1 < answer {
                answer = steps.len() - 1;
            }
        }
    }

    println!("{}", answer - 1);
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Node {
    point: Point,
    g_score: i32,
    f_score: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn a_star<F, N>(start: Point, goal: Point, h: F, n: N) -> Vec<Point>
where
    F: Fn(&Point) -> i32,
    N: Fn(&Point) -> Vec<Point>,
{
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        point: start.clone(),
        g_score: 0,
        f_score: h(&start),
    });

    let mut g_score: HashMap<Point, i32> = HashMap::new();
    g_score.insert(start.clone(), 0);

    let mut f_score: HashMap<Point, i32> = HashMap::new();
    f_score.insert(start.clone(), h(&start));

    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut open_set_contains: HashMap<Point, bool> = HashMap::new();

    while !open_set.is_empty() {
        let current = {
            let current = open_set.peek().unwrap();
            if current.point == goal {
                return reconstruct_path(came_from, current);
            }
            current.clone()
        };

        open_set.pop();
        open_set_contains.insert(current.point.clone(), false);
        for point in n(&current.point) {
            let mut neighbor = Node {
                point: point.clone(),
                g_score: g_score.get(&point).unwrap_or(&i32::MAX).clone(),
                f_score: f_score.get(&point).unwrap_or(&i32::MAX).clone(),
            };

            let tentative_g_score =
                current.g_score + distance(&current.point, &neighbor.point) as i32;
            if tentative_g_score < neighbor.g_score {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(neighbor.point.clone(), current.point.clone());
                g_score.insert(neighbor.point.clone(), tentative_g_score);
                f_score.insert(
                    neighbor.point.clone(),
                    tentative_g_score + h(&neighbor.point),
                );

                neighbor.g_score = tentative_g_score;
                neighbor.f_score = tentative_g_score + h(&neighbor.point);

                if let None = open_set_contains.get(&neighbor.point) {
                    open_set_contains.insert(neighbor.point.clone(), true);
                    open_set.push(neighbor);
                }
            }
        }
    }

    Vec::new()
}

fn distance(a: &Point, b: &Point) -> usize {
    a.col.abs_diff(b.col) + a.row.abs_diff(b.row)
}

fn reconstruct_path(came_from: HashMap<Point, Point>, current: &Node) -> Vec<Point> {
    let mut total_path = vec![current.point.clone()];
    let mut current = current.point.clone();
    while let Some(v) = came_from.get(&current) {
        current = v.clone();
        total_path.insert(0, current.clone());
    }
    total_path
}
