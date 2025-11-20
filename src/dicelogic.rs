use crate::roller::roll_dice;
use crate::lexer;

pub fn process_input(input : String) -> String {
    let i : usize = 0;
 if input.chars().nth(i).unwrap() >= '1' && input.chars().nth(i).unwrap().to_digit(10).unwrap() <= 9 {
    return number_node(input, i).to_string();
 }
 if input.chars().nth(i).unwrap() == '(' {
    return number_node(input, i).to_string();
 }
 return "".to_string();
}

fn number_node(input : String, mut i: usize) -> u32 {
    let mut result: String = String::new();
    // Reading in any number expecting the format ###d### where # is a number
    while input.chars().nth(i).expect("Couldn't unwrap due to no character existing").is_digit(10) {
        result.push(input.chars().nth(i).unwrap());
        i +=1;
        if input.len() <= i {
            return result.parse::<u32>().unwrap();
        }
    }

    // Picking up Brackets at the moment assuming no stacked brackets
    if input.chars().nth(i).unwrap() == '(' {
        let temp_result;
        (temp_result, i) = brackets(input.clone(),i);
        result.push_str(&temp_result);
    }
    if i >= input.len() {
        return result.parse::<u32>().unwrap();
    }
    // Picking up the d in ###d###
    if input.chars().nth(i).unwrap() == 'd' {
        // sends results backs to dice node to work out what size dice is being rolled.
        i += 1;
        let dice_size = number_node(input[i..].to_string(), 0);
        return roll_dice(result.parse::<u32>().unwrap(), dice_size).iter().sum();
    }

    return result.parse::<u32>().unwrap();
}

fn brackets(input: String, mut i: usize) -> (String, usize) {
    let mut bracket_content: String = String::new();
    i += 1;
    while input.chars().nth(i).unwrap() != ')' {
        if input.chars().nth(i).unwrap() == '(' {
            let result;
            (result ,i) = brackets(input.clone() ,i);
            bracket_content.push_str(&result);
        }
        else {
            bracket_content.push(input.chars().nth(i).unwrap());
        }
        i += 1;
    }
    i+=1;
    return (number_node(bracket_content.clone(), 0).to_string(),i);
}

fn dice_node(input : String, i: usize) -> u32 {
    if input.chars().nth(i).expect("String is empty").is_digit(10) {
        return number_node(input, i);
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    //This is testing to get basic functionality working
    #[test]
    fn random_test() {
        let input = String::from("1d6");
        let expected = String::from("1");
        let result = process_input(input);
        assert_eq!(expected, result);
    }

    //Testing basic braket testing
    #[test]
    fn bracket_test() {
        let input = String::from("(1d1)");
        let expected = String::from("1");
        let result = process_input(input);
        assert_eq!(expected,result);
    }
}