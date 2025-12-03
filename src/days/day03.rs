use std::cmp::Reverse;
use std::str::FromStr;

use crate::days::{DayError, DaySolver};

pub struct Day3Solver;

#[derive(Debug, Clone)]
pub struct Battery {
    joltage: u16,
    position: u16,
}

struct BatteryBank {
    batteries: Vec<Battery>,
}

impl FromStr for BatteryBank {
    type Err = DayError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let batteries: Vec<Battery> = line.chars().enumerate().map(|(index, c)| {
            Battery {
                joltage: c.to_digit(10).unwrap() as u16,
                position: index as u16,
            }
        }).collect();

        Ok(BatteryBank { batteries })
    }
}

impl BatteryBank {
    pub fn get_largest_joltage_with_two_batteries(&self) -> u16 {
        let mut batteries_sorted_with_joltage_without_last = self.batteries[..self.batteries.len() - 1].to_vec();
        batteries_sorted_with_joltage_without_last.sort_by_key(|battery| Reverse(battery.joltage), );

        let first_battery = batteries_sorted_with_joltage_without_last.first().unwrap();

        let mut batteries_sorted_with_joltage = self.batteries.clone();
        batteries_sorted_with_joltage.sort_by_key(|battery| Reverse(battery.joltage));

        let second_battery = batteries_sorted_with_joltage.iter().find(|battery| battery.position > first_battery.position).unwrap();

        let joltage = format!("{}{}", first_battery.joltage, second_battery.joltage);
        joltage.parse().unwrap()
    }

    pub fn get_largest_joltage_with_twelve_batteries(&self) -> u64 {
        let mut batteries_sorted_with_joltage = self.batteries.clone();
        batteries_sorted_with_joltage.sort_by_key(|battery| Reverse(battery.joltage));

        let amount_of_batteries = batteries_sorted_with_joltage.len();
        let mut joltages_to_find = 12;
        let mut max_starting_position = amount_of_batteries - joltages_to_find;
        let mut min_starting_position = 0;
        let mut found_batteries: Vec<Battery> = Vec::new();

        while joltages_to_find > 0 {
            let next_battery = batteries_sorted_with_joltage.iter().find(|battery| battery.position <= max_starting_position as u16 && battery.position >= min_starting_position).unwrap();

            found_batteries.push(next_battery.clone());

            joltages_to_find -= 1;
            max_starting_position += 1;
            min_starting_position = next_battery.position + 1;
        }


        let string: String = found_batteries.iter().map(|battery| char::from_digit(battery.joltage as u32, 10).unwrap()).collect();
        string.parse().unwrap()
    }
}

impl DaySolver for Day3Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let banks = input.lines().map(|line| line.parse()).collect::<Result<Vec<BatteryBank>, DayError>>()?;
        let joltages = banks.iter().map(|bank| bank.get_largest_joltage_with_two_batteries() as u32).collect::<Vec<u32>>();
        let sum: u32 = joltages.iter().sum();

        Ok(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let banks = input.lines().map(|line| line.parse()).collect::<Result<Vec<BatteryBank>, DayError>>()?;
        let joltages = banks.iter().map(|bank| bank.get_largest_joltage_with_twelve_batteries() as u64).collect::<Vec<u64>>();
        let sum: u64 = joltages.iter().sum();

        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input() -> &'static str {
        "987654321111111
811111111111119
234234234234278
818181911112111"
    }


    #[test]
    fn part1() {
        let solution = Day3Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "357")
    }

    #[test]
    fn part2() {
        let solution = Day3Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "3121910778619")
    }
}
