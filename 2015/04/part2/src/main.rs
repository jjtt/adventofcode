fn main() {
    let input = "iwrupvqb";

    let mut num = 346386;

    loop {
        num += 1;
        let adventcoin = md5::compute(format!("{}{}", input, num));
        if adventcoin[0..3] == [0; 3] {
            println!("{}: {:x}", num, adventcoin);
            break;
        }
    }
}
