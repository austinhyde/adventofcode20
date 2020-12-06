use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    // Correct answer: 6885
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:

*/
fn part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|s| {
            s.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

/** Part 2:

*/
fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 11;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 0;
        assert_eq!(expected, part2(input));
    }
}