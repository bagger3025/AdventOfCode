use std::collections::HashMap;

#[derive(Eq, PartialEq)]
enum Direction {
	North,
	West,
	East,
	South,
	None,
}

struct PipeDirection {
	dir1: Direction,
	dir2: Direction,
}

fn symbol_to_direction(pipe: char) -> PipeDirection {
	use Direction::*;
	match pipe {
		'|' => PipeDirection {
			dir1: South,
			dir2: North,
		},
		'-' => PipeDirection {
			dir1: East,
			dir2: West,
		},
		'L' => PipeDirection {
			dir1: East,
			dir2: North,
		},
		'J' => PipeDirection {
			dir1: West,
			dir2: North,
		},
		'7' => PipeDirection {
			dir1: West,
			dir2: South,
		},
		'F' => PipeDirection {
			dir1: East,
			dir2: South,
		},
		'.' => PipeDirection {
			dir1: None,
			dir2: None,
		},
		'S' => PipeDirection {
			dir1: None,
			dir2: None,
		},
		_ => panic!("unexpected char"),
	}
}

fn examine_dir(
	file: &str,
	point: (usize, usize),
	dir: Direction,
	visited: &mut HashMap<(usize, usize), i32>,
	cur_len: i32,
) -> bool {
	let symbol = file
		.lines()
		.nth(point.0)
		.unwrap()
		.chars()
		.nth(point.1)
		.unwrap();
	let symbol = symbol_to_direction(symbol);
	if symbol.dir1 == dir || symbol.dir2 == dir {
		if !visited.contains_key(&point) {
			visited.insert(point, cur_len);
			true
		} else {
			false
		}
	} else {
		false
	}
}

pub fn process_part1(file: &str) -> String {
	let mut start_point = file
		.lines()
		.enumerate()
		.filter_map(|(i, line)| {
			let res = line
				.chars()
				.enumerate()
				.filter_map(|(j, ch)| if ch != 'S' { None } else { Some(j) })
				.collect::<Vec<_>>();
			if res.len() == 0 {
				None
			} else if res.len() == 1 {
				Some((i, res[0]))
			} else {
				panic!("S symbol duplicated");
			}
		})
		.collect::<Vec<_>>();
	if start_point.len() != 1 {
		panic!("starting point more than 1");
	}
	// dbg!(&start_point);
	let mut visited = HashMap::new();

	visited.insert(start_point[0], 0);
	let mut cur_len = 1;
	loop {
		start_point = start_point
			.into_iter()
			.flat_map(|point| {
				let mut returning_vec = Vec::new();
				if point.0 > 0
					&& examine_dir(
						file,
						(point.0 - 1, point.1),
						Direction::South,
						&mut visited,
						cur_len,
					)
				{
					returning_vec.push((point.0 - 1, point.1));
				}
				if point.1 > 0
					&& examine_dir(
						file,
						(point.0, point.1 - 1),
						Direction::East,
						&mut visited,
						cur_len,
					)
				{
					returning_vec.push((point.0, point.1 - 1));
				}
				if point.1 < file.lines().next().unwrap().chars().count() - 1
					&& examine_dir(
						file,
						(point.0, point.1 + 1),
						Direction::West,
						&mut visited,
						cur_len,
					)
				{
					returning_vec.push((point.0, point.1 + 1));
				}
				if point.0 < file.lines().count() - 1
					&& examine_dir(
						file,
						(point.0 + 1, point.1),
						Direction::North,
						&mut visited,
						cur_len,
					)
				{
					returning_vec.push((point.0 + 1, point.1));
				}
				returning_vec
			})
			.collect();
		cur_len += 1;
		if start_point.len() == 0 {
			break;
		}
	}

	let result = visited.iter().max_by(|a1, a2| a1.1.cmp(a2.1)).unwrap();
	result.1.to_string()
}

