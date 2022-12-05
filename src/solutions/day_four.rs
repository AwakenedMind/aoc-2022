pub fn solve(data: &str) -> i32 {
    let pairs = data
        .lines()
        .filter_map(|pair_string| {
            let (left_pair_string, right_pair_string) = pair_string.split_once(',')?;
            let (left_x, left_y) = left_pair_string.split_once('-')?;
            let (right_x, right_y) = right_pair_string.split_once('-')?;

            Some((
                left_x.parse().ok()?,
                left_y.parse().ok()?,
                right_x.parse().ok()?,
                right_y.parse().ok()?,
            ))
        })
        .collect::<Vec<(usize, _, _, _)>>();

    let p1 = pairs
        .iter()
        //   Full Contain
        //   x1---------y1
        //   x2---------y2
        //
        //   Left Contain inside Right
        //      x1------y1
        //   x2---------y2
        //
        //   Right Contain inside Left
        // x1-----------y1
        //   x2---------y2
        .filter(|(x1, y1, x2, y2)| (x1 <= x2 && y1 >= y2) || (x2 <= x1 && y2 >= y1))
        .count();

    let p2 = pairs
        .iter()
        //  x1---------y1
        //           x2---------y2
        .filter(|(x1, y1, x2, y2)| x2 <= y1 && x1 <= y2)
        .count();

    return p2 as i32;
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code::AdventOfCodeInput;

    #[test]
    fn d4_aoc_input() {
        let aoc_input = AdventOfCodeInput::get_input(4);

        let answer = super::solve(&aoc_input.inp);

        assert_eq!(answer, 2);
    }
}
