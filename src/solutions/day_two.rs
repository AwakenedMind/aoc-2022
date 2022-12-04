pub fn solve(data: &str) -> i32 {
    let mut rounds: Vec<String> = vec![];

    // my shapes
    let rock: char = 'X';
    let paper: char = 'Y';
    let scissors: char = 'Z';

    // my score translation
    let rock_score = 1;
    let paper_score = 2;
    let scissors_score = 3;

    // outcome translation
    let rock_win = 6;
    let rock_draw = 3;
    let rock_lose = 0;

    let paper_win = 6;
    let paper_draw = 3;
    let paper_lose = 0;

    let scissors_win = 6;
    let scissors_draw = 3;
    let scissors_lose = 0;

    for line in data.lines() {
        rounds.push(line.split(" ").collect())
    }

    let mut total_score: i32 = 0;

    rounds.iter().for_each(|s| {
        let mut my_pos = s.chars().nth(1).unwrap();
        let mut enemy_pos = s.chars().nth(0).unwrap();

        let mut my_pos_score: i32 = 0;

        match enemy_pos {
            'A' => {
                enemy_pos = rock;
            }
            'B' => {
                enemy_pos = paper;
            }
            'C' => {
                enemy_pos = scissors;
            }
            _ => (),
        };

        match my_pos {
            // lose
            'X' => {
                if enemy_pos == paper {
                    my_pos = rock;
                    my_pos_score = rock_score;
                } else if enemy_pos == scissors {
                    my_pos = paper;
                    my_pos_score = paper_score
                } else if enemy_pos == rock {
                    my_pos = scissors;
                    my_pos_score = scissors_score;
                }
            }
            // draw
            'Y' => {
                if enemy_pos == paper {
                    my_pos = paper;
                    my_pos_score = paper_score;
                } else if enemy_pos == scissors {
                    my_pos = scissors;
                    my_pos_score = scissors_score;
                } else if enemy_pos == rock {
                    my_pos = rock;
                    my_pos_score = rock_score;
                }
            }
            // win
            'Z' => {
                if enemy_pos == paper {
                    my_pos = scissors;
                    my_pos_score = scissors_score;
                } else if enemy_pos == scissors {
                    my_pos = rock;
                    my_pos_score = rock_score;
                } else if enemy_pos == rock {
                    my_pos = paper;
                    my_pos_score = paper_score;
                }
            }
            _ => (),
        };

        // draw conditions
        if my_pos == enemy_pos {
            if my_pos == rock {
                my_pos_score = rock_draw + my_pos_score;
            } else if my_pos == paper {
                my_pos_score = paper_draw + my_pos_score;
            } else if my_pos == scissors {
                my_pos_score = scissors_draw + my_pos_score;
            }
        }

        // win conditions
        if my_pos == rock && enemy_pos == scissors {
            my_pos_score = my_pos_score + rock_win;
        } else if my_pos == paper && enemy_pos == rock {
            my_pos_score = my_pos_score + paper_win;
        } else if my_pos == scissors && enemy_pos == paper {
            my_pos_score = scissors_win + my_pos_score;
        }

        // lose conditions
        if my_pos == rock && enemy_pos == paper {
            my_pos_score = my_pos_score + rock_lose;
        } else if my_pos == scissors && enemy_pos == rock {
            my_pos_score = my_pos_score + scissors_lose;
        } else if my_pos == paper && enemy_pos == scissors {
            my_pos_score = my_pos_score + paper_lose;
        }

        total_score = total_score + my_pos_score;
    });

    total_score
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code::AdventOfCodeInput;
    use indoc::indoc;

    #[test]
    fn d2_aoc_input() {
        let aoc_input = AdventOfCodeInput::get_input(2);

        let answer = super::solve(&aoc_input.inp);

        assert_eq!(answer, 14060);
    }

    #[test]
    fn d2_aoc_test_input() {
        let aoc_input = indoc! {"
        A Y
        B X
        C Z
        "};

        let answer = super::solve(&aoc_input);

        assert_eq!(answer, 12);
    }
}
