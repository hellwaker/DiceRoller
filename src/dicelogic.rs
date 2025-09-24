use crate::roller::roll_dice;

pub fn process_input(input : String) -> String {
    let i : usize = 0;
 if input.chars().nth(i).unwrap() >= '1' && input.chars().nth(i).unwrap().to_digit(10).unwrap() <= 9 {
    return number_node(input, i).to_string();
 }
 return "".to_string();
}

fn number_node(input : String, mut i: usize) -> u32 {
    let mut result: String = String::new();
    while input.chars().nth(i).expect("Couldn't unwrap due to no character existing").is_digit(10) {
        result.push(input.chars().nth(i).unwrap());
        i +=1;
        if ( input.len() <= i )
        {
            return result.parse::<u32>().unwrap();
        }
    }
    if input.chars().nth(i).unwrap() == 'd' {
        // sends results backs to dice node to work out what size dice is being rolled.
        i += 1;
        let dice_size = number_node(input[i..].to_string(), 0);
        return roll_dice(result.parse::<u32>().unwrap(), dice_size).iter().sum();
    }
    return result.parse::<u32>().unwrap();
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
    fn test() {
        let input = String::from("1d6");
        let expected = String::from("1");
        let result = process_input(input);
        assert_eq!(expected, result);
    }
}