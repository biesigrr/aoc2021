use std::io::BufRead;

fn main() {
    let mut previous_measurement = None;
    let mut counter = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let measurement = line.parse::<u32>().unwrap();

        previous_measurement.map(|m| {
            if measurement > m {
                counter += 1;
            }
        });

        previous_measurement = Some(measurement);
    }

    println!("{}", counter)
}
