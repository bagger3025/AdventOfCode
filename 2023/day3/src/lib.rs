use std::collections::HashMap;

fn is_symbol(ch: char) -> bool {
	if ch == '.' {
		false
	} else if '0' as u8 <= ch as u8 && ch as u8 <= '9' as u8 {
		false
	} else {
		true
	}
}

fn symbol_around(i: usize, j: usize, ret: &Vec<&str>) -> bool {
	if let Some(temp) = ret.get(i.saturating_sub(1)) {
		if let Some(ch) = temp.chars().nth(j.saturating_sub(1)) {
			if is_symbol(ch) {
				return true;
			}
		}
		if let Some(ch) = temp.chars().nth(j) {
			if is_symbol(ch) {
				return true;
			}
		}
		if let Some(ch) = temp.chars().nth(j + 1) {
			if is_symbol(ch) {
				return true;
			}
		}
	}
	if let Some(temp) = ret.get(i) {
		if let Some(ch) = temp.chars().nth(j.saturating_sub(1)) {
			if is_symbol(ch) {
				return true;
			}
		}
		if let Some(ch) = temp.chars().nth(j) {
			if is_symbol(ch) {
				return true;
			}
		}
		if let Some(ch) = temp.chars().nth(j + 1) {
			if is_symbol(ch) {
				return true;
			}
		}
	}
	if let Some(temp) = ret.get(i + 1) {
		if let Some(ch) = temp.chars().nth(j.saturating_sub(1)) {
			if is_symbol(ch) {
				return true;
			}
		}
		if let Some(ch) = temp.chars().nth(j) {
			if is_symbol(ch) {
				return true;
			}
		}
		if let Some(ch) = temp.chars().nth(j + 1) {
			if is_symbol(ch) {
				return true;
			}
		}
	}
	false
}

pub fn process_part1(file: &str) -> String {
	let ret: Vec<_> = file.lines().collect();

	let mut res: usize = 0;
	let mut temp: usize = 0;
	let mut isvalid = false;
	let mut processing = false;
	for (i, ele) in ret.iter().enumerate() {
		for (j, ch) in ele.chars().enumerate() {
			// dbg!((i, j, res, temp, isvalid, processing));
			match ch {
				'.' => {
					if processing {
						processing = false;
						if isvalid {
							res += temp;
						}
						isvalid = false;
						temp = 0;
					}
				},
				ch if '0' as u8 <= ch as u8 && ch as u8 <= '9' as u8 => {
					if processing {
						if !isvalid && symbol_around(i, j, &ret) {
							isvalid = true;
						}
						temp = temp * 10 + ch as usize - '0' as usize;
					} else {
						processing = true;
						isvalid = false;
						temp = ch as usize - '0' as usize;
						if symbol_around(i, j, &ret) {
							isvalid = true;
						}
					}
				},
				_ => {
					if processing {
						processing = false;
						if isvalid {
							res += temp;
						}
						isvalid = false;
						temp = 0;
					}
				}
			}
		}
		if processing {
			processing = false;
			if isvalid {
				res += temp;
			}
			isvalid = false;
			temp = 0;
		}
	}
	res.to_string()
}

pub fn process_part2(file: &str) -> String {
	let lines: Vec<_> = file.lines().collect();

	let mut start_pos = vec!();
	lines.iter().enumerate().for_each(|(i, l)| {
		l.chars().enumerate().for_each(|(j, ch)| {
			if ch == '*' {
				start_pos.push((i, j));
			}
		});
	});

	let mut pos_to_star_idx = vec!();

	start_pos.iter().enumerate().for_each(|(i, pos)| {
		if pos.0 > 0 {
			if pos.1 > 0 {
				pos_to_star_idx.push(((pos.0 - 1, pos.1 - 1), i));
			}
			pos_to_star_idx.push(((pos.0 - 1, pos.1), i));
			pos_to_star_idx.push(((pos.0 - 1, pos.1 + 1), i));
		}
		if pos.1 > 0 {
			pos_to_star_idx.push(((pos.0, pos.1 - 1), i));
		}
		pos_to_star_idx.push(((pos.0, pos.1), i));
		pos_to_star_idx.push(((pos.0, pos.1 + 1), i));
		if pos.1 > 0 {
			pos_to_star_idx.push(((pos.0 + 1, pos.1 - 1), i));
		}
		pos_to_star_idx.push(((pos.0 + 1, pos.1), i));
		pos_to_star_idx.push(((pos.0 + 1, pos.1 + 1), i));
	});

	let mut storing:Vec<Vec<usize>> = vec!();
	storing.resize(start_pos.len(), vec!());
	let mut belongs_to = vec!();
	let mut temp: usize = 0;
	let mut processing: bool = false;
	lines.iter().enumerate().for_each(|(i, l)| {
		l.chars().enumerate().for_each(|(j, ch)| {
			match ch {
				ch if '0' as u8 <= ch as u8 && ch as u8 <= '9' as u8 => {
					if processing {
						temp = temp * 10 + ch as usize - '0' as usize;
					} else {
						processing = true;
						belongs_to.clear();
						temp = ch as usize - '0' as usize;
					}
					pos_to_star_idx.iter().filter(|ele| {
						if ele.0 == (i, j) {
							true
						} else {
							false
						}
					}).map(|ele| {
						ele.1
					}).for_each(|ele| {
						belongs_to.push(ele);
					});
				},
				_ => {
					if processing {
						processing = false;
						belongs_to.sort();
						belongs_to.dedup();
						belongs_to.iter().for_each(|idx| {
							storing.get_mut(*idx).unwrap().push(temp);
						});
					}
				}
			}
		});
		if processing {
			processing = false;
			belongs_to.iter().for_each(|idx| {
				storing.get_mut(*idx).unwrap().push(temp);
			});
		}
	});
	let real_result: usize = storing.iter().filter(|ele| {
		ele.len() == 2
	}).map(|ele| {
		ele.get(0).unwrap() * ele.get(1).unwrap()
	}).sum();

	real_result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
		let file ="\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
		assert_eq!(process_part1(file), "4361");
    }

	#[test]
	fn part2_works() {
		let file ="\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
		assert_eq!(process_part2(file), "467835");
	}
}
