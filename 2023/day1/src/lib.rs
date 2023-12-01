pub fn process_part1(file: &str) -> String {
	let ret = file.lines().map(|ele| {
		let chars_list: Vec<_> = ele.chars().filter(|e| {
			'0' as u32 <= *e as u32 && *e as u32 <= '9' as u32
		}).collect();
		(chars_list[0] as u32 - '0' as u32) * 10 + *chars_list.last().unwrap() as u32 - '0' as u32
	}).sum::<u32>();
	ret.to_string()
}

pub fn process_part2(file: &str) -> String {
	let str_vec = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	let ret: u32 = file.lines().map(|ele| {
		let left = 'out_loop: loop {
			for ele_from in 0..ele.len() {
				let ele_sliced = &ele[ele_from..];
				let echar = ele_sliced.chars().nth(0).unwrap() as u32;
				if '0' as u32 <= echar && echar <= '9' as u32 {
					break 'out_loop echar - '0' as u32;
				}
				for i in 0..str_vec.len() {
					if ele_sliced.starts_with(str_vec[i]) {
						break 'out_loop i as u32 + 1;
					}
				}
			}
			break 0;
		};
		let right = 'out_loop: loop {
			for ele_to in (0..=ele.len()).rev() {
				let ele_sliced = &ele[..ele_to];
				let echar = ele_sliced.chars().last().unwrap() as u32;
				if '0' as u32 <= echar && echar <= '9' as u32 {
					break 'out_loop echar - '0' as u32;
				}
				for i in 0..str_vec.len() {
					if ele_sliced.ends_with(str_vec[i]) {
						break 'out_loop i as u32 + 1;
					}
				}
			}
			break 0;
		};
		left * 10 + right
	}).sum();

	ret.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
		let file = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
		assert_eq!(process_part1(file), "142");
    }

	#[test]
	fn part2_works() {
		let file = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
		assert_eq!(process_part2(file), "281");
	}
}
