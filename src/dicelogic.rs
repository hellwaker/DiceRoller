use crate::roller::roll_dice;

pub fn process_input(input : String) -> String {
    let i : usize = 0;
 if input.chars().nth(i).unwrap() >= '1' && input.chars().nth(i).unwrap().to_digit(10).unwrap() <= 9 {
    return number_node(input).to_string();
 }
 return "".to_string();
}

fn number_node(input : String) -> u32 {
    let mut i : usize = 0;
    let mut result: Vec<u32> = Vec::new();
    while input.chars().nth(i).expect("Couldn't unwrap due to no character existing").is_digit(10) {
        // Grabs the number
        result.push(input.chars().nth(i).unwrap().to_digit(10).unwrap());
        i +=1;
    }
    if input.chars().nth(i).unwrap() == 'd' {
        return roll_dice(result.iter().sum(), dice_node(input[i..input.len()].to_string())).iter().sum();
    }
    return 0;
}

fn dice_node(input : String) -> u32 {
    if input.chars().nth(0).expect("String is empty").is_digit(10) {
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
        let expected = String::from("1");
        let result = process_input(input);
        assert_eq!(expected, result);
    }
}