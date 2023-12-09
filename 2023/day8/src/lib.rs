use std::collections::HashMap;

pub fn process_part1(file: &str) -> String {
    let instructions = file.lines().next().unwrap().to_string();

    let mut map = HashMap::new();
    file.lines().skip(2).for_each(|line| {
        let start = line.split(" = ").next().unwrap();
        line.split(" = ")
            .nth(1)
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .enumerate()
            .for_each(|(i, target)| {
                match i {
                    0 => map.insert((start, 0), target),
                    1 => map.insert((start, 1), target),
                    _ => panic!("not expected"),
                };
            });
    });

    // dbg!(&map);

    let mut cur_state = "AAA";
    (instructions
        .chars()
        .cycle()
        .take_while(|inst| {
            cur_state = match *inst {
                'L' => map[&(cur_state, 0)],
                'R' => map[&(cur_state, 1)],
                _ => panic!("Not L or R"),
            };
            cur_state != "ZZZ"
        })
        .collect::<Vec<_>>()
        .len()
        + 1)
    .to_string()
}

fn get_lcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a > b {
        get_lcd(a % b, b)
    } else if a == b {
        a
    } else {
        get_lcd(b % a, a)
    }
}

pub fn process_part2(file: &str) -> String {
    let instructions = file.lines().next().unwrap().to_string();

    let mut map = HashMap::new();
    file.lines().skip(2).for_each(|line| {
        let start = line.split(" = ").next().unwrap();
        line.split(" = ")
            .nth(1)
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .enumerate()
            .for_each(|(i, target)| {
                map.insert((start, i), target);
            });
    });

    let mut every_states = map
        .iter()
        .map(|((v, _), _)| (*v, *v))
        .collect::<HashMap<_, _>>();
    let mut states_to_state = every_states
        .iter()
        .map(|st| match st.0.ends_with('Z') {
            true => (*st.0, vec![0], *st.0),
            false => (*st.0, vec![], *st.0),
        })
        .collect::<Vec<_>>();

    instructions.chars().enumerate().for_each(|(i, inst)| {
        every_states.iter_mut().for_each(|(start, cur)| {
            match inst {
                'L' => *cur = map[&(*cur, 0)],
                'R' => *cur = map[&(*cur, 1)],
                _ => panic!("not L or R"),
            };
            if cur.ends_with('Z') {
                let mut temp = states_to_state
                    .iter_mut()
                    .filter(|(st, _, _)| st == start)
                    .collect::<Vec<_>>();
                temp[0].1.push(i + 1);
            }
        });
    });
    states_to_state
        .iter_mut()
        .zip(every_states.iter())
        .for_each(|(st, ev)| {
            if st.0 != *ev.0 {
                panic!("not same!");
            }
            st.2 = ev.1;
        });
    // dbg!(&states_to_state);

    let mut cur_states = map
        .iter()
        .filter_map(|((v, _), _)| match v.ends_with('A') {
            true => Some(*v),
            false => None,
        })
        .collect::<Vec<_>>();
    cur_states.sort();
    cur_states.dedup();
    // dbg!(&cur_states);

    let res;
    let mut length = instructions.len();

    // dbg!(&states_to_state);
    loop {
        let states_to_state2 = states_to_state.clone();

        states_to_state.iter_mut().for_each(|(_, v, end)| {
            let l = states_to_state2
                .iter()
                .filter(|(start, _, _)| start == end)
                .collect::<Vec<_>>();
            let after = l.get(0).unwrap();
            after.1.iter().for_each(|val| {
                v.push(*val + length);
            });
            v.dedup();
            *end = after.2;
        });
        length = length * 2;
        dbg!(&length);

        let temp = states_to_state
            .iter()
            .filter(|e| e.0.ends_with("Z"))
            .map(|e| e.1.get(1).clone())
            .collect::<Vec<_>>();

        if temp.iter().all(|ele| {
            ele.is_some()
        }) {
            let temp = temp.into_iter().map(|ele| {
                *ele.unwrap()
            }).fold(1_usize, |before, cur|{
                let lcd = get_lcd(before, cur);
                dbg!(before, cur, lcd);
                before / lcd * cur
            });
            res = temp;
            break;
        }
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let file = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process_part1(file), "2");
    }

    #[test]
    fn part1_works2() {
        let file = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process_part1(file), "6");
    }

    #[test]
    fn part2_works() {
        let file = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process_part2(file), "6");
    }
}
