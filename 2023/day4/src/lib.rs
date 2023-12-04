pub fn process_part1(file: &str) -> String {
    let res: i32 = file
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|line| {
            let left_list: Vec<_> = line
                .split(" | ")
                .next()
                .unwrap()
                .split(" ")
                .filter_map(|num| {
                    if num.len() == 0 {
                        None
                    } else {
                        Some(num.parse::<u32>().unwrap())
                    }
                })
                .collect();

            let winning_cards = line
                .split(" | ")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter_map(|num| {
                    if num.len() == 0 {
                        return None;
                    }
                    let num = num.parse::<u32>().unwrap();
                    if left_list.contains(&num) {
                        Some(1)
                    } else {
                        Some(0)
                    }
                })
                .sum::<u32>();
            if winning_cards == 0 {
                0
            } else {
                2_i32.pow(winning_cards - 1)
            }
        })
        .sum();
    res.to_string()
}

pub fn process_part2(file: &str) -> String {
	let mut cards: Vec<u32> = Vec::new();
	cards.resize(file.lines().count(), 1);
    file
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap())
        .enumerate()
		.for_each(|(i, line)| {
            let left_list: Vec<_> = line
                .split(" | ")
                .next()
                .unwrap()
                .split(" ")
                .filter_map(|num| {
                    if num.len() == 0 {
                        None
                    } else {
                        Some(num.parse::<u32>().unwrap())
                    }
                })
                .collect();

            let winning_cards = line
                .split(" | ")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter_map(|num| {
                    if num.len() == 0 {
                        return None;
                    }
                    let num = num.parse::<u32>().unwrap();
                    if left_list.contains(&num) {
                        Some(1)
                    } else {
                        Some(0)
                    }
                })
                .sum::<u32>();
			
			let i_card = *cards.get(i).unwrap();
			for j in 1..=winning_cards as usize {
				let cards = cards.get_mut(i + j);
				if let Some(c) = cards {
					*c += i_card;
				}
			}
        });
    cards.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let file = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_part1(file), "13");
    }

    #[test]
    fn part2_works() {
        let file = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_part2(file), "30");
    }
}
