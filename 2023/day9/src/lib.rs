use std::iter::zip;

pub fn process_part1(file: &str) -> String {
	file.lines().map(|line| {
		let mut res = 0_i64;
		let mut list = line.split(" ").map(|num| num.parse::<i64>().unwrap()).collect::<Vec<_>>();
		loop {
			res += list.last().unwrap();
			{
				let list_cloned = list.clone();
				let next_iter = list_cloned.iter().skip(1);
				for (before, next) in zip(list.iter_mut(), next_iter) {
					*before = next - *before;
				}
			}
			let _ = list.pop();

			// dbg!(&list, res);
			if list.iter().all(|num| *num == 0) {
				break;
			}
		}
		// dbg!(res);
		res
	}).sum::<i64>().to_string()
}

pub fn process_part2(file: &str) -> String {
	file.lines().map(|line| {
		let mut res = 0_i64;
		let mut sign = 1_i64;
		let mut list = line.split(" ").map(|num| num.parse::<i64>().unwrap()).collect::<Vec<_>>();
		loop {
			res += list.get(0).unwrap() * sign;
			{
				let list_cloned = list.clone();
				let next_iter = list_cloned.iter().skip(1);
				for (before, next) in zip(list.iter_mut(), next_iter) {
					*before = next - *before;
				}
			}
			let _ = list.pop();
			sign = sign * -1;
			// dbg!(&list, res);
			if list.iter().all(|num| *num == 0) {
				break;
			}
		}
		// dbg!(res);
		res
	}).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_works() {
		let file = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
		assert_eq!(process_part1(file), "114");
	}

	#[test]
	fn part2_works() {
		let file = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
		assert_eq!(process_part2(file), "2");
	}
}
