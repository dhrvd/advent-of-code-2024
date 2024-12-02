use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    let total_distance = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right_freq: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        left.push(parts.next().unwrap().parse().unwrap());
        *right_freq
            .entry(parts.next().unwrap().parse().unwrap())
            .or_default() += 1;
    }

    let similarity_score: u32 = left
        .iter()
        .map(|l| l * right_freq.get(l).unwrap_or(&0))
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
