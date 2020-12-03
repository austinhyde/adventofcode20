use lib::grid::Grid;

fn main() {
    /* Part 1:
        Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
        You make a map (your puzzle input) of the open squares (.) and trees (#) you can see.

        These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability,
        the same pattern repeats to the right many times

        You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

        Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
    */
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", count_trees(input, (3, 1)));
    // Correct answer: 244
}

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    Tree,
}
use Cell::*;

fn count_trees(input: &str, (right, down): (usize, usize)) -> usize {
    let grid: Grid<Cell> = Grid::from_lines_chars(input, |c| match c { '.' => Empty, _ => Tree});

    
    // the problem says to start at the top-left, but we already know that's empty
    // so start at the first point instead
    let mut x = right;
    let mut count = 0;
    for y in (down..grid.height()).step_by(down) {
        if grid[(x, y)] == Tree {
            count += 1
        }

        x = (x + right) % grid.width();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        assert_eq!(7, count_trees(input, (3, 1)));
    }
}