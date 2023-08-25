use crate::solution::Resource::Ore;
use anyhow::bail;
use pathfinding::prelude::dfs_reach;
use pathfinding::utils::uint_sqrt;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;
use std::str::FromStr;
use Resource::{Clay, Geode, Obsidian};

#[derive(Debug, PartialEq)]
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
            "clay" => Ok(Clay),
            "obsidian" => Ok(Obsidian),
            "geode" => Ok(Geode),
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

    fn quality(self) -> usize {
        self.id * self.max_geodes(24)
    }

    fn max_geodes(&self, time_left: usize) -> usize {
        let robots = [1, 0, 0, 0];
        let resources = [0; 4];
        let result = dfs_reach(
            (time_left, robots, resources),
            |(time_left, robots, resources)| {
                dbg!(robots);
                let mut successors = vec![];
                if *time_left == 0 {
                    return successors;
                }

                let try_to_build = |which| -> Option<(usize, [usize; 4], [usize; 4])> {
                    let costs: [usize; 4] = self.costs[which];
                    if resources[Ore as usize] >= costs[Ore as usize]
                        && resources[Clay as usize] >= costs[Clay as usize]
                        && resources[Obsidian as usize] >= costs[Obsidian as usize]
                        && resources[Geode as usize] >= costs[Geode as usize]
                    {
                        Some((
                            time_left - 1,
                            [
                                robots[Ore as usize] + if which == Ore as usize { 1 } else { 0 },
                                robots[Clay as usize] + if which == Clay as usize { 1 } else { 0 },
                                robots[Obsidian as usize]
                                    + if which == Obsidian as usize { 1 } else { 0 },
                                robots[Geode as usize]
                                    + if which == Geode as usize { 1 } else { 0 },
                            ],
                            [
                                resources[Ore as usize] - costs[Ore as usize]
                                    + robots[Ore as usize],
                                resources[Clay as usize] - costs[Clay as usize]
                                    + robots[Clay as usize],
                                resources[Obsidian as usize] - costs[Obsidian as usize]
                                    + robots[Obsidian as usize],
                                resources[Geode as usize] - costs[Geode as usize]
                                    + robots[Geode as usize],
                            ],
                        ))
                    } else {
                        None
                    }
                };

                if let Some(s) = try_to_build(Geode as usize) {
                    successors.push(s);
                }
                if let Some(s) = try_to_build(Obsidian as usize) {
                    successors.push(s);
                }
                if let Some(s) = try_to_build(Clay as usize) {
                    successors.push(s);
                }
                if let Some(s) = try_to_build(Ore as usize) {
                    successors.push(s);
                }

                successors.push((
                    (time_left - 1),
                    *robots,
                    [
                        resources[Ore as usize] + robots[Ore as usize],
                        resources[Clay as usize] + robots[Clay as usize],
                        resources[Obsidian as usize] + robots[Obsidian as usize],
                        resources[Geode as usize] + robots[Geode as usize],
                    ],
                ));

                successors
            },
        );

        result
            .map(|(_time_left, _robots, resources)| resources[Geode as usize])
            .max()
            .expect("at least one result")
    }
}

pub fn part1(input: &str) -> usize {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(Blueprint::from_str)
        .filter_map(Result::ok)
        .map(Blueprint::quality)
        .sum()
}

pub fn part2(input: &str) -> usize {
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
        assert_eq!(4, bp.costs[Clay as usize][Resource::Ore as usize]);
        assert_eq!(4, bp.costs[Obsidian as usize][Resource::Ore as usize]);
        assert_eq!(15, bp.costs[Obsidian as usize][Clay as usize]);
        assert_eq!(4, bp.costs[Geode as usize][Resource::Ore as usize]);
        assert_eq!(17, bp.costs[Geode as usize][Obsidian as usize]);
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
    fn maximising_geodes_easy_blueprint() {
        let bp = "Blueprint 1: Each ore robot costs 100 ore.  Each clay robot costs 100 ore.  Each obsidian robot costs 100 ore.  Each geode robot costs 1 ore.";
        assert_eq!(
            1,
            Blueprint::from_str(bp)
                .expect("valid blueprint")
                .max_geodes(3)
        );
    }

    #[test]
    fn maximising_geodes() {
        let string = read_to_string("sample.txt").expect("sample file");
        let mut lines = string.lines();

        assert_eq!(
            9,
            Blueprint::from_str(lines.next().expect("1st blueprint"))
                .expect("valid blueprint")
                .max_geodes(24)
        );
        assert_eq!(
            12,
            Blueprint::from_str(lines.next().expect("2nd blueprint"))
                .expect("valid blueprint")
                .max_geodes(24)
        );
    }

    #[test]
    fn part1_sample() {
        assert_eq!(33, part1("sample.txt"));
    }
}
