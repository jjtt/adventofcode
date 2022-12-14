use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

type Pos = (i64, i64);
type Rock = (RangeInclusive<i64>, RangeInclusive<i64>);

#[derive(Debug)]
struct SandPile {
    rocks: Vec<Rock>,
    pile: HashSet<Pos>,
    maxx: i64,
    maxy: i64,
    with_floor: bool,
}

impl SandPile {
    fn new(rocks: Vec<Rock>, with_floor: bool) -> SandPile {
        let maxx = *rocks
            .iter()
            .map(|(range, _)| range.end())
            .max()
            .unwrap_or(&-1);
        let maxy = *rocks
            .iter()
            .map(|(_, range)| range.end())
            .max()
            .unwrap_or(&-1);
        SandPile {
            rocks,
            pile: HashSet::new(),
            maxx,
            maxy,
            with_floor,
        }
    }

    fn drop(&self, pos: Pos) -> Option<Pos> {
        if !self.with_floor && (pos.0 < 0 || pos.0 > self.maxx || pos.1 < 0 || pos.1 > self.maxy) {
            None
        } else if self.free((pos.0, pos.1 + 1)) {
            self.drop((pos.0, pos.1 + 1))
        } else if self.free((pos.0 - 1, pos.1 + 1)) {
            self.drop((pos.0 - 1, pos.1 + 1))
        } else if self.free((pos.0 + 1, pos.1 + 1)) {
            self.drop((pos.0 + 1, pos.1 + 1))
        } else if (500, 0) == pos {
            None
        } else {
            Some(pos)
        }
    }

    fn free(&self, pos: Pos) -> bool {
        !(self.pile.contains(&pos) || self.with_floor && pos.1 >= self.maxy + 2)
            && self
                .rocks
                .iter()
                .all(|rock| !rock.0.contains(&pos.0) || !rock.1.contains(&pos.1))
    }
}

pub fn part1(input: &str) -> usize {
    let rocks = parse_input(input);
    let mut sand_pile = SandPile::new(rocks, false);

    while let Some(resting_place) = sand_pile.drop((500, 0)) {
        sand_pile.pile.insert(resting_place);
    }

    sand_pile.pile.len()
}

pub fn part2(input: &str) -> usize {
    let rocks = parse_input(input);
    let mut sand_pile = SandPile::new(rocks, true);

    while let Some(resting_place) = sand_pile.drop((500, 0)) {
        sand_pile.pile.insert(resting_place);
    }

    sand_pile.pile.len() + 1 // the cherry on top also
}

fn parse_input(input: &str) -> Vec<Rock> {
    read_to_string(input)
        .unwrap()
        .lines()
        .flat_map(parse)
        .collect()
}

fn parse(input: &str) -> Vec<Rock> {
    input
        .split(" -> ")
        .map(|s| scan_fmt!(s, "{d},{d}", i64, i64).unwrap())
        .collect::<Vec<Pos>>()
        .windows(2)
        .map(|w| (range(w[0], w[1])))
        .collect()
}

fn range(start: Pos, end: Pos) -> Rock {
    (
        start.0.min(end.0)..=start.0.max(end.0),
        start.1.min(end.1)..=start.1.max(end.1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_parsing() {
        assert_eq!(vec![(0..=0, 0..=1)], parse("0,0 -> 0,1"));
        assert_eq!(vec![(0..=2, 0..=0)], parse("2,0 -> 0,0"));
        assert_eq!(
            vec![(0..=2, 0..=0), (0..=0, 0..=10)],
            parse("2,0 -> 0,0 -> 0,10")
        );
    }

    #[test]
    fn dropping_to_void() {
        let sand_pile = SandPile::new(vec![], false);
        assert!(sand_pile.drop((0, 0)).is_none());
    }

    #[test]
    fn dropping_on_rock() {
        let sand_pile = SandPile::new(vec![(0..=2, 1..=1)], false);
        assert_eq!((1, 0), sand_pile.drop((1, 0)).unwrap());
    }

    #[test]
    fn dropping_on_rock_from_higher() {
        let sand_pile = SandPile::new(vec![(0..=2, 2..=2)], false);
        assert_eq!((1, 1), sand_pile.drop((1, 0)).unwrap());
    }

    #[test]
    fn dropping_on_rock_and_sand() {
        let mut sand_pile = SandPile::new(vec![(0..=4, 2..=2)], false);
        sand_pile.pile.insert((2, 1));
        assert_eq!((1, 1), sand_pile.drop((2, 0)).unwrap());
        sand_pile.pile.insert((1, 1));
        assert_eq!((3, 1), sand_pile.drop((2, 0)).unwrap());
    }

    #[test]
    fn part1_sample() {
        assert_eq!(24, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(93, part2("sample.txt"));
    }
}
