use std::io;

fn get_disk_command_line_parameter(args: &[String]) -> Result<Option<u32>, String> {
    if args.is_empty() {
        return Ok(None);
    }

    let disk_amount = args[0].parse::<u32>();
    match disk_amount {
        Ok(amount) => Ok(Some(amount)),
        Err(_) => Err(format!("Invalid disk amount: {}", args[0])),
    }
}
// Function to read disk amount from user input
fn get_disk_amount_from_user_input<R: io::BufRead>(reader: &mut R) -> Result<Option<u32>, String> {
    let mut input = String::new();
    if let Err(error) = reader.read_line(&mut input) {
        return Err(error.to_string());
    }

    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        trimmed.parse::<u32>().map(Some).map_err(|e| e.to_string())
    }
}

pub fn get_disk_amount<R: io::BufRead>(args: &[String], reader: &mut R) -> u32 {
    let disk_amount = get_disk_command_line_parameter(args);

    match disk_amount {
        Ok(Some(amount)) => amount,
        _ => {
            println!("Please enter the amount of disks:");
            loop {
                let disk_amount = get_disk_amount_from_user_input(reader);

                match disk_amount {
                    Ok(Some(amount)) => {
                        return amount;
                    }
                    _ => {
                        println!("Invalid disk amount. Please enter a valid number:");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn should_get_command_line_parameter() {
        // GIVEN a command line parameter
        let args = vec!["3".to_string()];

        // WHEN we parse the command line parameter
        let disk_amount = get_disk_command_line_parameter(&args);

        // THEN the disk amount should be 3
        assert!(disk_amount.is_ok());
        assert_eq!(Ok(Some(3)), disk_amount);
    }

    #[test]
    fn should_return_error_because_of_parsing() {
        // GIVEN a command line parameter
        let args = vec!["3az".to_string()];

        // WHEN we parse the command line parameter
        let disk_amount = get_disk_command_line_parameter(&args);

        // THEN the disk amount should be 3
        assert!(disk_amount.is_err());
        assert_eq!(Err("Invalid disk amount: 3az".to_string()), disk_amount);
    }

    #[test]
    fn should_return_empty_option() {
        // GIVEN no command line parameter
        let args = vec![];

        // WHEN we parse the command line parameter
        let disk_amount = get_disk_command_line_parameter(&args);

        // THEN the disk amount should be 3
        assert!(disk_amount.is_ok());
        assert_eq!(Ok(None), disk_amount);
    }

    #[test]
    fn should_get_disk_amount_from_user_input() {
        // GIVEN a user input
        let input = "3\n";
        let mut cursor = Cursor::new(input);

        // WHEN we read the disk amount from user input
        let disk_amount = get_disk_amount_from_user_input(&mut cursor);

        // THEN the disk amount should be 3
        assert!(disk_amount.is_ok());
        assert_eq!(Ok(Some(3)), disk_amount);
    }

    #[test]
    fn should_get_error_because_of_parsing() {
        // GIVEN a user input
        let input = "3az\n";
        let mut cursor = Cursor::new(input);

        // WHEN we read the disk amount from user input
        let disk_amount = get_disk_amount_from_user_input(&mut cursor);

        // THEN the disk amount should be 3
        assert!(disk_amount.is_err());
        assert_eq!(
            Err("invalid digit found in string".to_string()),
            disk_amount
        );
    }

    #[test]
    fn should_return_empty_option_from_user_input() {
        // GIVEN a user input
        let input = "\n";
        let mut cursor = Cursor::new(input);

        // WHEN we read the disk amount from user input
        let disk_amount = get_disk_amount_from_user_input(&mut cursor);

        // THEN the disk amount should be 3
        assert!(disk_amount.is_ok());
        assert_eq!(Ok(None), disk_amount);
    }

    #[test]
    fn should_get_disk_amount() {
        // GIVEN a command line parameter
        let args = vec![];
        let mut cursor = Cursor::new("3\n");

        // WHEN we get the disk amount
        let disk_amount = get_disk_amount(&args, &mut cursor);

        // THEN the disk amount should be 3
        assert_eq!(3, disk_amount);
    }

    #[test]
    fn should_get_disk_amount_from_command_parameter() {
        // GIVEN a command line parameter
        let args = vec!["3".to_string()];
        let mut cursor = Cursor::new("3\n");

        // WHEN we get the disk amount
        let disk_amount = get_disk_amount(&args, &mut cursor);

        // THEN the disk amount should be 3
        assert_eq!(3, disk_amount);
    }

    #[test]
    fn should_get_disk_amount_from_user_input_with_error() {
        // GIVEN a command line parameter
        let args = vec![];
        let mut cursor = Cursor::new("3az\n3\n");

        // WHEN we get the disk amount
        let disk_amount = get_disk_amount(&args, &mut cursor);

        // THEN the disk amount should be 3
        assert_eq!(3, disk_amount);
    }
}