fn print_map(map: &Vec<Vec<char>>) {
	map.iter().for_each(|line| {
		line.iter().for_each(|ch| {
			print!("{ch}");
		});
		println!();
	})
}

fn dfs(map: &mut Vec<Vec<char>>, point: (usize, usize), set_to: char) -> i32 {
	let mut res = 0;
	match map[point.0][point.1] {
		'O' => (),
		ch if ch == set_to => (),
		_ => {
			map[point.0][point.1] = set_to;
			res = 1;
			if point.0 > 0 {
				res += dfs(map, (point.0 - 1, point.1), set_to);
			}
			if point.0 < map.len() - 1 {
				res += dfs(map, (point.0 + 1, point.1), set_to);
			}
			if point.1 > 0 {
				res += dfs(map, (point.0, point.1 - 1), set_to);
			}
			if point.1 < map[0].len() - 1 {
				res += dfs(map, (point.0, point.1 + 1), set_to);
			}
		}
	}
	res
}

fn update_startsymbol(original_symbols: &mut Vec<Vec<char>>, original_point: (usize, usize), starting_point: &Vec<(usize, usize)>) {
	let dir1 = {
		if original_point.0 == starting_point[0].0 {
			if original_point.1 == starting_point[0].1 + 1 {
				Direction::West
			} else {
				Direction::East
			}
		} else if original_point.0 == starting_point[0].0 + 1 {
			Direction::North
		} else {
			Direction::South
		}
	};

	let dir2 = {
		if original_point.0 == starting_point[1].0 {
			if original_point.1 == starting_point[1].1 + 1 {
				Direction::West
			} else {
				Direction::East
			}
		} else if original_point.0 == starting_point[1].0 + 1 {
			Direction::North
		} else {
			Direction::South
		}
	};

	use Direction::*;
	let ch = match (dir1, dir2) {
		(North, South) | (South, North) => '|',
		(East, West) | (West, East) => '-',
		(North, East) | (East, North) => 'L',
		(North, West) | (West, North) => 'J',
		(South, West) | (West, South) => '7',
		(South, East) | (East, South) => 'F',
		_ => panic!("not possible")
	};

	original_symbols[original_point.0][original_point.1] = ch;
}

fn set(map: &mut Vec<Vec<char>>, start_pos: (usize, usize), change_to: char) -> i32 {
	match map[start_pos.0][start_pos.1] {
		'O' => 0,
		ch if ch == change_to => 0,
		_ => {
			let mut changed = 1;
			map[start_pos.0][start_pos.1] = change_to;
			return 0;
			if start_pos.0 > 0 {
				changed += set(map, (start_pos.0 - 1, start_pos.1), change_to);
			}
			if start_pos.0 < map.len() - 1 {
				changed += set(map, (start_pos.0 + 1, start_pos.1), change_to);
			}
			if start_pos.1 > 0 {
				changed += set(map, (start_pos.0, start_pos.1 - 1), change_to);
			}
			if start_pos.1 < map[0].len() - 1 {
				changed += set(map, (start_pos.0, start_pos.1 + 1), change_to);
			}
			changed
		}
	}
}

