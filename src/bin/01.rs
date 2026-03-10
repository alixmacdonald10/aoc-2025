advent_of_code::solution!(1);

const MAX_VALUE: isize = 100;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                // get value for each turn
                let (direction, number) = line.split_at(1);
                let value = number.parse::<isize>().expect("failed to parse number");
                (value, direction)
            })
            .scan(50isize, |acc, (value, direction)| {
                *acc = match direction {
                    "L" => (*acc - value).rem_euclid(MAX_VALUE),
                    "R" => (*acc + value).rem_euclid(MAX_VALUE),
                    _ => unreachable!(),
                };
                Some(*acc)
            })
            .filter(|&val| 0 == val)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
