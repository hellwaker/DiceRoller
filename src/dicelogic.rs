

pub fn process_input(String input) -> String {
 if input[0] == 0..9 {
    return number_node(input);
 }
}

fn number_node(String input) -> u32 {
    let i = 0;
    let result: String = String::new();
    while input[i] == 0..9 {
        // Grabs the number
        result.push(input[i]);
        i++;
    }
    else if input[i] == "d" {
        return roll_dice(result, dice_node(input[i..input.length()]))
    }
}

fn dice_node(String input) -> u32 {
    if input[0] == 0..9 {
        return number_node(input);
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    //This is testing to get basic functionality working
    #[test]
    fn test() {
        let input = String::from("1d6");
        let expected = 1;
        let result = dice_node(input);
        assert_eq!(expected, result.length());
    }
}