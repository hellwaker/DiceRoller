pub fn roll_dice(num_dice : u32, dice_size : u32) -> Vec<u32> 
{
    use rand::{rng, Rng};
    let mut rng = rng();
    let mut return_value: Vec<u32> = Vec::new();
    for _index in 0..num_dice {
        return_value.push( rng.random_range(1..dice_size));
    }
    return_value
}