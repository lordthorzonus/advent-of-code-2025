use std::str::FromStr;

use crate::days::{DayError, DaySolver};

pub struct Day2Solver;

pub struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    pub fn get_invalid_ids(&self) -> Vec<u64> {
        (self.start..=self.end)
            .filter(|id| is_repeated_sequence(id))
            .collect()
    }
}

fn is_repeated_sequence(number: &u64) -> bool {
    let number_as_string = number.to_string();
    let length = number_as_string.len();

    if length % 2 != 0 {
        return false;
    }

    let midpoint = length / 2;
    number_as_string[..midpoint] == number_as_string[midpoint..]
}

impl FromStr for IdRange {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = match s.split_once("-") {
            Some((start, end)) => (start, end),
            None => return Err(DayError::InvalidInputError(s.to_owned())),
        };

        Ok(IdRange {
            start: start
                .parse()
                .map_err(|_| DayError::InvalidInputError(s.to_owned()))?,
            end: end
                .parse()
                .map_err(|_| DayError::InvalidInputError(s.to_owned()))?,
        })
    }
}

impl DaySolver for Day2Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let id_ranges: Vec<IdRange> = input
            .lines()
            .flat_map(|line| line.split(","))
            .map(|line| line.parse())
            .collect::<Result<Vec<IdRange>, DayError>>()?;

        let invalid_id_sum: u64 = id_ranges.iter().flat_map(|range| range.get_invalid_ids()).sum();
        Ok(invalid_id_sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        Ok(String::from("Hello"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input() -> &'static str {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    #[test]
    fn test_repeated_sequence() {
        assert!(is_repeated_sequence(&123123));
        assert!(!is_repeated_sequence(&12));
        assert!(is_repeated_sequence(&222222));
    }

    #[test]
    fn part1() {
        let solution = Day2Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "1227775554")
    }

    #[test]
    fn part2() {
        let solution = Day2Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "6")
    }
}
