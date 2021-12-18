#[macro_use]
extern crate scan_fmt;

use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn area_from_string(input: String) -> ((i32, i32), (i32, i32)) {
    let (x1, x2, y1, y2) = scan_fmt!(
        &input,
        "target area: x={d}..{d}, y={d}..{d}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();

    ((x1, y1), (x2, y2))
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(45); "sample1")]
    #[test_case("input.txt" => is eq(7750); "input")]
    fn part1(input: &str) -> i32 {
        let ((x1, y1), (x2, y2)) = area_from_string(read_to_string(input).unwrap());

        // vx>0
        // vy>0
        // horiz speed stop: vx-n+1/2=0 == n=vx+1/2
        // vert speed stop: vy-n+1/2=0 == n=vy+1/2
        // x=n*vx+n-n(n+1)/2
        // y=n*vy+n-n(n+1)/2

        // max height:
        // y=vy*vy+vy-vy(vy+1)/2
        // y=vy*(vy+1-(vy+1)/2)
        // y=vy*(vy+1)/2

        // y=vy²/2+vy/2

        // x^2/+x/2-t

        // x = -1/2 +- sqrt(1/4+2t)

        let minvx = f32::ceil(f32::sqrt(1.0 / 4.0 + 2.0 * x1 as f32) - 1.0 / 2.0) as i32;
        let maxvx = f32::floor(f32::sqrt(1.0 / 4.0 + 2.0 * x2 as f32) - 1.0 / 2.0) as i32;

        let mut max_height = 0;

        const LIMIT: i32 = 300;
        for vx in minvx..=maxvx {
            for vy in 1..=LIMIT {
                let y = vx * vy + vx - vx * (vx + 1) / 2;
                if y >= y1 {
                    for n in vx..=LIMIT {
                        let nx = if n <= vx { n } else { vx };
                        let x = nx * vx + nx - nx * (nx + 1) / 2;
                        let y = n * vy + n - n * (n + 1) / 2;
                        if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                            let max = vy * (vy + 1) / 2;
                            if max > max_height {
                                max_height = max;
                            }
                        }
                    }
                }
            }
        }

        max_height
    }
    #[test_case("sample1.txt" => is eq(112); "sample1")]
    #[test_case("input.txt" => is eq(4120); "input")]
    fn part2(input: &str) -> usize {
        let ((x1, y1), (x2, y2)) = area_from_string(read_to_string(input).unwrap());

        let minvx = f32::ceil(f32::sqrt(1.0 / 4.0 + 2.0 * x1 as f32) - 1.0 / 2.0) as i32;
        let maxvx = x2;

        let mut hits: HashSet<(i32, i32)> = HashSet::new();

        const LIMIT: i32 = 300;
        for vx in minvx..=maxvx {
            for vy in y1..=LIMIT {
                for n in 1..=LIMIT {
                    let nx = if n <= vx { n } else { vx };
                    let x = nx * vx + nx - nx * (nx + 1) / 2;
                    let y = n * vy + n - n * (n + 1) / 2;
                    if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                        hits.insert((vx, vy));
                    }
                }
            }
        }

        hits.len()
    }
}
