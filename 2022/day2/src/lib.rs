pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn process_part1(file: &str) -> String {
	let result: i32 = file.lines().map(|s| {
		let opponent = match s.chars().next() {
			Some('A') => 1,
			Some('B') => 2,
			_ => 3
		};
		let my_play = match s.chars().nth(2) {
			Some('X') => 1,
			Some('Y') => 2,
			_ => 3,
		};
		if opponent == my_play{
			my_play + 3
		}
		else if opponent + 1 == my_play || opponent - 2 == my_play {
			my_play + 6
		} else {
			my_play
		}
	}).sum();
	result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = "A Y
B X
C Z";
        assert_eq!(process_part1(result), "15");
    }
}
