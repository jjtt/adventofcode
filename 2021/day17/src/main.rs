use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;


    #[test_case("sample1.txt" => is eq(0); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> u32 {
        let _ = read_to_string(input).unwrap().trim();

        // vx>0
        // horiz speed stop: vx-n+1/2=0 == n=vx+1/2
        // max height: vy-n+1/2=0 == n=vy+1/2
        // x=n*vx+n-n(n+1)/2
        // y=n*vy+n-n(n+1)/2

        0
    }
}
