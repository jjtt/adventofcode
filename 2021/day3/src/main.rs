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

fn count_bitwise_ones<'a, I>(nums: I, bits: usize) -> Vec<i32>
where I: Iterator<Item=&'a isize>{
    let mut counts = vec![0; bits];

    let n: Vec<isize> = nums.cloned().collect();

    for i in 0..bits {
        let mut value = 0;
        for (j, num) in n.iter().enumerate() {
            value = value | (((num & (1 << i)) as usize >> i) << j);
        }
        counts[bits - i - 1] = value.count_ones() as i32;
    }

    counts
}

fn increment_one_counts(bits: usize, sums: &mut Vec<i32>, nums: &mut Vec<isize>) {
    let counts = count_bitwise_ones(nums.iter(), bits);
    sums.iter_mut().zip(counts).for_each(|(a, b)| *a = *a + b);
    nums.clear();
}

#[cfg(test)]
mod test {
    use std::collections::LinkedList;
    use std::usize;
    use test_case::test_case;

    use super::*;

    #[test]
    fn count() {
        assert_eq!([0], count_bitwise_ones((vec![0]).iter(), 1)[..]);
        assert_eq!([1], count_bitwise_ones((vec![1]).iter(), 1)[..]);
        assert_eq!([2], count_bitwise_ones((vec![1, 1]).iter(), 1)[..]);
        assert_eq!([2, 1], count_bitwise_ones((vec![2, 3]).iter(), 2)[..]);
        assert_eq!([64], count_bitwise_ones([1; 64].to_vec().iter(), 1)[..]);
    }

    #[test_case("sample1.txt", 5 => is eq(198) ; "sample")]
    #[test_case("input.txt", 12 => is eq(3309596) ; "input")]
    #[test_case("test1.txt", 5 => is eq(198) ; "test")]
    fn part1(input: &str, bits: usize) -> u32 {
        const SIZE: usize = usize::BITS as usize;
        let mut input_count = 0;
        let mut sums = vec![0; bits];
        let mut nums = Vec::with_capacity(SIZE);
        for line in read_lines(input).unwrap() {
            let cur = line.unwrap();
            let int = isize::from_str_radix(&cur, 2).unwrap();
            nums.push(int);
            if nums.len() % SIZE == 0 {
                increment_one_counts(bits, &mut sums, &mut nums);
            }
            input_count += 1;
        }
        increment_one_counts(bits, &mut sums, &mut nums);

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

    #[test_case("sample1.txt", 5 => is eq(230) ; "sample")]
    #[test_case("input.txt", 12 => is eq(0) ; "input")]
    fn part2(input: &str, bits: usize) -> u32 {
        let mut candidates_o = vec![];
        let mut candidates_c = vec![];
        for line in read_lines(input).unwrap() {
            let cur = line.unwrap();
            let int = isize::from_str_radix(&cur, 2).unwrap();
            candidates_o.push(int.clone());
            candidates_c.push(int);
        }

        let input_count = candidates_o.len();

        for i in 0..bits {
            let counts = count_bitwise_ones(candidates_o.iter(), bits);
            dbg!(&counts);

            if counts[i] >= (candidates_o.len() as i32 - counts[i]) {
                candidates_o.retain(|v| (*v & (1 << (bits - i - 1))) > 0);
            } else {
                candidates_o.retain(|v| (*v & (1 << (bits - i - 1))) == 0);
            }

            dbg!(&candidates_o);

            if candidates_o.len() <= 1 {
                break;
            }
        }

        for i in 0..bits {
            let counts = count_bitwise_ones(candidates_c.iter(), bits);
            dbg!(&counts);

            if counts[i] < (candidates_c.len() as i32 - counts[i]) {
                candidates_c.retain(|v| (*v & (1 << (bits - i - 1))) > 0);
            } else {
                candidates_c.retain(|v| (*v & (1 << (bits - i - 1))) == 0);
            }

            dbg!(&candidates_c);

            if candidates_c.len() <= 1 {
                break;
            }
        }

        let oxygen = candidates_o[0] as u32;
        let co2 = candidates_c[0] as u32;

        oxygen * co2
    }
}
