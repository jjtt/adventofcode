fn main() {
    println!("Day 25 part 1: {}", enc(14788856, 19316454));
}

fn transform(mut val:i64, sub:i64, loopsize:i64) -> i64
{
    for _ in 0..loopsize
    {
        val *= sub;
        val %= 20201227
    }
    val
}

fn crack(sub:i64, public:i64) -> i64
{
    let mut cand = 1;
    for i in 1..
    {
        cand = transform(cand, sub, 1);
        if cand == public
        {
            return i
        }
    };
    -1 // Should never reach this
}

fn enc(c:i64, d:i64) -> i64
{
    transform(1, c, crack(7, d))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_card_public() {
        assert_eq!(5764801, transform(1,7,8));
    }
    #[test]
    fn sample_door_public() {
        assert_eq!(17807724, transform(1,7,11));
    }
    #[test]
    fn sample_card_loop() {
        assert_eq!(8, crack(7,5764801));
    }
    #[test]
    fn sample_door_loop() {
        assert_eq!(11, crack(7,17807724));
    }
    #[test]
    fn sample_card_enc() {
        assert_eq!(14897079, transform(1,17807724,8));
    }
    #[test]
    fn sample_door_enc() {
        assert_eq!(14897079, transform(1,5764801,11));
    }
    #[test]
    fn sample_enc() {
        assert_eq!(14897079, enc(5764801, 17807724));
        assert_eq!(14897079, enc(17807724, 5764801));
    }
}
