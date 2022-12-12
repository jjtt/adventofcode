use anyhow::bail;
use ndarray::{arr2, Array2};
use pathfinding::prelude::bfs;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

#[derive(PartialEq, Debug)]
struct Map {
    heights: Array2<char>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn successors(&self, (row, col): &(usize, usize)) -> Vec<(usize, usize)> {
        let mut s = vec![];
        let nrows = self.heights.nrows();
        let ncols = self.heights.ncols();
        let cur_plus_one = char::from(self.heights[[*row, *col]] as u8 + 1);
        if *row > 0 && cur_plus_one >= self.heights[[row - 1, *col]] {
            s.push((*row - 1, *col))
        }
        if *col > 0 && cur_plus_one >= self.heights[[*row, *col - 1]] {
            s.push((*row, *col - 1))
        }
        if *row < nrows - 1 && cur_plus_one >= self.heights[[*row + 1, *col]] {
            s.push((*row + 1, *col))
        }
        if *col < ncols - 1 && cur_plus_one >= self.heights[[*row, *col + 1]] {
            s.push((*row, *col + 1))
        }

        s
    }

    fn from(input: &str) -> Map {
        let map: Vec<Vec<char>> = input.lines().map(str::chars).map(|c| c.collect()).collect();
        let nrows = map.len();
        let ncols = map.first().unwrap().len();

        let mut map: Vec<char> = map.into_iter().flatten().collect();
        let start = map.iter().position(|c| *c == 'S').unwrap();
        let end = map.iter().position(|c| *c == 'E').unwrap();

        map[start] = 'a';
        map[end] = 'z';

        Map {
            heights: Array2::from_shape_vec((nrows, ncols), map).unwrap(),
            start: (start / nrows, start % ncols),
            end: (end / ncols, end % ncols),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let map = Map::from(&read_to_string(input).unwrap());
    dbg!(&map);
    let result = bfs(&map.start, |p| map.successors(p), |p| *p == map.end);

    result.unwrap().len() - 1
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pathfinding::prelude::bfs;

    #[test]
    fn pathfinding() {
        let map = Map {
            heights: arr2(&[['b', 'c'], ['e', 'd']]),
            start: (0, 0),
            end: (1, 0),
        };

        let result = bfs(&map.start, |p| map.successors(p), |p| *p == map.end);

        assert_eq!(4, result.expect("to find a path").len());
    }

    #[test]
    fn parsing() {
        let expected = Map {
            heights: arr2(&[['z', 'b'], ['a', 'c']]),
            start: (1, 0),
            end: (0, 0),
        };
        let map = Map::from("Eb\nSc");
        assert_eq!(expected, map);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(31, part1("sample.txt"));
    }
}
