use std::{env, io, io::BufReader};

use hanoi_solver::{
    hanoi_solver::{Hanoi, HanoiSolver},
    io::get_disk_amount,
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    let disk_amount = get_disk_amount(&args, &mut reader);
    let mut hanoi = Hanoi::new(disk_amount);

    if let Err(error_message) = hanoi.solve() {
        eprintln!("{}", error_message);
    }
}
