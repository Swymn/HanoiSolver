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

    fn _move_disk(&mut self, from: usize, to: usize) -> Result<(), String> {
        if let Some(disk) = self.rods[from].pop() {
            if let Some(&top_disk) = self.rods[to].last() {
                if disk > top_disk {
                    self.rods[from].push(disk);
                    return Err(format!("Disk {} is bigger than disk {}", disk, top_disk));
                }
            }
            self.rods[to].push(disk);
            Ok(())
        } else {
            Err(format!("No disk to move from tower {from}"))
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
        assert_eq!(&vec![2, 1, 0], hanoi.rods.first().unwrap());
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
        assert_eq!(&vec![4, 3, 2, 1, 0], hanoi.rods.first().unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_move_disk_from_rod_1_to_rod_2() {
        // GIVEN a hanoi tower with 3 disks
        let mut hanoi = Hanoi::new(3);

        // WHEN we move a disk from rod 1 to rod 2
        let operation_result = hanoi._move_disk(0, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_ok());
        assert_eq!(&vec![2, 1], hanoi.rods.first().unwrap());
        assert_eq!(&vec![0], hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_not_move_from_rod_2_to_rod_1() {
        // GIVEN a hanoi tower with 3 disks
        let mut hanoi = Hanoi::new(3);

        // WHEN we move a disk from rod 2 to rod 1
        let operation_result = hanoi._move_disk(1, 0);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_err());
        assert_eq!(&vec![2, 1, 0], hanoi.rods.first().unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_not_move_big_disk_on_small_disk() {
        // GIVEN a hanoi tower with 3 disks
        let mut hanoi = Hanoi::new(3);

        // WHEN we move a disk from rod 1 to rod 2
        let operation_result = hanoi._move_disk(0, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_ok());
        assert_eq!(&vec![2, 1], hanoi.rods.first().unwrap());
        assert_eq!(&vec![0], hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());

        // WHEN we move a disk from rod 1 to rod 2
        let operation_result = hanoi._move_disk(0, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_err());
        assert_eq!(&vec![2, 1], hanoi.rods.first().unwrap());
        assert_eq!(&vec![0], hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }
}
