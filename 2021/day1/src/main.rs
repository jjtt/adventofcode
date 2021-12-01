use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(7) ; "sample")]
    #[test_case("input1.txt" => is eq(1655) ; "input")]
    fn part1(input: &str) -> i32 {
        let mut result = 0;
        let mut prev = i32::MAX;
        for line in read_lines(input).unwrap() {
            //println!("{}", line.unwrap());
            let cur = line.unwrap().parse::<i32>().unwrap();
            if cur > prev {
                result = result + 1;
            }
            prev = cur;
        }
        return result;
    }
}
