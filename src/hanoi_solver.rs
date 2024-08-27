use std::fmt;

#[derive(Debug)]
pub struct Hanoi {
    rods: [Vec<u32>; 3],
    disks_amount: u32,
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
            disks_amount: disks,
        }
    }

    fn move_disk(&mut self, from: usize, to: usize) -> Result<(), String> {
        if let Some(disk) = self.rods[from].pop() {
            if let Some(&top_disk) = self.rods[to].last() {
                if disk > top_disk {
                    self.rods[from].push(disk);
                    return Err(format!("Disk {} is bigger than disk {}", disk, top_disk));
                }
            }
            self.rods[to].push(disk);
            println!("{}", self);
            Ok(())
        } else {
            Err(format!("No disk to move from tower {from}"))
        }
    }

    fn solve_recursive(
        &mut self,
        number_of_disks: u32,
        source_rod_index: usize,
        destination_rod_index: usize,
        auxiliary_rod_index: usize,
    ) -> Result<(), String> {
        if number_of_disks == 0 {
            return Ok(());
        }
        self.solve_recursive(
            number_of_disks - 1,
            source_rod_index,
            auxiliary_rod_index,
            destination_rod_index,
        )?;
        self.move_disk(source_rod_index, destination_rod_index)?;
        self.solve_recursive(
            number_of_disks - 1,
            auxiliary_rod_index,
            destination_rod_index,
            source_rod_index,
        )?;
        Ok(())
    }
}

impl HanoiSolver for Hanoi {
    fn solve(&mut self) -> Result<(), String> {
        let disks = self.disks_amount;
        self.solve_recursive(disks, 0, 2, 1)
    }
}

impl fmt::Display for Hanoi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut format = String::new();
        // Update this string to dynamic add - between every from the disk value
        format.push_str(&format!(
            "\n{0}|{0}\t{0}|{0}\t{0}|{0}\n",
            " ".repeat((self.disks_amount + 1) as usize)
        ));

        for i in (0..self.disks_amount as usize).rev() {
            format.push_str(&format!(
                "{}\t{}\t{}\n",
                self.rods[0].get(i).map_or(
                    format!("{0}|{0}", " ".repeat((self.disks_amount + 1) as usize)),
                    |&disk| format!(
                        "{1}[{0}]{1}",
                        "=".repeat(((disk * 2) + 1) as usize),
                        " ".repeat((self.disks_amount - disk) as usize)
                    )
                ),
                self.rods[1].get(i).map_or(
                    format!("{0}|{0}", " ".repeat((self.disks_amount + 1) as usize)),
                    |&disk| format!(
                        "{1}[{0}]{1}",
                        "=".repeat(((disk * 2) + 1) as usize),
                        " ".repeat((self.disks_amount - disk) as usize)
                    )
                ),
                self.rods[2].get(i).map_or(
                    format!("{0}|{0}", " ".repeat((self.disks_amount + 1) as usize)),
                    |&disk| format!(
                        "{1}[{0}]{1}",
                        "=".repeat(((disk * 2) + 1) as usize),
                        " ".repeat((self.disks_amount - disk) as usize)
                    ),
                )
            ));
        }

        format.push_str(&format!(
            "{0}+{0}\t{0}+{0}\t{0}+{0}",
            "-".repeat((self.disks_amount + 1) as usize)
        ));
        write!(f, "{}", format)
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
    fn shouldmove_disk_from_rod_1_to_rod_2() {
        // GIVEN a hanoi tower with 3 disks
        let mut hanoi = Hanoi::new(3);

        // WHEN we move a disk from rod 1 to rod 2
        let operation_result = hanoi.move_disk(0, 1);

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
        let operation_result = hanoi.move_disk(1, 0);

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
        let operation_result = hanoi.move_disk(0, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_ok());
        assert_eq!(&vec![2, 1], hanoi.rods.first().unwrap());
        assert_eq!(&vec![0], hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());

        // WHEN we move a disk from rod 1 to rod 2
        let operation_result = hanoi.move_disk(0, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_err());
        assert_eq!(&vec![2, 1], hanoi.rods.first().unwrap());
        assert_eq!(&vec![0], hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_solve_the_hanoi_towers_with_three_disks() {
        // GIVEN a hanoi tower with 3 disks
        let disks_amount = 3;
        let mut hanoi = Hanoi::new(disks_amount);

        // WHEN we solve the hanoi tower
        let operation_result = hanoi.solve_recursive(disks_amount, 0, 2, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_ok());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.first().unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![2, 1, 0], hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_solve_the_hanoi_towers_with_five_disks() {
        // GIVEN a hanoi tower with 3 disks
        let disks_amount = 5;
        let mut hanoi = Hanoi::new(disks_amount);

        // WHEN we solve the hanoi tower
        let operation_result = hanoi.solve_recursive(disks_amount, 0, 2, 1);

        // THEN the hanoi tower should have the following model
        assert!(operation_result.is_ok());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.first().unwrap());
        assert_eq!(&vec![] as &Vec<u32>, hanoi.rods.get(1).unwrap());
        assert_eq!(&vec![4, 3, 2, 1, 0], hanoi.rods.last().unwrap());
    }

    #[test]
    fn should_display_raw_hanoi_towers_with_3_disks() {
        // GIVEN a hanoi towers
        let hanoi = Hanoi::new(3);

        // WHEN we display the hanoi towers
        let display = format!("{}", hanoi);

        // THEN the hanoi towers should be displayed as
        println!("{}", hanoi);
        assert_eq!("\n    |    \t    |    \t    |    \n   [=]   \t    |    \t    |    \n  [===]  \t    |    \t    |    \n [=====] \t    |    \t    |    \n----+----\t----+----\t----+----", display);
    }

    #[test]
    fn should_display_raw_hanoi_towers_with_five_disks() {
        // GIVEN a hanoi towers
        let hanoi = Hanoi::new(5);

        // WHEN we display the hanoi towers
        let display = format!("{}", hanoi);

        // THEN the hanoi towers should be displayed as
        println!("{}", hanoi);
        assert_eq!("\n      |      \t      |      \t      |      \n     [=]     \t      |      \t      |      \n    [===]    \t      |      \t      |      \n   [=====]   \t      |      \t      |      \n  [=======]  \t      |      \t      |      \n [=========] \t      |      \t      |      \n------+------\t------+------\t------+------", display);
    }
}
