use itertools::Itertools;
use ndarray::{s, Array2};
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    count_visible(&parse(&read_to_string(input).unwrap()))
}

pub fn part2(input: &str) -> usize {
    let trees = parse(&read_to_string(input).unwrap());
    let rows = trees.shape()[0];
    let columns = trees.shape()[1];

    (0..rows)
        .cartesian_product(0..columns)
        .map(|(r, c)| scenic_score(&trees, r, c))
        .max()
        .unwrap()
}

fn visible(trees: &Array2<u8>, row: usize, column: usize) -> bool {
    let current = trees[(row, column)];
    trees
        .row(row)
        .slice(s![..column])
        .iter()
        .all(|v| *v < current)
        || trees
            .row(row)
            .slice(s![(column + 1)..])
            .iter()
            .all(|v| *v < current)
        || trees
            .column(column)
            .slice(s![..row])
            .iter()
            .all(|v| *v < current)
        || trees
            .column(column)
            .slice(s![(row + 1)..])
            .iter()
            .all(|v| *v < current)
}

fn count_visible(trees: &Array2<u8>) -> usize {
    let rows = trees.shape()[0];
    let columns = trees.shape()[1];

    (0..rows)
        .cartesian_product(0..columns)
        .filter(|(r, c)| visible(trees, *r, *c))
        .count()
}

fn parse(input: &str) -> Array2<u8> {
    let lines = input.lines();
    let rows = lines.clone().count();
    let columns = lines.clone().next().unwrap().len();

    Array2::from_shape_vec((rows, columns), lines.flat_map(parse_row).collect()).unwrap()
}

fn parse_row(row: &str) -> Vec<u8> {
    row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn scenic_score(trees: &Array2<u8>, row: usize, column: usize) -> usize {
    let rows = trees.shape()[0];
    let columns = trees.shape()[1];

    let current = trees[(row, column)];
    let left = trees
        .row(row)
        .slice(s![..column])
        .iter()
        .rev()
        .enumerate()
        .find(|(_, v)| **v >= current)
        .map(|(index, _)| index + 1)
        .unwrap_or(column);
    let right = trees
        .row(row)
        .slice(s![(column + 1)..])
        .iter()
        .enumerate()
        .find(|(_, v)| **v >= current)
        .map(|(index, _)| index + 1)
        .unwrap_or(columns - column - 1);
    let up = trees
        .column(column)
        .slice(s![..row])
        .iter()
        .rev()
        .enumerate()
        .find(|(_, v)| **v >= current)
        .map(|(index, _)| index + 1)
        .unwrap_or(row);
    let down = trees
        .column(column)
        .slice(s![(row + 1)..])
        .iter()
        .enumerate()
        .find(|(_, v)| **v >= current)
        .map(|(index, _)| index + 1)
        .unwrap_or(rows - row - 1);
    left * right * up * down
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{arr1, arr2, s};

    #[test]
    fn array_slicing() {
        let trees = arr2(&[[1, 1, 1], [1, 2, 1], [1, 2, 3]]);
        assert_eq!(&[3, 3], trees.shape());

        let row = trees.slice(s![0..1, ..]);
        assert_eq!(arr2(&[[1, 1, 1]]), row);

        let row = trees.slice(s![1..2, ..]);
        assert_eq!(arr2(&[[1, 2, 1]]), row);

        let row = trees.slice(s![2..3, ..]);
        assert_eq!(arr2(&[[1, 2, 3]]), row);

        let col = trees.slice(s![.., 0..1]);
        assert_eq!(arr2(&[[1], [1], [1]]), col);

        let col = trees.slice(s![.., 1..2]);
        assert_eq!(arr2(&[[1], [2], [2]]), col);

        let col = trees.slice(s![.., 2..3]);
        assert_eq!(arr2(&[[1], [1], [3]]), col);

        let row = trees.slice(s![1, ..]);
        assert_eq!(arr1(&[1, 2, 1]), row);
        assert_eq!(arr1(&[1]), row.slice(s![..1]));
        assert_eq!(arr1(&[2]), row.slice(s![1..2]));
        assert_eq!(arr1(&[1]), row.slice(s![2..]));

        let col = trees.slice(s![.., 1]);
        assert_eq!(arr1(&[1, 2, 2]), col);
    }

    #[test]
    fn rows_and_columns() {
        let trees = arr2(&[[1, 1, 1], [1, 2, 1], [1, 2, 3]]);
        assert_eq!(&[3, 3], trees.shape());

        let row = trees.row(0);
        assert_eq!(arr1(&[1, 1, 1]), row);

        let row = trees.row(1);
        assert_eq!(arr1(&[1, 2, 1]), row);

        let row = trees.row(2);
        assert_eq!(arr1(&[1, 2, 3]), row);

        let row = trees.column(0);
        assert_eq!(arr1(&[1, 1, 1]), row);

        let row = trees.column(1);
        assert_eq!(arr1(&[1, 2, 2]), row);

        let row = trees.column(2);
        assert_eq!(arr1(&[1, 1, 3]), row);
    }

    #[test]
    fn all_visible() {
        let trees = arr2(&[[1, 1, 1], [1, 2, 1], [1, 2, 3]]);
        assert_eq!(&[3, 3], trees.shape());

        assert!(visible(&trees, 0, 0));
        assert!(visible(&trees, 0, 1));
        assert!(visible(&trees, 0, 2));
        assert!(visible(&trees, 1, 0));
        assert!(visible(&trees, 1, 1));
        assert!(visible(&trees, 1, 2));
        assert!(visible(&trees, 2, 0));
        assert!(visible(&trees, 2, 1));
        assert!(visible(&trees, 2, 2));
    }

    #[test]
    fn middle_hidden() {
        let trees = arr2(&[[1, 1, 1], [1, 0, 1], [1, 2, 3]]);
        assert_eq!(&[3, 3], trees.shape());

        assert!(visible(&trees, 0, 0));
        assert!(visible(&trees, 0, 1));
        assert!(visible(&trees, 0, 2));
        assert!(visible(&trees, 1, 0));
        assert!(!visible(&trees, 1, 1));
        assert!(visible(&trees, 1, 2));
        assert!(visible(&trees, 2, 0));
        assert!(visible(&trees, 2, 1));
        assert!(visible(&trees, 2, 2));
    }

    #[test]
    fn counting_visible() {
        let trees = arr2(&[[1, 1, 1], [1, 0, 1], [1, 2, 3]]);
        assert_eq!(8, count_visible(&trees));

        let trees = arr2(&[[1, 0, 1], [1, 1, 1], [1, 2, 3]]);
        assert_eq!(9, count_visible(&trees));
    }

    #[test]
    fn parsing_trees() {
        assert_eq!(arr2(&[[1]]), parse("1"));
        assert_eq!(arr2(&[[1, 2], [3, 4]]), parse("12\n34"));
        assert_eq!(
            arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            parse("123\n456\n789")
        );
    }

    #[test]
    fn scenic_scores() {
        let trees = parse(&read_to_string("sample.txt").unwrap());
        assert_eq!(4, scenic_score(&trees, 1, 2));
        assert_eq!(8, scenic_score(&trees, 3, 2));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(21, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(8, part2("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1816, part1("input.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(383520, part2("input.txt"));
    }
}
