pub fn process_part1(file: &str) -> String {
    let times = file
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|line| {
            if line.len() == 0 {
                None
            } else {
                Some(line.parse::<u64>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    let distance = file
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|line| {
            if line.len() == 0 {
                None
            } else {
                Some(line.parse::<u64>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    dbg!(&times);
    dbg!(&distance);
	times.iter().zip(distance.iter()).map(|(e1, e2)| {
		let mut res = 0;
		for t in 0..=*e1 {
			if t * (e1 - t) > *e2 {
				res += 1;
			}
		}
		res
	}).product::<u32>().to_string()
}

pub fn process_part2(file: &str) -> String {
    let times = file
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
		.chars().filter(|ch| {
			if *ch == ' ' {
				false
			} else {
				true
			}
		}).collect::<String>().parse::<u64>().unwrap();
		let distance = file
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
		.chars().filter(|ch| {
			if *ch == ' ' {
				false
			} else {
				true
			}
		}).collect::<String>().parse::<u64>().unwrap();
    dbg!(&times);
    dbg!(&distance);

	let mut res = 0;
	for t in 0..=times {
		if t * (times - t) > distance {
			res += 1;
		}
	}
	res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let file = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process_part1(file), "288");
    }

    #[test]
    fn part2_works() {
        let file = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process_part2(file), "71503");
    }
}
