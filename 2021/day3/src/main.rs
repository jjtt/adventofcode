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

fn count_bitwise_ones(nums: &Vec<isize>, bits: usize) -> Vec<i32> {
    let mut counts = vec![0; bits];

    for i in 0..bits {
        let mut value = 0;
        for (j, num) in nums.iter().enumerate() {
            value = value | ((num & (1 << i)) << j);
        }
        counts[bits - i - 1] = value.count_ones() as i32;
    }

    counts
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn count() {
        assert_eq!([0], count_bitwise_ones(&vec![0], 1)[..]);
        assert_eq!([1], count_bitwise_ones(&vec![1], 1)[..]);
        assert_eq!([2], count_bitwise_ones(&vec![1, 1], 1)[..]);
        assert_eq!([2, 1], count_bitwise_ones(&vec![2, 3], 2)[..]);
    }

    #[test_case("sample1.txt", 5 => is eq(198) ; "sample")]
    #[test_case("input.txt", 12 => is eq(3309596) ; "input")]
    fn part1(input: &str, bits: usize) -> u32 {
        const SIZE: usize = 2; //i8::BITS as usize;
        let mut input_count = 0;
        let mut sums = vec![0; bits];
        let mut nums = Vec::with_capacity(SIZE);
        for line in read_lines(input).unwrap() {
            let cur = line.unwrap();
            let int = isize::from_str_radix(&cur, 2).unwrap();
            nums.push(int);
            if nums.len() % SIZE == 0 {
                let counts = count_bitwise_ones(&nums, bits);
                sums.iter_mut().zip(counts).for_each(|(a, b)| *a = *a + b);
                nums.clear();
            }
            input_count += 1;
        }

        let mut g = 0;
        let mut e;
        for b in 0..bits {
            let over_half_ones = sums[bits - b - 1] > input_count / 2;
            g = g | (if over_half_ones { 1 } else { 0 } << b);
        }

        // flip bits to get epsilon
        e = !g;
        e = e << (i32::BITS as usize - bits);
        e = e as u32 >> (i32::BITS as usize - bits);

        g * e
    }
}
