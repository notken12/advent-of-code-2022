fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut x = 1;
    let mut cycle = 1;

    let mut answer = 0;

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            if line == "noop" {
                cycle += 1;
                check_answer(x, cycle, &mut answer);
            } else {
                // addx
                let amount: i32 = line.get(5..).unwrap().parse().unwrap();
                for i in 0..2 {
                    cycle += 1;
                    if i == 1 {
                        x += amount;
                    }
                    check_answer(x, cycle, &mut answer);
                }
            }
        }
    }

    println!("{}", answer);
}

fn check_answer(x: i32, cycle: u32, answer: &mut i32) {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        *answer += check_signal_strength(x, cycle);
    }
}

fn check_signal_strength(x: i32, cycle: u32) -> i32 {
    x * cycle as i32
}
