use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Coords {
    key: isize,
    values: Vec<i16>,
    keyed: Vec<i16>,
    predecessors: Vec<usize>,
    successors: Vec<usize>,
}

impl Coords {
    fn mix(&mut self, index: usize) {
        if self.keyed[index] == 0 {
            return;
        }

        self.successors[self.predecessors[index]] = self.successors[index];
        self.predecessors[self.successors[index]] = self.predecessors[index];
        let steps = self.keyed[index].rem_euclid(self.keyed.len() as i16 - 1);
        let mut s = self.successors[index];
        for _ in 0..(steps - 1) {
            s = self.successors[s];
        }
        self.successors[index] = self.successors[s];
        self.predecessors[index] = s;
        self.predecessors[self.successors[s]] = index;
        self.successors[s] = index;
    }

    fn mix_completely(&mut self) {
        for i in 0..self.values.len() {
            self.mix(i);
        }
    }

    fn zero(&self) -> usize {
        for i in 0..self.values.len() {
            if self.values[i] == 0 {
                return i;
            }
        }
        panic!("No zero?")
    }

    fn nth(&self, start: usize, n: usize) -> i16 {
        let mut i = start;
        for _ in 0..n.rem_euclid(self.values.len()) {
            i = self.successors[i];
        }
        self.values[i]
    }

    fn grove(&self) -> isize {
        let zero = self.zero();

        self.nth(zero, 1000) as isize * self.key
            + self.nth(zero, 2000) as isize * self.key
            + self.nth(zero, 3000) as isize * self.key
    }

    fn apply_key(&mut self, key: isize) {
        self.key = key;
        self.keyed = self
            .values
            .iter()
            .map(|v| (*v as isize * key).rem_euclid(self.values.len() as isize - 1) as i16)
            .collect();
    }

    #[allow(dead_code)]
    fn as_vec(&self) -> Vec<i16> {
        let mut v = Vec::with_capacity(self.values.capacity());

        let mut i = 0;
        while v.len() < self.values.len() {
            v.push(self.values[i]);
            i = self.successors[i];
        }

        v
    }
}

impl FromStr for Coords {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i16> = s
            .lines()
            .map(i16::from_str)
            .filter_map(Result::ok)
            .collect();
        let count = values.len();
        let predecessors = (0..count).map(|v| (v + count - 1) % count).collect();
        let successors = (0..count).map(|v| (v + 1) % count).collect();

        let keyed = values.clone();

        Ok(Coords {
            key: 1,
            values,
            keyed,
            predecessors,
            successors,
        })
    }
}

pub fn part1(input: &str) -> isize {
    let mut coords =
        Coords::from_str(&read_to_string(input).expect("coordinates")).expect("valid coords");

    coords.apply_key(1);

    coords.mix_completely();

    coords.grove()
}

pub fn part2(input: &str) -> isize {
    let mut coords =
        Coords::from_str(&read_to_string(input).expect("coordinates")).expect("valid coords");

    coords.apply_key(811589153);

    for _ in 0..10 {
        coords.mix_completely();
    }

    coords.grove()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let coords = Coords::from_str(&read_to_string("sample.txt").expect("a sample"))
            .expect("valid coords");
        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], coords.values);
        assert_eq!(vec![6, 0, 1, 2, 3, 4, 5], coords.predecessors);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 0], coords.successors);
        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], coords.as_vec());
    }

    #[test]
    fn mixing() {
        let mut coords = Coords::from_str(&read_to_string("sample.txt").expect("a sample"))
            .expect("valid coords");
        coords.mix(0);
        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], coords.values);
        assert_eq!(vec![1, 6, 0, 2, 3, 4, 5], coords.predecessors);
        assert_eq!(vec![2, 0, 3, 4, 5, 6, 1], coords.successors);
        assert_eq!(vec![1, -3, 3, -2, 0, 4, 2], coords.as_vec());

        coords.mix(5);
        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], coords.values);
        assert_eq!(vec![1, 6, 0, 2, 3, 4, 5], coords.predecessors);
        assert_eq!(vec![2, 0, 3, 4, 5, 6, 1], coords.successors);
        assert_eq!(vec![1, -3, 3, -2, 0, 4, 2], coords.as_vec());

        coords.mix(2);
        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], coords.values);
        assert_eq!(vec![1, 3, -2, 0, -3, 4, 2], coords.as_vec());
    }

    #[test]
    fn find_nth() {
        let coords = Coords::from_str(&read_to_string("sample.txt").expect("a sample"))
            .expect("valid coords");
        assert_eq!(2, coords.nth(0, 1));
        assert_eq!(-3, coords.nth(0, 2));
    }

    #[test]
    fn find_zero() {
        let coords = Coords::from_str(&read_to_string("sample.txt").expect("a sample"))
            .expect("valid coords");
        assert_eq!(5, coords.zero());
    }

    #[test]
    fn part1_sample() {
        assert_eq!(3, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(1623178306, part2("sample.txt"));
    }
}
