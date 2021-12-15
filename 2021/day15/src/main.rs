use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Debug)]
struct Head {
    x: usize,
    y: usize,
    r: u32,
}

impl PartialOrd<Self> for Head {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.r.partial_cmp(&self.r)
    }
}

impl Ord for Head {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn game_from_input(input: &str) -> Vec<Vec<u32>> {
    let string = read_to_string(input).unwrap();
    let lines = string.lines();

    lines
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn bfs(mut heads: BinaryHeap<Head>, map: Vec<Vec<u32>>) -> u32 {
    let maxy = map.len() - 1;
    let maxx = map.first().unwrap().len() - 1;
    let mut mincosts = vec![vec![<u32>::MAX; maxx + 1]; maxy + 1];
    let mut paths = vec![];
    while paths.is_empty() {
        let head = heads.pop().unwrap();
        if head.x == maxx && head.y == maxy {
            paths.push(head.r);
        } else {
            for (x2, y2) in neighbours(head.x, head.y, maxx, maxy) {
                let r2 = *map.get(y2).unwrap().get(x2).unwrap();

                if head.r + r2 < *mincosts.get(y2).unwrap().get(x2).unwrap() {
                    *mincosts.get_mut(y2).unwrap().get_mut(x2).unwrap() = head.r + r2;
                    heads.push(Head {
                        x: x2,
                        y: y2,
                        r: head.r + r2,
                    });
                }
            }
        }
    }
    *paths.iter().min().unwrap()
}

fn neighbours(x: usize, y: usize, maxx: usize, maxy: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x < maxx {
        neighbours.push((x + 1, y));
    }
    if y < maxy {
        neighbours.push((x, y + 1));
    }
    neighbours
}

#[cfg(test)]
mod test {
    use std::collections::BinaryHeap;
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(40); "sample1")]
    #[test_case("input.txt" => is eq(537); "input")]
    fn part1(input: &str) -> u32 {
        let game = game_from_input(input);

        let mut heads = BinaryHeap::new();
        heads.push(Head { x: 0, y: 0, r: 0 });
        bfs(heads, game)
    }
}
