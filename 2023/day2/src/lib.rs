pub fn process_part1(file: &str) -> String {
	let ret: u32 = file.lines().map(|ele| {
		let res: Vec<_> = ele.split(": ").collect();
		let num = res[0].split(" ").nth(1).unwrap().parse::<u32>().unwrap();

		let res = res[1].split("; ").fold((0, 0, 0), |before, ball_case| {
			let mut maxval = before;

			ball_case.split("; ").for_each(|c| {
				c.split(", ").for_each(|cc| {
					let one_ball: Vec<_> = cc.split(" ").collect();
					let val = one_ball[0].parse::<u32>().unwrap();
					match one_ball[1] {
						"red" => {
							if val > maxval.0 {
								maxval.0 = val;
							}
						},
						"blue" => {
							if val > maxval.1 {
								maxval.1 = val;
							}
						},
						"green" => {
							if val > maxval.2 {
								maxval.2 = val;
							}
						},
						_ => panic!("not expected color"),
					};
				});
			});

			maxval
		});

		if res.0 > 12 || res.1 > 14 || res.2 > 13 {
			0
		} else {
			num
		}
	}).sum();
	ret.to_string()
}

pub fn process_part2(file: &str) -> String {
	let ret: u32 = file.lines().map(|ele| {
		let res: Vec<_> = ele.split(": ").collect();

		let res = res[1].split("; ").fold((0, 0, 0), |before, ball_case| {
			let mut maxval = before;

			ball_case.split("; ").for_each(|c| {
				c.split(", ").for_each(|cc| {
					let one_ball: Vec<_> = cc.split(" ").collect();
					let val = one_ball[0].parse::<u32>().unwrap();
					match one_ball[1] {
						"red" => {
							if val > maxval.0 {
								maxval.0 = val;
							}
						},
						"blue" => {
							if val > maxval.1 {
								maxval.1 = val;
							}
						},
						"green" => {
							if val > maxval.2 {
								maxval.2 = val;
							}
						},
						_ => panic!("not expected color"),
					};
				});
			});
			maxval
		});

		res.0 * res.1 * res.2
	}).sum();
	ret.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
		let file = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
		assert_eq!(process_part1(file), "8");
    }

	#[test]
	fn part2_works() {
		let file = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
		assert_eq!(process_part2(file), "2286");
	}
}
