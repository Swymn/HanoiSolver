use hanoi_solver::hanoi_solver::{Hanoi, HanoiSolver};

fn main() {
    let disk_amount = 3;
    let mut hanoi = Hanoi::new(disk_amount);

    if let Err(error_message) = hanoi.solve() {
        eprintln!("{}", error_message);
    }
}
