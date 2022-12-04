pub fn solve(data: &str) -> i32 {
    let mut top_elves_max = vec![0, 0, 0];
    let mut temp: Vec<i32> = vec![];

    data.lines()
        .map(|line| line.split("/n").collect())
        .collect::<Vec<String>>()
        .into_iter()
        .for_each(|string| {
            match string.parse::<i32>() {
                Ok(x) => temp.push(x),
                Err(_) => {
                    if temp.is_empty() {
                        return;
                    }

                    let elf_sum: i32 = temp.iter().sum();

                    // replace lowest elf score with the new elf_sum
                    if let Some(min_elf) = top_elves_max.iter_mut().min() {
                        if elf_sum > *min_elf {
                            *min_elf = elf_sum;
                        }
                    }

                    // clear temp vec
                    temp.clear();
                }
            }
        });

    top_elves_max.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code::AdventOfCodeInput;
    use indoc::indoc;

    #[test]
    fn d1_aoc_input() {
        let aoc_input = AdventOfCodeInput::get_input(1);

        let answer = super::solve(&aoc_input.inp);

        assert_eq!(answer, 200945);
    }

    #[test]
    fn d1_aoc_test_input() {
        let aoc_input = indoc! {"

        1
        1

        2
        1

        3
        1

        4
        1

        "};

        let answer = super::solve(&aoc_input);

        assert_eq!(answer, 12);
    }
}
