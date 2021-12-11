use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn octos_from_input(input: &str) -> Vec<Vec<u8>> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn step(octos: &mut Vec<Vec<u8>>) {
    for y in 0..10 {
        for x in 0..10 {
            let o = octos.get_mut(y).unwrap().get_mut(x).unwrap();
            *o += 1;
        }
    }
}

fn flash(octos: &mut Vec<Vec<u8>>) -> i32 {
    let mut flashes = 0;
    for y in 0..10 {
        for x in 0..10 {
            let o = octos.get_mut(y).unwrap().get_mut(x).unwrap();
            if *o > 9 {
                flashes += 1;
                *o = 0;
                energize_neighbours(octos, &x, &y);
            }
        }
    }
    let mut more = flashes > 0;
    while more {
        let more_flashes = flash(octos);
        flashes += more_flashes;
        more = more_flashes > 0;
    }

    flashes
}

fn energize_neighbours(octos: &mut Vec<Vec<u8>>, x: &usize, y: &usize) {
    for ny in 0.max(*y as i32 - 1)..=9.min(*y as i32 + 1) {
        for nx in 0.max(*x as i32 - 1)..=9.min(*x as i32 + 1) {
            let o = octos
                .get_mut(ny as usize)
                .unwrap()
                .get_mut(nx as usize)
                .unwrap();
            if *o > 0 {
                *o += 1;
            }
        }
    }
}

fn print(octos: &Vec<Vec<u8>>) {
    dbg!(octos
        .iter()
        .map(|row| row
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(""))
        .collect::<Vec<String>>());
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(1656) ; "sample")]
    #[test_case("input.txt" => is eq(1649) ; "input")]
    fn part1(input: &str) -> i32 {
        let mut octos = octos_from_input(input);

        let mut flashes = 0;

        for _ in 0..100 {
            step(&mut octos);

            flashes += flash(&mut octos);
        }

        flashes
    }

    #[test_case("sample1.txt" => is eq(195) ; "sample")]
    #[test_case("input.txt" => is eq(256) ; "input")]
    fn part2(input: &str) -> i32 {
        let mut octos = octos_from_input(input);

        let mut flashes = 0;
        let mut counter = 0;
        while flashes < 100 {
            counter += 1;
            step(&mut octos);

            flashes = flash(&mut octos);
        }

        counter
    }

    #[test]
    fn first_steps() {
        let mut octos = octos_from_input("sample1.txt");

        step(&mut octos);
        assert_eq!(0, flash(&mut octos));
        print(&octos);

        step(&mut octos);
        assert_eq!(35, flash(&mut octos));
        print(&octos);

        step(&mut octos);
        assert_eq!(45, flash(&mut octos));
        print(&octos);

        step(&mut octos);
        flash(&mut octos);
        print(&octos);

        step(&mut octos);
        flash(&mut octos);
        print(&octos);

        step(&mut octos);
        assert_eq!(1, flash(&mut octos));
        print(&octos);
    }
}
