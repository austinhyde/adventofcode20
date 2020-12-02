use std::str::FromStr;

fn main() {
    /* Part 1
        Each line gives the password policy and then the password. The password policy indicates the lowest
        and highest number of times a given letter must appear for the password to be valid. For example,
        `1-3 a` means that the password must contain a at least 1 time and at most 3 times.

        How many passwords are valid according to their policies?
    */
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", count_invalid_passwords(input))
    // Incorrect, too low: 385 (missed the inclusive lower bound on policy)
    // Correct answer: 550
}

fn count_invalid_passwords(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Entry>().unwrap())
        .filter(|e| e.check())
        .count()
}

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn check(&self) -> bool {
        self.policy.check(&self.password)
    }
}

#[derive(Debug)]
struct Policy {
    min: u32,
    max: u32,
    letter: char,
}

impl Policy {
    fn check(&self, password: &str) -> bool {
        let mut count = 0;
        for c in password.chars() {
            if c == self.letter {
                count += 1;
            }
            if count > self.max {
                return false
            }
        }
        count >= self.min
    }
}

impl FromStr for Entry {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1-3 a: asdf -> "1-3", "a:", "asdf"
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let minmax = parts[0]
            .split('-')
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>,_>>()
            .map_err(|e| e.to_string())?;
        
        let letter = parts[1].chars().next().ok_or_else(|| "no letter".to_string())?;

        Ok(Entry {
            policy: Policy {
                min: minmax[0],
                max: minmax[1],
                letter,
            },
            password: parts[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1_INPUT: &str = "\
        1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\n\
    ";
    #[test]
    fn part1_example1() {
        let n = count_invalid_passwords(EXAMPLE_1_INPUT);
        assert_eq!(n, 2);
    }
}