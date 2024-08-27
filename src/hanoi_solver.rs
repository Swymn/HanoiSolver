pub struct Hanoi {
    pub rods: [Vec<u32>; 3],
}

pub trait HanoiSolver {
    fn solve(&mut self) -> Result<(), String>;
}

impl Hanoi {
    pub fn new(disks: u32) -> Self {
        Self {
            rods: [
                (0..disks).rev().collect::<Vec<u32>>(),
                Vec::with_capacity(disks as usize),
                Vec::with_capacity(disks as usize),
            ],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_gameboard_with_three_disk() {
        // GIVEN a number of disks
        let disk_amount = 3;

        // WHEN we create an hanoi tower
        let hanoi = Hanoi::new(disk_amount);

        // THEN the hanoi tower should have the following model
        assert_eq!(
            &(0..disk_amount).rev().collect::<Vec<u32>>(),
            hanoi.rods.first().unwrap()
        );
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_create_gameboard_with_five_disk() {
        // GIVEN a number of disks
        let disk_amount = 5;

        // WHEN we create an hanoi tower
        let hanoi = Hanoi::new(disk_amount);

        // THEN the hanoi tower should have the following model
        assert_eq!(
            &(0..disk_amount).rev().collect::<Vec<u32>>(),
            hanoi.rods.first().unwrap()
        );
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }
}