pub fn process_part2(file: &str) -> String {
	let mut start_point = file
		.lines()
		.enumerate()
		.filter_map(|(i, line)| {
			let res = line
				.chars()
				.enumerate()
				.filter_map(|(j, ch)| if ch != 'S' { None } else { Some(j) })
				.collect::<Vec<_>>();
			if res.len() == 0 {
				None
			} else if res.len() == 1 {
				Some((i, res[0]))
			} else {
				panic!("S symbol duplicated");
			}
		})
		.collect::<Vec<_>>();
	if start_point.len() != 1 {
		panic!("starting point more than 1");
	}
	let mut original_symbols = file.lines().map(|line| {
		line.chars().collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	// dbg!(&start_point);
	let mut visited = HashMap::new();

	let original_point = start_point[0].clone();

	visited.insert(start_point[0], 0);
	let mut cur_len = 1;
	start_point = start_point
	.into_iter()
	.flat_map(|point| {
		let mut returning_vec = Vec::new();
		if point.0 > 0
			&& examine_dir(
				file,
				(point.0 - 1, point.1),
				Direction::South,
				&mut visited,
				cur_len,
			)
		{
			returning_vec.push((point.0 - 1, point.1));
		}
		if point.1 > 0
			&& examine_dir(
				file,
				(point.0, point.1 - 1),
				Direction::East,
				&mut visited,
				cur_len,
			)
		{
			returning_vec.push((point.0, point.1 - 1));
		}
		if point.1 < file.lines().next().unwrap().chars().count() - 1
			&& examine_dir(
				file,
				(point.0, point.1 + 1),
				Direction::West,
				&mut visited,
				cur_len,
			)
		{
			returning_vec.push((point.0, point.1 + 1));
		}
		if point.0 < file.lines().count() - 1
			&& examine_dir(
				file,
				(point.0 + 1, point.1),
				Direction::North,
				&mut visited,
				cur_len,
			)
		{
			returning_vec.push((point.0 + 1, point.1));
		}
		returning_vec
	})
	.collect();
	cur_len += 1;

	update_startsymbol(&mut original_symbols, original_point, &start_point);

	loop {
		start_point = start_point.into_iter().filter_map(|point| {
			
			let symbol = file.lines().nth(point.0).unwrap().chars().nth(point.1).unwrap();
			let dir = symbol_to_direction(symbol);
			let mut result = None;
			let width = file.lines().next().unwrap().chars().count();
			let height = file.lines().count();
			match dir.dir1 {
				Direction::East => {
					if point.1 < width - 1 && !visited.contains_key(&(point.0, point.1 + 1)) {
						result = Some((point.0, point.1 + 1));
					}
				},
				Direction::West => {
					if point.1 > 0 && !visited.contains_key(&(point.0, point.1 - 1)) {
						result = Some((point.0, point.1 - 1));
					}
				},
				Direction::South => {
					if point.0 < height - 1 && !visited.contains_key(&(point.0 + 1, point.1)) {
						result = Some((point.0 + 1, point.1));
					}
				},
				Direction::North => {
					if point.0 > 0 && !visited.contains_key(&(point.0 - 1, point.1)) {
						result = Some((point.0 - 1, point.1));
					}
				},
				_=>panic!("unexpected symbol")
			};

			if result.is_none() {
				match dir.dir2 {
					Direction::East => {
						if point.1 < width - 1 && !visited.contains_key(&(point.0, point.1 + 1)) {
							result = Some((point.0, point.1 + 1));
						}
					},
					Direction::West => {
						if point.1 > 0 && !visited.contains_key(&(point.0, point.1 - 1)) {
							result = Some((point.0, point.1 - 1));
						}
					},
					Direction::South => {
						if point.0 < height - 1 && !visited.contains_key(&(point.0 + 1, point.1)) {
							result = Some((point.0 + 1, point.1));
						}
					},
					Direction::North => {
						if point.0 > 0 && !visited.contains_key(&(point.0 - 1, point.1)) {
							result = Some((point.0 - 1, point.1));
						}
					},
					_=>panic!("unexpected symbol")
				};
			}
			if let Some(r) = result {
				visited.insert(r, cur_len);
			}
			result
		}).collect();
		cur_len += 1;
		if start_point.len() == 0 {
			break;
		}
	}

	let mut map = file.lines().map(|line| {
		line.chars().collect::<Vec<_>>()
	}).collect::<Vec<_>>();

	visited.iter().for_each(|(k, _)| {
		map[k.0][k.1] = 'O';
	});

	print_map(&map);
	let width = map[0].len();
	let height = map.len();

	for i in 0..width {
		dfs(&mut map, (0, i), 'X');
		dfs(&mut map, (height - 1, i), 'X');
	}
	for i in 0..height {
		dfs(&mut map, (i, 0), 'X');
		dfs(&mut map, (i, width - 1), 'X');
	}

	println!();
	print_map(&map);

	println!();
	print_map(&original_symbols);

	loop {
		let mut changed = 0;

		visited.iter().for_each(|(k, _)| {
			let symbol = original_symbols[k.0][k.1];
			let dir = symbol_to_direction(symbol);
			let mut p1_pos = (0, 0);
			let p1 = if dir.dir1 != Direction::East && dir.dir2 != Direction::East {
				if k.1 < width - 1 {
					p1_pos = (k.0, k.1 + 1);
					map[k.0][k.1 + 1]
				} else {
					'?'
				}
			} else if dir.dir1 != Direction::West && dir.dir2 != Direction::West {
				if k.1 > 0 {
					p1_pos = (k.0, k.1 - 1);
					map[k.0][k.1 - 1]
				} else {
					'?'
				}
			} else if dir.dir1 != Direction::South && dir.dir2 != Direction::South {
				if k.0 < height - 1 {
					p1_pos = (k.0 + 1, k.1);
					map[k.0 + 1][k.1]
				} else {
					'?'
				}
			} else if dir.dir1 != Direction::North && dir.dir2 != Direction::North {
				if k.0 > 0 {
					p1_pos = (k.0 - 1, k.1);
					map[k.0 - 1][k.1]
				} else {
					'?'
				}
			} else {
				'?'
			};
			if p1 == '?' {
				return;
			}
			let mut p2_pos = (0, 0);
			let p2 = if dir.dir1 != Direction::North && dir.dir2 != Direction::North {
				if k.0 > 0 {
					p2_pos = (k.0 - 1, k.1);
					map[k.0 - 1][k.1]
				} else {
					'?'
				}
			} else if dir.dir1 != Direction::South && dir.dir2 != Direction::South {
				if k.0 < height - 1 {
					p2_pos = (k.0 + 1, k.1);
					map[k.0 + 1][k.1]
				} else {
					'?'
				}
			} else if dir.dir1 != Direction::West && dir.dir2 != Direction::West {
				if k.1 > 0 {
					p2_pos = (k.0, k.1 - 1);
					map[k.0][k.1 - 1]
				} else {
					'?'
				}
			} else if dir.dir1 != Direction::East && dir.dir2 != Direction::East {
				if k.1 < width - 1 {
					p2_pos = (k.0, k.1 + 1);
					map[k.0][k.1 + 1]
				} else {
					'?'
				}
			} else {
				'?'
			};
			if p2 == '?' {
				return;
			}

			if p1 == '*' {
				dbg!(p1, p2_pos);
				changed += set(&mut map, p2_pos, 'X');
			} else if p1 == 'X' {
				dbg!(p1, p2_pos);
				changed += set(&mut map, p2_pos, '*');
			}
			if p2 == 'X' {
				dbg!(p1, p1_pos);
				changed += set(&mut map, p1_pos, '*');
			} else if p2 == '*' {
				dbg!(p2, p1_pos);
				changed += set(&mut map, p1_pos, 'X');
			}
		});

		dbg!(changed);
		print_map(&map);
		if changed == 0 {
			break;
		}
	}

	file.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_works() {
		let file = "\
.....
.S-7.
.|.|.
.L-J.
.....";
		assert_eq!(process_part1(file), "4");
	}

	#[test]
	fn part1_works2() {
		let file = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
		assert_eq!(process_part1(file), "8");
	}

	#[test]
	fn part2_works() {
		let file = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
		assert_eq!(process_part2(file), "4");
	}

	#[test]
	fn part2_works2() {
		let file = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
		assert_eq!(process_part2(file), "4");
	}

	#[test]
	fn part2_works3() {
		let file = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
		assert_eq!(process_part2(file), "8");
	}

	#[test]
	fn part2_works4() {
		let file = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
		assert_eq!(process_part2(file), "10");
	}
}
