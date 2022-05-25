use std::io;
use clearscreen::clear;

const pos: i64 = [1.5, 2.0, 2.5, 3.0, 3.5, 4];
const neg: i64 = [2/3, 2/4, 2/5, 2/6, 2/7, 2/8];
const spd_title: String = [
    "BENEFICIAL NATURE, 252 EVs:",
    "NEUTRAL NATURE, 252 EVs:",
    "NEUTRAL NATURE, 0 EVs:"
];

fn main() {
    println!("STAT CALC LOADED.");

    loop {
        println!("ENTER STAT VALUE:");
        let mut stat = String::new();

        // Get input from user
        io::stdin()
            .read_line(&mut stat)
            .expect("FAILED TO READ LINE");

        // 
        let stat: u32 = match stat.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // Clear terminal window
        clear().expect("FAILED TO CLEAR SCREEN");

        // Mathematical calculations to obtain values
        let neu_max: i32 = ((stat + 1) as f64 / 1.1) as i32;
        let neu_min: i32 = neu_max - 63;
        let base: i32 = ((neu_min - 36) as f64 / 2.0) as i32;

        // Generate arrays to iterate over
        let stat_dist = [stat as i32, neu_max, neu_min];

        // Print base value to screen
        println!("{0: >70} {1: >4}", "BASE:", base);

        // Print the stats for each distribution
        for i in 0..3 {
            println!("{0: >70} {1: >4}", spd_title[i], stat_dist[i]);
            print_stats(stat_dist[i])
        }
    }

}

fn print_stats(stat: i32) {
    println!("{}", stat);
}