use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let split_input = input.split("\n");

    let mut max: i32 = 0;
    let mut cur: i32 = 0;
    for i in split_input {
        if i.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            cur += i.parse::<i32>().unwrap();
        }
    }

    Some(max as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: HashMap<usize, u32> = HashMap::new();
    let split_input = input.split("\n");

    let mut elf = 0;
    for i in split_input {
        if i.is_empty() {
            elf += 1;
        } else {
            elves
                .entry(elf)
                .and_modify(|e| *e += i.parse::<u32>().unwrap())
                .or_insert(i.parse::<u32>().unwrap());
        }
    }

    let mut top_3: Vec<u32> = elves.values().cloned().collect();
    top_3.sort();
    top_3.reverse();
    Some(top_3[0] + top_3[1] + top_3[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
