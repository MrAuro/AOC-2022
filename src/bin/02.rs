pub fn part_one(input: &str) -> Option<u32> {
    let split_input = input.split("\n");

    let mut score: u32 = 0;

    for s in split_input {
        if s.is_empty() {
            continue;
        }
        let opponent = s.chars().nth(0).unwrap();
        let me = s.chars().nth(2).unwrap();

        // A X Rock     1
        // B Y Paper    2
        // C Z Scissors 3

        if calc(&opponent) == calc(&me) {
            score += calc(&me) + 3;
            continue;
        }

        if opponent == 'A' {
            if me == 'Y' {
                score += calc(&me) + 6;
                continue;
            } else {
                score += calc(&me);
            }
        }

        if opponent == 'B' {
            if me == 'Z' {
                score += calc(&me) + 6;
                continue;
            } else {
                score += calc(&me);
            }
        }

        if opponent == 'C' {
            if me == 'X' {
                score += calc(&me) + 6;
                continue;
            } else {
                score += calc(&me);
            }
        }
    }

    Some(score)
}

pub fn calc(input: &char) -> u32 {
    match input {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let split_input = input.split('\n');

    let mut score = 0;
    for s in split_input {
        if s.is_empty() {
            continue;
        }

        let opponent = s.chars().nth(0).unwrap();
        let me = s.chars().nth(2).unwrap();

        // A X Rock     1
        // B Y Paper    2
        // C Z Scissors 3

        if me == 'X' {
            if opponent == 'A' {
                score += calc(&'Z');
            }

            if opponent == 'B' {
                score += calc(&'X');
            }

            if opponent == 'C' {
                score += calc(&'Y');
            }
        }

        if me == 'Y' {
            score += calc(&opponent) + 3;
        }

        if me == 'Z' {
            if opponent == 'A' {
                score += calc(&'Y') + 6;
            }

            if opponent == 'B' {
                score += calc(&'Z') + 6;
            }

            if opponent == 'C' {
                score += calc(&'X') + 6;
            }
        }
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
