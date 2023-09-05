use phf::phf_map;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

static NETS: [phf::Map<u8, (u8, Facing)>; 2] = [
    // sample cube
    phf_map! {
        13u8 => (1,Facing::Up),
        21u8 => (2,Facing::Up),
        22u8 => (3,Facing::Up),
        23u8 => (4,Facing::Up),
        33u8 => (5,Facing::Up),
        34u8 => (6,Facing::Up),
    },
    // input cube
    phf_map! {
        12u8 => (1,Facing::Up),
        13u8 => (6,Facing::Down),
        22u8 => (4,Facing::Up),
        31u8 => (3,Facing::Left),
        32u8 => (5,Facing::Up),
        41u8 => (2,Facing::Left),
    },
    // add new nets here
    // ...
];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Pos {
    row: usize,
    col: usize,
    facing: Facing,
}

impl Pos {
    pub(crate) fn turn_right(&mut self) {
        match self.facing {
            Facing::Right => self.facing = Facing::Down,
            Facing::Down => self.facing = Facing::Left,
            Facing::Left => self.facing = Facing::Up,
            Facing::Up => self.facing = Facing::Right,
        }
    }
    pub(crate) fn turn_left(&mut self) {
        match self.facing {
            Facing::Right => self.facing = Facing::Up,
            Facing::Down => self.facing = Facing::Right,
            Facing::Left => self.facing = Facing::Down,
            Facing::Up => self.facing = Facing::Left,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Action {
    Move(u32),
    Left,
    Right,
}

impl Action {
    pub(crate) fn parse_actions(input: &str) -> Vec<Action> {
        let mut actions = vec![];
        let mut num = 0;
        for c in input.chars() {
            match c {
                'R' => {
                    actions.push(Action::Move(num));
                    num = 0;
                    actions.push(Action::Right);
                }
                'L' => {
                    actions.push(Action::Move(num));
                    num = 0;
                    actions.push(Action::Left);
                }
                c => {
                    num *= 10;
                    num += c.to_digit(10).expect("a number")
                }
            }
        }
        actions
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<(usize, usize), bool>,
    rows: usize,
    cols: usize,
    net: &'static phf::Map<u8, (u8, Facing)>,
    cube_face_size: usize,
    tile_faces: HashMap<(usize, usize), (u8, Facing)>,
}

impl Map {
    fn tiles_to_faces(
        tiles: &HashMap<(usize, usize), bool>,
        cube_face_size: usize,
        net: &'static phf::Map<u8, (u8, Facing)>,
    ) -> HashMap<(usize, usize), (u8, Facing)> {
        tiles
            .iter()
            .map(|((row, col), _)| {
                (
                    (*row, *col),
                    *net.get(
                        &((((row - 1) / cube_face_size + 1) * 10 + ((col - 1) / cube_face_size + 1))
                            as u8),
                    )
                    .expect("Tile must map to the net"),
                )
            })
            .collect()
    }

    pub(crate) fn identify(&self) -> &phf::Map<u8, (u8, Facing)> {
        self.net
    }

    pub(crate) fn identify_tiles(
        tiles: &HashMap<(usize, usize), bool>,
        face_size: usize,
    ) -> &'static phf::Map<u8, (u8, Facing)> {
        let mut net_points = HashSet::new();
        for row in 0..4 {
            for col in 0..4 {
                if tiles.contains_key(&(1 + row * face_size, 1 + col * face_size)) {
                    net_points.insert(((row + 1) * 10 + col + 1) as u8);
                }
            }
        }

        for net in &NETS {
            if net_points.iter().all(|point| net.contains_key(point)) {
                return net;
            }
        }
        todo!("Net not found in NETS")
    }

    pub(crate) fn find_start(&self) -> Pos {
        for col in 1..=self.cols {
            if let Some(true) = self.tiles.get(&(1, col)) {
                return Pos {
                    row: 1,
                    col,
                    facing: Facing::Right,
                };
            }
        }
        panic!("unable to find a start")
    }

    fn parse_map(input: &str) -> (Map, Vec<Action>) {
        let (map, actions) = input
            .split_once("\n\n")
            .expect("One empty line to separate map from actions");
        let tiles: HashMap<(usize, usize), bool> = map
            .lines()
            .enumerate()
            .flat_map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_index, c)| (row_index + 1, col_index + 1, c))
                    .filter_map(|(row, col, c)| match c {
                        ' ' => None,
                        '.' => Some(((row, col), true)),
                        '#' => Some(((row, col), false)),
                        _ => panic!("{} not supported", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let rows = *tiles.keys().map(|(row, _)| row).max().expect("a number");
        let cols = *tiles.keys().map(|(_, col)| col).max().expect("a number");
        let cube_face_size = Map::cube_face_size(rows, cols);
        let net = Map::identify_tiles(&tiles, cube_face_size);
        let tile_faces: HashMap<(usize, usize), (u8, Facing)> =
            Map::tiles_to_faces(&tiles, cube_face_size, net);
        (
            Map {
                tiles,
                rows,
                cols,
                net,
                cube_face_size,
                tile_faces,
            },
            Action::parse_actions(actions.trim()),
        )
    }

    fn cube_face_size(rows: usize, cols: usize) -> usize {
        let bigger = rows.max(cols);
        let smaller = rows.min(cols);

        if bigger % 5 == 0 && smaller % 2 == 0 && bigger / 5 == smaller / 2 {
            bigger / 5
        } else if bigger % 4 == 0 && smaller % 3 == 0 && bigger / 4 == smaller / 3 {
            bigger / 4
        } else {
            panic!("Not a cube net?")
        }
    }

    fn step(&self, pos: &Pos, cube: bool) -> (Pos, bool) {
        if cube {
            self.step_cube(pos.clone())
        } else {
            self.step_map(pos.clone())
        }
    }

    fn step_map(&self, mut pos: Pos) -> (Pos, bool) {
        loop {
            match pos.facing {
                Facing::Right => pos.col = (pos.col % self.cols) + 1,
                Facing::Down => pos.row = (pos.row % self.rows) + 1,
                Facing::Left => pos.col = ((pos.col + self.cols - 2) % self.cols) + 1,
                Facing::Up => pos.row = ((pos.row + self.rows - 2) % self.rows) + 1,
            };
            if let Some(&tile) = self.tiles.get(&(pos.row, pos.col)) {
                return (pos, tile);
            }
        }
    }

    fn step_cube(&self, mut pos: Pos) -> (Pos, bool) {
        todo!()
    }
}

fn walk(input: &str, cube: bool) -> usize {
    let input = read_to_string(input).expect("a file");
    let (map, actions) = Map::parse_map(&input);

    let mut pos = map.find_start();

    for a in actions {
        match a {
            Action::Move(count) => {
                for _ in 0..count {
                    if let (new_pos, true) = map.step(&pos, cube) {
                        pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
            Action::Left => pos.turn_left(),
            Action::Right => pos.turn_right(),
        }
    }

    1000 * pos.row + 4 * pos.col + pos.facing as usize
}

pub fn part1(input: &str) -> usize {
    walk(input, false)
}

pub fn part2(input: &str) -> usize {
    walk(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parsing_actions() {
        let actions = Action::parse_actions("10R5L");
        assert_eq!(
            vec![
                Action::Move(10),
                Action::Right,
                Action::Move(5),
                Action::Left
            ],
            actions
        );
    }

    #[test]
    fn parsing() {
        let input = indoc! {"
              . 
            ... 
              .#

            L1R
        "};

        let (map, actions) = Map::parse_map(input);

        assert_eq!(6, map.tiles.len());
        assert!(matches!(map.tiles.get(&(1, 3)), Some(true)));
        assert!(matches!(map.tiles.get(&(2, 1)), Some(true)));
        assert!(matches!(map.tiles.get(&(2, 2)), Some(true)));
        assert!(matches!(map.tiles.get(&(2, 3)), Some(true)));
        assert!(matches!(map.tiles.get(&(3, 3)), Some(true)));
        assert!(matches!(map.tiles.get(&(3, 4)), Some(false)));
        assert_eq!(3, map.rows);
        assert_eq!(4, map.cols);

        assert_eq!(
            vec![
                Action::Move(0),
                Action::Left,
                Action::Move(1),
                Action::Right
            ],
            actions
        )
    }

    #[test]
    fn stepping() {
        let input = indoc! {"
              . 
            ... 
              .#

            L1R
        "};

        let (map, _) = Map::parse_map(input);

        assert_eq!(
            (
                Pos {
                    row: 2,
                    col: 1,
                    facing: Facing::Right
                },
                true
            ),
            map.step_map(Pos {
                row: 2,
                col: 3,
                facing: Facing::Right
            })
        );
    }

    #[test]
    fn finding_the_start() {
        let input = indoc! {"
              . 
            ... 
              .#

            L1R
        "};

        let (map, _) = Map::parse_map(input);

        assert_eq!(
            Pos {
                row: 1,
                col: 3,
                facing: Facing::Right
            },
            map.find_start()
        );
    }

    #[test]
    fn cube_faces() {
        let input = indoc! {"
              . 
            ... 
              .#

            L1R
        "};

        let (map, _) = Map::parse_map(input);
        assert_eq!(1, map.cube_face_size);

        let (map, _) = Map::parse_map(&read_to_string("sample.txt").expect("sample"));
        assert_eq!(4, map.cube_face_size);

        let (map, _) = Map::parse_map(&read_to_string("input.txt").expect("sample"));
        assert_eq!(50, map.cube_face_size);
    }

    #[test]
    fn turning() {
        let mut pos = Pos {
            row: 1,
            col: 1,
            facing: Facing::Right,
        };
        pos.turn_right();
        assert_eq!(Facing::Down, pos.facing);
        pos.turn_right();
        assert_eq!(Facing::Left, pos.facing);
        pos.turn_right();
        assert_eq!(Facing::Up, pos.facing);
        pos.turn_right();
        assert_eq!(Facing::Right, pos.facing);
        pos.turn_left();
        assert_eq!(Facing::Up, pos.facing);
        pos.turn_left();
        assert_eq!(Facing::Left, pos.facing);
        pos.turn_left();
        assert_eq!(Facing::Down, pos.facing);
        pos.turn_left();
        assert_eq!(Facing::Right, pos.facing);
    }

    #[test]
    fn identifying_the_net() {
        let (map, _) = Map::parse_map(&read_to_string("sample.txt").expect("sample"));
        assert_eq!(format!("{:?}", NETS[0]), format!("{:?}", map.identify()));

        let (map, _) = Map::parse_map(&read_to_string("input.txt").expect("sample"));
        assert_eq!(format!("{:?}", NETS[1]), format!("{:?}", map.identify()));
    }

    #[test]
    fn facing_tiles_sample_net() {
        let input = indoc! {"
              . 
            ... 
              ..

            L1R
        "};

        let (map, _) = Map::parse_map(input);

        assert!(matches!(map.tile_faces.get(&(1, 3)), Some((1, Facing::Up))))
    }

    #[test]
    fn facing_tiles_input_net() {
        let input = indoc! {"
             ..
             . 
            .. 
            .  

            R
        "};

        let (map, _) = Map::parse_map(input);

        assert!(matches!(
            map.tile_faces.get(&(4, 1)),
            Some((2, Facing::Left))
        ))
    }

    #[test]
    fn facing_tiles_sample() {
        let (map, _) = Map::parse_map(&read_to_string("sample.txt").expect("sample"));
        assert!(matches!(
            map.tile_faces.get(&(12, 16)),
            Some((6, Facing::Up))
        ));
        assert!(matches!(
            map.tile_faces.get(&(12, 13)),
            Some((6, Facing::Up))
        ));
        assert!(matches!(
            map.tile_faces.get(&(12, 12)),
            Some((5, Facing::Up))
        ));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(6032, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(5031, part2("sample.txt"));
    }
}
