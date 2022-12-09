fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).unwrap();

    let lines = std::io::BufRead::lines(std::io::BufReader::new(file));

    let mut overlapping_count = 0;

    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            let mut assignments = line.split(",");

            let assignment1: Vec<&str> = assignments.next().unwrap().split("-").collect();
            let assignment1 = (
                assignment1[0].parse::<u32>().unwrap(),
                assignment1[1].parse::<u32>().unwrap(),
            );

            let assignment2: Vec<&str> = assignments.next().unwrap().split("-").collect();
            let assignment2 = (
                assignment2[0].parse::<u32>().unwrap(),
                assignment2[1].parse::<u32>().unwrap(),
            );

            if (assignment1.0 <= assignment2.0 && assignment1.1 >= assignment2.1)
                || (assignment2.0 <= assignment1.0 && assignment2.1 >= assignment1.1)
            {
                overlapping_count += 1;
            }
        }
    }

    println!("{}", overlapping_count);
}
