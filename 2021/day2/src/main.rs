use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(150) ; "sample")]
    #[test_case("input1.txt" => is eq(1855814) ; "input")]
    fn part1(input: &str) -> i32 {
        let mut h = 0;
        let mut d = 0;
        for line in read_lines(input).unwrap() {
            let cur = line.unwrap();
            let mut split = cur.split(" ");
            let movement = split.next().unwrap();
            let amount = split.next().unwrap().parse::<i32>().unwrap();
            //println!("{}, {}", movement, amount);
            match movement {
                "forward" => h = h + amount,
                "down" => d = d + amount,
                "up" => d = d - amount,
                _ => panic!(),
            }
        }
        return h * d;
    }

}
