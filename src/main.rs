use std::io;

// Multipliers
const POS: [f64; 6] = [1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
const NEG: [f64; 6] = [2.0/3.0, 2.0/4.0, 2.0/5.0, 2.0/6.0, 2.0/7.0, 2.0/8.0];
const DIST: [&str; 3] = [
    "BENEFICIAL NATURE, 252 EVs:",
    "NEUTRAL NATURE, 252 EVs:",
    "NEUTRAL NATURE, 0 EVs:"
];

fn print_stats(stat: f64) {
    for i in 0..6 {
        println!("{0: >40}+{1}: {2: <7}{3: >7}-{4}: {5}", 
            "", i + 1, (stat * POS[i]) as i32,
            "", i + 1, (stat * NEG[i]) as i32
        );
    }
}

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
        //clear().expect("FAILED TO CLEAR SCREEN");
        print!("{}[2J", 27 as char);

        // Mathematical calculations to obtain values
        let neu_max: i32 = ((stat + 1) as f64 / 1.1) as i32;
        let neu_min: i32 = neu_max - 63;
        let base: i32 = ((neu_min - 36) as f64 / 2.0) as i32;

        // Generate arrays to iterate over
        let stat_dist = [stat as i32, neu_max, neu_min];

        // Print base value to screen
        println!("{0} {1: >4}", "BASE:", base);

        // Print the stats for each distribution
        for i in 0..3 {
            println!("{0: <30} {1}", DIST[i], stat_dist[i]);
            print_stats(stat_dist[i] as f64)
        }
        for _i in 0..3 {
            println!();
        }
    }

}