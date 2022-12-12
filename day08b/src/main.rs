#[derive(Debug)]
struct Tree {
    height: i8,
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

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
                    })
                    .collect(),
            );
            size = line.len();
        }
    }

    let mut max_visibility = 0;

    for row in 0..size {
        for col in 0..size {
            let mut visibility = 1;
            for dir in DIRECTIONS {
                visibility *= check_visible(&mut grid, size, dir.0, dir.1, row, col);
            }
            if visibility > max_visibility {
                max_visibility = visibility;
            }
        }
    }

    println!("{}", max_visibility);
}

fn check_visible(
    grid: &mut Vec<Vec<Tree>>,
    size: usize,
    dir_row: isize,
    dir_col: isize,
    start_row: usize,
    start_col: usize,
) -> u32 {
    let mut total_visible = 0;
    let max_height = grid[start_row][start_col].height;
    let mut row = start_row as isize + dir_row;
    let mut col = start_col as isize + dir_col;
    while (0..size as isize).contains(&row) && (0..size as isize).contains(&col) {
        let current = &mut grid[row as usize][col as usize];
        total_visible += 1;
        if current.height >= max_height {
            break;
        }

        row += dir_row;
        col += dir_col;
    }
    total_visible
}
