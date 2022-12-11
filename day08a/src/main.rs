#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut grid: Vec<Vec<Tree>> = Vec::new();

    let mut size = 0;

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            grid.push(
                line.chars()
                    .map(|c| Tree {
                        height: c.to_digit(10).unwrap() as i8,
                        visible: false,
                    })
                    .collect(),
            );
            size = line.len();
        }
    }

    let mut total_visible = 0;

    // Look left->right
    check_visible(
        &mut total_visible,
        &mut grid,
        size,
        false,
        Direction::Horizontal,
    );
    // Look right->left
    check_visible(
        &mut total_visible,
        &mut grid,
        size,
        true,
        Direction::Horizontal,
    );

    // Look up->down
    check_visible(
        &mut total_visible,
        &mut grid,
        size,
        false,
        Direction::Vertical,
    );
    // Look down->up
    check_visible(
        &mut total_visible,
        &mut grid,
        size,
        true,
        Direction::Vertical,
    );

    for row in grid {
        for tree in row {
            if tree.visible {
                print!("\u{001b}[31m");
            } else {
                print!("\u{001b}[0m");
            }
            print!("{}", tree.height);
        }
        println!("\u{001b}[0m");
    }

    println!("\ntotal visible: {}", total_visible);
}

enum Direction {
    Horizontal,
    Vertical,
}

fn check_visible(
    total_visible: &mut u32,
    grid: &mut Vec<Vec<Tree>>,
    size: usize,
    rev: bool,
    dir: Direction,
) {
    for row in 0..size {
        let mut tallest: i8 = -1;
        let mut col = if rev { size - 1 } else { 0 };
        while col <= size - 1 {
            let mut current = match dir {
                Direction::Horizontal => &mut grid[row][col],
                Direction::Vertical => &mut grid[col][row],
            };
            if tallest < current.height {
                if !current.visible {
                    current.visible = true;
                    *total_visible += 1;
                }
                tallest = current.height;
                if tallest == 9 {
                    break;
                }
            }

            if rev && col == 0 {
                break;
            }

            if rev {
                col -= 1;
            } else {
                col += 1;
            }
        }
    }
}
