use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate zxcvbn;
use zxcvbn::zxcvbn;

fn main() {
    let filename = env::args().nth(1).expect("No filename given");
    let argv2 = env::args().nth(2).expect("No minimum zxcvbn score given");
    let minimum_zxcvbn_score: u8 = argv2.trim().parse().expect("Wrong number in argv[2]");

    if minimum_zxcvbn_score > 4 {
        panic!("Minimum zxcvbn score must be between 0 and 4");
    }

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(password) = line {
                let paspio_entropy = paspio::entropy(&password);
                if let Ok(zxcvbn_stats) = zxcvbn(&password, &[]) {
                    if zxcvbn_stats.score() < minimum_zxcvbn_score {
                        continue;
                    }
                    println!(
                        "'{}' has an entropy of {} (paspio) or {} (zxcvbn) and a score of {}",
                        &password,
                        paspio_entropy,
                        zxcvbn_stats.guesses_log10(),
                        zxcvbn_stats.score()
                    );
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
