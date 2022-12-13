use ndarray::Array2;
use pathfinding::prelude::bfs;
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
            start: (start / ncols, start % ncols),
            end: (end / ncols, end % ncols),
        }
    }

    fn with_start(&self, start: usize) -> Map {
        let shape = self.heights.shape();
        Map {
            heights: self.heights.clone(),
            start: (start / shape[1], start % shape[1]),
            end: self.end,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let map = Map::from(&read_to_string(input).unwrap());

    let result = bfs(&map.start, |p| map.successors(p), |p| *p == map.end);

    result.unwrap().len() - 1
}

pub fn part2(input: &str) -> usize {
    let map = Map::from(&read_to_string(input).unwrap());

    let starts = map
        .heights
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'a')
        .map(|(index, _)| index);

    let mut best = usize::MAX;
    for start in starts {
        let newmap = map.with_start(start);
        if let Some(result) = bfs(
            &newmap.start,
            |p| newmap.successors(p),
            |p| *p == newmap.end,
        ) {
            let len = result.len() - 1;
            if len < best {
                best = len;
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

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
    fn parsing2() {
        let expected = Map {
            heights: arr2(&[['a', 'a'], ['b', 'z']]),
            start: (0, 1),
            end: (1, 1),
        };
        let map = Map::from("aS\nbE");
        assert_eq!(expected, map);
    }

    #[test]
    fn parsing3() {
        let expected = Map {
            heights: arr2(&[['a', 'a', 'a'], ['b', 'z', 'a']]),
            start: (1, 2),
            end: (1, 1),
        };
        let map = Map::from("aaa\nbES");
        assert_eq!(expected, map);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(31, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(29, part2("sample.txt"));
    }
}
