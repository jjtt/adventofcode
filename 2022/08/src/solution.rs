use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    //todo!()
    0
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{arr1, arr2, s};

    #[test]
    fn array_slicing() {
        let trees = arr2(&[[1, 1, 1], [1, 2, 1], [1, 1, 1]]);
        assert_eq!(&[3, 3], trees.shape());

        let row = trees.slice(s![0..1, ..]);
        assert_eq!(arr2(&[[1, 1, 1]]), row);

        let row = trees.slice(s![1..2, ..]);
        assert_eq!(arr2(&[[1, 2, 1]]), row);

        let row = trees.slice(s![2..3, ..]);
        assert_eq!(arr2(&[[1, 1, 1]]), row);

        let col = trees.slice(s![.., 0..1]);
        assert_eq!(arr2(&[[1], [1], [1]]), col);

        let col = trees.slice(s![.., 1..2]);
        assert_eq!(arr2(&[[1], [2], [1]]), col);

        let col = trees.slice(s![.., 2..3]);
        assert_eq!(arr2(&[[1], [1], [1]]), col);

        let row = trees.slice(s![1, ..]);
        assert_eq!(arr1(&[1, 2, 1]), row);
        assert_eq!(arr1(&[1]), row.slice(s![..1]));
        assert_eq!(arr1(&[2]), row.slice(s![1..2]));
        assert_eq!(arr1(&[1]), row.slice(s![2..]));

        let col = trees.slice(s![.., 1]);
        assert_eq!(arr1(&[1, 2, 1]), col);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(21, part1("sample.txt"));
    }
}
