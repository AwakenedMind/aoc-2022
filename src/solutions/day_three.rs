use std::{cell::RefCell, collections::HashMap};

pub fn solve(data: &str) -> i32 {
    let all_rucksacks = data.lines().into_iter().collect::<Vec<_>>();
    let all_groups = all_rucksacks.chunks(3).collect::<Vec<_>>();

    let mut priority_map: HashMap<char, i32> = HashMap::new();
    let mut item_map: HashMap<String, char> = HashMap::new();
    let mut priority_cache: HashMap<String, bool> = HashMap::new();

    let mut priority_sum = 0;

    // map chars to priority values in hashmap
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .for_each(|(i, u)| {
            priority_map.insert(u, i as i32 + 1);
        });

    for (i, rucksack) in all_groups.iter().enumerate() {
        rucksack.iter().enumerate().for_each(|(j, f)| {
            f.chars().for_each(|cur_letter| {
                // start with 1
                let group = i as i32 + 1;
                let rucksack = j as i32 + 1;

                let cur_key =
                    RefCell::new(format!("{}{}{}", group, rucksack, cur_letter).to_owned());

                item_map.insert(cur_key.borrow().clone(), cur_letter);

                // only start on the last rucksack of the group
                if rucksack == 3 {
                    let prev_rucksack = format!("{}{}{}", group, rucksack - 1, cur_letter);
                    let prev2_rucksack = format!("{}{}{}", group, rucksack - 2, cur_letter);

                    // check for existence of pairs in the group's rucksack
                    if item_map.contains_key(&prev_rucksack)
                        && item_map.contains_key(&prev2_rucksack)
                    {
                        let prev_value = item_map.get(&prev_rucksack).unwrap();
                        let prev2_value = item_map.get(&prev2_rucksack).unwrap();

                        // check if the cur_letter matches the 2 previous rucksacks letters
                        if cur_letter == *prev_value
                            && cur_letter == *prev2_value
                            && !priority_cache.contains_key(&cur_key.borrow().clone())
                        {
                            priority_cache.insert(cur_key.borrow().clone(), true);

                            priority_sum = priority_sum + priority_map[&cur_letter];

                            println!("priority match {:?}", cur_letter);
                        }
                    }
                }
            })
        });
    }

    priority_sum
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code::AdventOfCodeInput;

    #[test]
    fn d3_aoc_input() {
        let aoc_input = AdventOfCodeInput::get_input(3);

        let answer = super::solve(&aoc_input.inp);

        assert_eq!(answer, 2276);
    }
}
