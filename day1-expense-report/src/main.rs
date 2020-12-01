/**
the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
*/

fn main() {
    let input = include_str!("input.txt");

    println!("Answer: {:?}", expense_report(input));
    // correct answer: 539851
}

fn expense_report(input: &str) -> Result<i32, String> {
    /*
    we need to find two entries that sum to 2020.
    the naive solution is to compare every entry to every other entry.

    we can do better though:
    if we sort the input, we only need to check half the inputs
    against the other half, and we can use a binary search to
    rapidly find candidates

    the naive solution is, worst case, O(n^2)
    this solution is O(n log n) for the sort and O(n log n) for the search
    */

    // first, let's parse and sort our input
    let mut entries = input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    entries.sort_unstable();

    // for every element, binary search the rest of the list
    for (i, first) in entries.iter().enumerate() {
        let target = 2020 - first;
        let rest = &entries[i+1..];
        if let Ok(second_idx) = rest.binary_search(&target) {
            return Ok(first * rest[second_idx])
        }
    }
    Err("No value found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "\
            1721\n\
            979\n\
            366\n\
            299\n\
            675\n\
            1456\n";
        let answer = expense_report(input).unwrap();
        assert_eq!(answer, 514579);
    }
}