use thiserror::Error;

use crate::days::{DayError, DaySolver};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day1Solver;

pub enum DialRotateDirection {
    Right(i16),
    Left(i16),
}

impl FromStr for DialRotateDirection {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let (direction, distance) = s.split_at(1) {
            let distance = distance
                .parse::<i16>()
                .map_err(|_| DayError::InvalidInputError(s.to_owned()))?;

            return match direction {
                "R" => Ok(DialRotateDirection::Right(distance)),
                "L" => Ok(DialRotateDirection::Left(distance)),
                _ => Err(DayError::InvalidInputError(s.to_owned())),
            };
        }

        Err(DayError::InvalidInputError(s.to_owned()))
    }
}

pub struct Dial {
    pub position: u8,
    pub password: u16,
    pub new_password: u16,
}

impl Dial {
    pub fn make_dial() -> Dial {
        Dial { position: 50, password: 0, new_password: 0 }
    }

    pub fn rotate(&mut self, direction: DialRotateDirection) {
        let new_position = match direction {
            DialRotateDirection::Right(distance) => self.position as i16 + distance % 100,
            DialRotateDirection::Left(distance) => self.position as i16 - distance % 100,
        };

        let how_many_times_points_at_0: i16 = match direction {
            DialRotateDirection::Right(distance) => {
                (self.position as i16 + distance) / 100
            },
            DialRotateDirection::Left(distance) => {
                if self.position == 0 {
                    distance / 100
                } else if distance >= self.position as i16 {
                    1 + (distance - self.position as i16) / 100
                } else {
                    0
                }
            },
        };

        self.new_password = self.new_password + how_many_times_points_at_0 as u16;

        match new_position {
            i16::MIN..=-1 => self.position = (new_position + 100) as u8,
            0..=99 => self.position = new_position as u8,
            100..=i16::MAX => self.position = (new_position - 100) as u8,
        }

        if self.position == 0 {
            self.password += 1;
        }
    }
}

impl DaySolver for Day1Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let instructions: Vec<DialRotateDirection> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<DialRotateDirection>, DayError>>(
        )?;
        let mut dial = Dial::make_dial();

        for instruction in instructions {
            dial.rotate(instruction);
        }

        Ok(dial.password.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let instructions: Vec<DialRotateDirection> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<DialRotateDirection>, DayError>>(
            )?;
        let mut dial = Dial::make_dial();

        for instruction in instructions {
            dial.rotate(instruction);
        }

        Ok(dial.new_password.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input() -> &'static str {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
    }

    #[test]
    fn test_dial() {
        let mut dial = Dial { position: 99, password: 0, new_password: 0};
        dial.rotate(DialRotateDirection::Left(99));
        assert_eq!(dial.position, 0);
    }

    #[test]
    fn part1() {
        let solution = Day1Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "3")
    }

    #[test]
    fn part2() {
        let solution = Day1Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "6")
    }
}
