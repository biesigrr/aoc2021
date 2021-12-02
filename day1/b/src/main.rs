use std::collections::VecDeque;
use std::io::BufRead;

fn main() {
    const WINDOW_LEN: usize = 3;

    let mut counter = 0;
    let mut window = VecDeque::with_capacity(WINDOW_LEN);
    let mut previous_sum = None;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let measurement = line.parse::<u32>().unwrap();
        window.push_back(measurement);

        if WINDOW_LEN == window.len() {
            let sum = window.iter().sum::<u32>();
            previous_sum.map(|m| {
                if sum > m {
                    counter += 1;
                }
            });

            previous_sum = Some(sum);
            window.pop_front();
        }
    }

    println!("{}", counter)
}
