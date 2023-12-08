use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Eq, Debug)]
struct Card {
    card: String,
    strength: Strength,
    bid: u64,
}

fn card_num(c: char) -> i32 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("not expecting value"),
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card.eq(&other.card)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength != other.strength {
            self.strength.cmp(&other.strength)
        } else {
            for (c1, c2) in self.card.chars().zip(other.card.chars()) {
                if card_num(c1) > card_num(c2) {
                    return std::cmp::Ordering::Greater;
                } else if card_num(c1) < card_num(c2) {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

fn get_strength(card: &str) -> Strength {
    let mut mapping = HashMap::new();
    card.chars().for_each(|ch| {
        if let Some(e) = mapping.get_mut(&ch) {
            *e += 1;
        } else {
            mapping.insert(ch, 1);
        }
    });

    let list = mapping.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();

    if list.len() == 1 {
        Strength::FiveOfKind
    } else if list.len() == 2 {
        if list[0].1 == &1 || list[0].1 == &4 {
            Strength::FourOfKind
        } else {
            Strength::FullHouse
        }
    } else if list.len() == 3 {
        if list[0].1 == &3 || list[1].1 == &3 || list[2].1 == &3 {
            Strength::ThreeOfKind
        } else {
            Strength::TwoPair
        }
    } else if list.len() == 4 {
        Strength::OnePair
    } else {
        Strength::HighCard
    }
}

pub fn process_part1(file: &str) -> String {
    let mut res = file
        .lines()
        .map(|line| {
            let mut l = line.split(' ');
            let card = l.next().unwrap().to_string();
            let bid = l.next().unwrap().parse::<u64>().unwrap();
            let strength = get_strength(&card);
            Card {
                card,
                bid,
                strength,
            }
        })
        .collect::<Vec<_>>();

    res.sort();
    res.iter()
        .enumerate()
        .map(|(i, r)| (i + 1) as u64 * r.bid)
        .sum::<u64>()
        .to_string()
}

#[derive(Eq, Debug)]
struct Card2 {
    card: String,
    strength: Strength,
    bid: u64,
}

fn card_num2(c: char) -> i32 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("not expecting value"),
    }
}

impl PartialEq for Card2 {
    fn eq(&self, other: &Self) -> bool {
        self.card.eq(&other.card)
    }
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength != other.strength {
            self.strength.cmp(&other.strength)
        } else {
            for (c1, c2) in self.card.chars().zip(other.card.chars()) {
                if card_num2(c1) > card_num2(c2) {
                    return std::cmp::Ordering::Greater;
                } else if card_num2(c1) < card_num2(c2) {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

fn get_strength2(card: &str) -> Strength {
    let mut mapping = HashMap::new();
    card.chars().for_each(|ch| {
        if let Some(e) = mapping.get_mut(&ch) {
            *e += 1;
        } else {
            mapping.insert(ch, 1);
        }
    });

    let mut list = mapping.into_iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();

	list.sort_by(|left, right| {
		right.1.partial_cmp(&left.1).unwrap()
	});

    if let Some((_, j_v)) = list
        .iter()
        .filter(|(k, _)| k == &'J')
        .collect::<Vec<_>>()
        .get(0)
    {
		let j_v = *j_v;
		list = list
        .into_iter()
        .filter(|(k, _)| k != &'J')
        .collect::<Vec<_>>();
		if let Some(ele) = list.get_mut(0) {
			ele.1 += j_v;
		} else {
			list.push(('J', 5));
		}
    }

    if list.len() == 1 {
        Strength::FiveOfKind
    } else if list.len() == 2 {
        if list[0].1 == 1 || list[0].1 == 4 {
            Strength::FourOfKind
        } else {
            Strength::FullHouse
        }
    } else if list.len() == 3 {
        if list[0].1 == 3 || list[1].1 == 3 || list[2].1 == 3 {
            Strength::ThreeOfKind
        } else {
            Strength::TwoPair
        }
    } else if list.len() == 4 {
        Strength::OnePair
    } else {
        Strength::HighCard
    }
}

pub fn process_part2(file: &str) -> String {
    let mut res = file
        .lines()
        .map(|line| {
            let mut l = line.split(' ');
            let card = l.next().unwrap().to_string();
            let bid = l.next().unwrap().parse::<u64>().unwrap();
            let strength = get_strength2(&card);
            Card2 {
                card,
                bid,
                strength,
            }
        })
        .collect::<Vec<_>>();

    res.sort();
	dbg!(&res);
    res.iter()
        .enumerate()
        .map(|(i, r)| (i + 1) as u64 * r.bid)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let file = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process_part1(file), "6440");
    }

    #[test]
    fn part2_works() {
        let file = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process_part2(file), "5905");
    }
}
