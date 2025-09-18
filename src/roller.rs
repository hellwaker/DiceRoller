pub fn roll_dice(num_dice : u32, mut dice_size : u32) -> Vec<u32> 
{
    use rand::{rng, Rng};
    let mut rng = rng();
    let mut return_value: Vec<u32> = Vec::new();
    dice_size +=1;
    for _index in 0..num_dice {
        return_value.push( rng.random_range(1..dice_size));
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;
    
    //Basic testing seeing if we can force the result to be 1
    #[test]
    fn roll() {
        let result = roll_dice(1, 1);
        let expected = vec![1];
        println!("{:?}", result);
        println!("{:?}", expected);
        assert_eq!(result,expected);
    }
}