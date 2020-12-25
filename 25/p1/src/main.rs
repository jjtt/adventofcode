fn main() {
    println!("Day 25 part 1");
}

fn transform(sub:i64, loopsize:i64) -> i64
{
    let mut val = 1;
    for _ in 0..loopsize
    {
        val *= sub;
        val %= 20201227
    }
    val
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_card_public() {
        assert_eq!(5764801, transform(7,8));
    }
    #[test]
    fn sample_door_public() {
        assert_eq!(17807724, transform(7,11));
    }
}
