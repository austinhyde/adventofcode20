use std::str::FromStr;

fn main() {
    /* Part 1
        Each line gives the password policy and then the password. The password policy indicates the lowest
        and highest number of times a given letter must appear for the password to be valid. For example,
        `1-3 a` means that the password must contain a at least 1 time and at most 3 times.

        How many passwords are valid according to their policies?
    */
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", count_invalid_passwords(input, range_policy));
    // Incorrect, too low: 385 (missed the inclusive lower bound on policy)
    // Correct answer: 550

    /* Part 2:
        Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on.
        (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
        Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.    

        How many passwords are valid according to the new interpretation of the policies?
    */
    println!("Part 2 Answer: {}", count_invalid_passwords(input, position_policy))
    // Correct answer: 634
}

fn count_invalid_passwords(input: &str, validator: impl Fn(&Policy, &str) -> bool) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Entry>().unwrap())
        .filter(|e| validator(&e.policy, &e.password))
        .count()
}

fn range_policy(policy: &Policy, password: &str) -> bool {
    let mut count = 0;
    for c in password.chars() {
        if c == policy.letter {
            count += 1;
        }
        if count > policy.n2 {
            return false
        }
    }
    count >= policy.n1
}

fn position_policy(policy: &Policy, password: &str) -> bool {
    let pos1 = password.chars().nth(policy.n1 as usize - 1).unwrap() == policy.letter;
    let pos2 = password.chars().nth(policy.n2 as usize - 1).unwrap() == policy.letter;
    pos1 != pos2
}

struct Entry {
    policy: Policy,
    password: String,
}

struct Policy {
    n1: u32,
    n2: u32,
    letter: char,
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
                n1: minmax[0],
                n2: minmax[1],
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
        let n = count_invalid_passwords(EXAMPLE_1_INPUT, range_policy);
        assert_eq!(n, 2);
    }

    #[test]
    fn part2_example1() {
        let n = count_invalid_passwords(EXAMPLE_1_INPUT, position_policy);
        assert_eq!(n, 1);
    }
}