pub fn process_part1(file: &str) -> String {
	let res = file.lines().map(|ele| {
		let len = ele.len();
		let (left, right) = ele.split_at(len / 2);
		let overlapped: Vec<_> = left.chars().filter(|ele| {
			if right.contains(*ele){
				true
			} else {
				false
			}	
		}).collect();
		if 'a' <= overlapped[0] && overlapped[0] <= 'z' {
			overlapped[0] as u32 - 'a' as u32 + 1
		} else {
			overlapped[0] as u32 - 'A' as u32 + 26 + 1
		}
	}).sum::<u32>();
	res.to_string()
} 

pub fn process_part2(file: &str) -> String {
	let file_lines: Vec<_> = file.split("\n").collect();
	let mut ans = 0;
	for i in 0..(file_lines.len() / 3) {
		// dbg!(i);
		let res:Vec<_> = file_lines[i * 3].chars().filter(|e| {
			if file_lines[i * 3 + 1].contains(*e) && file_lines[i * 3 + 2].contains(*e) {
				true
			} else {
				false
			}
		}).collect();
		if 'a' <= res[0] && res[0] <= 'z' {
			ans += res[0] as u32 - 'a' as u32 + 1;
		} else {
			ans += res[0] as u32 - 'A' as u32 + 1 + 26;
		}
	} 
	ans.to_string()
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let file = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
		assert_eq!(process_part1(file), "157");
    }

	#[test]
	fn part2_test() {
        let file = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
		assert_eq!(process_part2(file), "70");
	}
}
