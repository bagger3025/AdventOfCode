#[derive(Debug, Eq)]
struct Info {
    destination: u64,
    src: u64,
    length: u64,
}

impl Ord for Info {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.destination.cmp(&other.destination)
    }
}

impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        self.destination.eq(&other.destination)
    }
}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.destination.partial_cmp(&other.destination)
    }
}

#[derive(Debug, Eq)]
struct Seed {
    start: u64,
    length: u64,
}

impl Ord for Seed {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.start.cmp(&other.start)
	}
}

impl PartialOrd for Seed {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.start.partial_cmp(&other.start)
	}
}

impl PartialEq for Seed {
	fn eq(&self, other: &Self) -> bool {
		self.start.eq(&other.start)
	}
}

pub fn process_part1(file: &str) -> String {
    let mut line_iterator = file.lines();
    let mut seeds = line_iterator
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|ele| ele.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    // dbg!(&seeds);

    let res = file
        .split("\n\n")
        .skip(1)
        .map(|lines| {
            lines
                .split("\n")
                .skip(1)
                .map(|line| {
                    let res = line
                        .split(" ")
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    Info {
                        destination: res[0],
                        src: res[1],
                        length: res[2],
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // dbg!(&res);

    res.iter().for_each(|info| {
        seeds.iter_mut().for_each(|ele| {
            let temp = info
                .iter()
                .filter(|i| i.src <= *ele && *ele < i.src + i.length)
                .map(|i| *ele - i.src + i.destination)
                .collect::<Vec<_>>();
            if let Some(t) = temp.get(0) {
                *ele = *t;
            }
        });
    });

    seeds.iter().min().unwrap().to_string()
}

pub fn process_part2(file: &str) -> String {
    let mut line_iterator = file.lines();
    let mut seeds = line_iterator
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|ele| ele.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    dbg!(&seeds);
    let first_iter = seeds.iter().step_by(2);
    let second_iter = seeds.iter().skip(1).step_by(2);
    let mut seeds = first_iter
        .zip(second_iter)
        .map(|(one, two)| Seed {
            start: *one,
            length: *two,
        })
        .collect::<Vec<_>>();
    dbg!(&seeds);

    let res = file
        .split("\n\n")
        .skip(1)
        .map(|lines| {
            let mut r = lines
                .split("\n")
                .skip(1)
                .map(|line| {
                    let res = line
                        .split(" ")
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    Info {
                        destination: res[0],
                        src: res[1],
                        length: res[2],
                    }
                })
                .collect::<Vec<_>>();
            r.sort();
            r
        })
        .collect::<Vec<_>>();

    dbg!(&res);

    res.iter().for_each(|info| {
		let mut info2 = info.iter().flat_map(|i| {
			vec![i.src, i.src+i.length]
		}).collect::<Vec<_>>();
		info2.sort();
		info2.dedup();
		dbg!(&info2);
		seeds = seeds.iter().flat_map(|seed| {
			let mut points = info2.iter().filter_map(|i|{
				if seed.start <= *i && *i < seed.start + seed.length {
					Some(i)
				} else {
					None
				}
			}).collect::<Vec<_>>();
			points.insert(0, &seed.start);
			let last_val = seed.start + seed.length;
			points.push(&last_val);
			points.dedup();

			dbg!(&seed);
			let mut prev = seed.start;
			let temp = points.iter().filter_map(|p| {
				if **p - prev == 0 {
					return None;
				}
				let ret = Seed{start: prev, length: **p - prev};
				prev = **p;
				Some(ret)
			}).collect::<Vec<_>>();
			dbg!(&temp);
			temp
		}).collect::<Vec<_>>();

		seeds.sort();
		dbg!(&seeds);

		seeds = seeds.iter().map(|seed| {
			let temp = info.iter().filter(|inf| {
				if seed.start >= inf.src + inf.length || seed.start + seed.length <= inf.src {
					false
				} else {
					true
				}
			}).collect::<Vec<_>>();
			dbg!(&seed, &temp);
			if temp.len() == 1{
				Seed{start: seed.start - temp[0].src + temp[0].destination, length: seed.length}}
			else if temp.len() == 0{
				Seed{start:seed.start, length:seed.length}}
			else{
				panic!("Could not happen")}
		}).collect::<Vec<_>>();
    });

	seeds.sort();
	dbg!(&seeds);

    seeds[0].start.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let file = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process_part1(file), "35");
    }

    #[test]
    fn part2_works() {
        let file = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process_part2(file), "46");
    }
}
