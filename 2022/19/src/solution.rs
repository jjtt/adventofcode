use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl FromStr for Resource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Self::Ore),
            "clay" => Ok(Self::Clay),
            "obsidian" => Ok(Self::Obsidian),
            "geode" => Ok(Self::Geode),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: [[usize; 4]; 4],
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, robots) = s.split_once(": Each ").expect("a blueprint");
        let id = scan_fmt!(id, "Blueprint {d}", usize).expect("usize blueprint id");
        let mut costs = [[0; 4]; 4];
        let robots = robots.split("Each ");
        for r in robots {
            let (robot_type, robot_costs) = r.split_once(" robot costs ").expect("a robot");
            let robot_type = Resource::from_str(robot_type).expect("a valid resource");
            costs[robot_type as usize] = Blueprint::split_costs(robot_costs);
        }
        Ok(Blueprint { id, costs })
    }
}

impl Blueprint {
    fn split_costs(costs: &str) -> [usize; 4] {
        let costs = costs.trim();
        assert!(costs.ends_with('.'));
        let costs = &costs[0..costs.len() - 1];
        let mut c = [0; 4];

        for t in costs.split(" and ") {
            let (count, resource_type) =
                scan_fmt!(t, "{d} {}", usize, String).expect("count and type");
            c[Resource::from_str(&resource_type).expect("valid type") as usize] = count;
        }

        c
    }
}

pub fn part1(input: &str) -> i32 {
    let blueprints: Vec<Blueprint> = read_to_string(input)
        .unwrap()
        .lines()
        .map(Blueprint::from_str)
        .filter_map(Result::ok)
        .collect();

    dbg!(blueprints);
    0
}

pub fn part2(input: &str) -> i32 {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_blueprint() {
        let bp = "Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 4 ore and 17 obsidian.";
        let bp = Blueprint::from_str(bp).expect("is parseable");
        assert_eq!(29, bp.id);
        assert_eq!(4, bp.costs[Resource::Ore as usize][Resource::Ore as usize]);
        assert_eq!(4, bp.costs[Resource::Clay as usize][Resource::Ore as usize]);
        assert_eq!(
            4,
            bp.costs[Resource::Obsidian as usize][Resource::Ore as usize]
        );
        assert_eq!(
            15,
            bp.costs[Resource::Obsidian as usize][Resource::Clay as usize]
        );
        assert_eq!(
            4,
            bp.costs[Resource::Geode as usize][Resource::Ore as usize]
        );
        assert_eq!(
            17,
            bp.costs[Resource::Geode as usize][Resource::Obsidian as usize]
        );
    }

    #[test]
    fn splitting_costs() {
        assert_eq!(
            [1, 1, 1, 1],
            Blueprint::split_costs("1 ore and 1 clay and 1 obsidian and 1 geode. ")
        );
        assert_eq!(
            [1, 2, 3, 0],
            Blueprint::split_costs("1 ore and 2 clay and 3 obsidian. ")
        );
        assert_eq!([0, 0, 0, 1], Blueprint::split_costs("1 geode. "));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(33, part1("sample.txt"));
    }
}
