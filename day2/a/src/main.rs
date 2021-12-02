use std::io::BufRead;

fn main() {

    let mut horizontal_position = 0;
    let mut depth = 0;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        assert_eq!(parts.len(), 2);

        let direction = parts[0];
        let magnitude = parts[1].parse::<u32>().unwrap();

        match direction {
            "up" => depth -= magnitude,
            "down" => depth += magnitude,
            "forward" => horizontal_position += magnitude,
            _ => panic!("where?")
        }
    }

    println!("{}", horizontal_position * depth);
}