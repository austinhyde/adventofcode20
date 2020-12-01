fn main() {
    let input = include_str!("input.txt");

    /* Part 1
        The Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
        Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
    */
    println!("Part 1 Answer: {:?}", expense_report(input, 2));
    // correct answer: 539851

    /* Part 2
        find three numbers in your expense report that meet the same criteria
    */
    println!("Part 2 Answer: {:?}", expense_report(input, 3));
    // correct answer: 212481360
}

fn expense_report(input: &str, n_candidates: usize) -> Option<i32> {
    /* part 1:
        we need to find two entries that sum to 2020.
        the naive solution is to compare every entry to every other entry.

        we can do better though:
        if we sort the input, we only need to check half the inputs
        against the other half, and we can use a binary search to
        rapidly find candidates

        the naive solution is, worst case, O(n^2)
        this solution is O(n log n) for the sort and O(n log n) for the search (O(n log n) total)

       part 2:
        we need to find three (or generally C) entries that sum to 2020

        observing that expense_report(entries, target, C=1) is just entries[entries.binary_search(target)],
        and expense_report(entries, target, C=2) is entries[i] * entries[entries[i+1..].binary_search(target-entries[i])]
        then we can generalize to

            expense_report(entries, target, C > 1) = entries[i] * expense_report(entries[i+1..], target-entries[i], C-1)
            expense_report(entries, target, 1) = entries[entries.binary_search(target)]
        
        this gives us O(n^(C-1) log n) performance, over the naive O(n^C)

        I'm sure there's more clever ways to do this, but I'm not sure how that would work, so we'll go with this implementation
    */

    // first, let's parse and sort our input
    let mut entries = input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    entries.sort_unstable();

    expense_report_impl(&entries, 2020, n_candidates)
}

fn expense_report_impl(entries: &[i32], target: i32, candidates: usize) -> Option<i32> {
    if candidates == 1 {
        if let Ok(idx) = entries.binary_search(&target) {
            return Some(entries[idx])
        }
        return None
    }

    // for every element, recurse on the rest of the list
    for (i, first) in entries.iter().enumerate() {
        if let Some(second) = expense_report_impl(&entries[i+1..], target - first, candidates-1) {
            return Some(first * second);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
            1721\n\
            979\n\
            366\n\
            299\n\
            675\n\
            1456\n";

    #[test]
    fn part1_example1() {
        let answer = expense_report(EXAMPLE_INPUT, 2).unwrap();
        assert_eq!(answer, 514579);
    }

    #[test]
    fn part2_example1() {
        let answer = expense_report(EXAMPLE_INPUT, 3).unwrap();
        assert_eq!(answer, 241861950);
    }
}