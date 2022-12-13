const SPRITE_WIDTH: usize = 3;
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

type Screen = [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut x = 1;
    let mut cycle = 1;

    let mut screen: Screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            if line == "noop" {
                draw_pixel(&mut screen, x, cycle);
                cycle += 1;
            } else {
                // addx
                let amount: i32 = line.get(5..).unwrap().parse().unwrap();
                for i in 0..2 {
                    draw_pixel(&mut screen, x, cycle);
                    cycle += 1;
                    if i == 1 {
                        x += amount;
                    }
                }
            }
        }
    }

    draw_pixel(&mut screen, x, cycle);

    for row in screen {
        for col in row {
            print!("{}", if col { "#" } else { " " });
        }
        println!();
    }
}

fn draw_pixel(screen: &mut Screen, x: i32, cycle: u32) {
    let row = (cycle - 1) as usize / SCREEN_WIDTH;
    let col = (cycle - 1) as usize % SCREEN_WIDTH;

    if col >= x as usize - SPRITE_WIDTH / 2 && col <= x as usize + SPRITE_WIDTH / 2 {
        screen[row][col] = true;
    }
}
