use std::collections::HashSet;

fn main() {
    let grid = Grid::new(include_str!("inputs/puzzle_input.txt"));
    let ((x, y), visible) = grid.best_position().unwrap();

    println!("From position ({}, {}), we can see {} asteroids.", x, y, visible);
}

// Find greatest common divisor of two integers by the Euclidean Algorithm.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a
}

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    positions: HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    pub fn new(grid: &str) -> Grid {
        let mut positions = HashSet::new();

        for (y, line) in grid.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character == '#' {
                    positions.insert((x as i32, y as i32));
                }
            }
        }
    
        let max_x = positions.iter().map(|&(x, _)| x).max().unwrap_or(0);
        let max_y = positions.iter().map(|&(_, y)| y).max().unwrap_or(0);
    
        Grid {
            positions,
            max_x,
            max_y,
        }
    }

    pub fn best_position(&self) -> Option<((i32, i32), usize)> {
        self.positions.iter()
            .map(|p| (*p, self.count_visible(*p)))
            .max_by_key(|(_, visible)| *visible)
    }

    fn count_visible(&self, position: (i32, i32)) -> usize {
        let mut hidden = HashSet::with_capacity(self.positions.len());
        hidden.insert(position);
    
        for &asteroid in &self.positions {
            if hidden.contains(&asteroid) {
                continue;
            }
    
            let displacement = (asteroid.0 - position.0, asteroid.1 - position.1);
            let factor = gcd(displacement.0.abs(), displacement.1.abs());
            let hidden_vector = (displacement.0 / factor, displacement.1 / factor);
            
            let mut hidden_position = (asteroid.0 + hidden_vector.0, asteroid.1 + hidden_vector.1);
            while self.contains_position(hidden_position) {
                hidden.insert(hidden_position);
                hidden_position = (hidden_position.0 + hidden_vector.0, hidden_position.1 + hidden_vector.1);
            }
        }
    
        self.positions.difference(&hidden).count()
    }

    fn contains_position(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;

        x >= 0 && x <= self.max_x && y >= 0 && y <= self.max_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_positive() {
        assert_eq!(gcd(12, 4), 4);
    }

    #[test]
    fn test_gcd_negative() {
        assert_eq!(gcd(12, -4), -4);
        assert_eq!(gcd(-12, -4), -4);
        assert_eq!(gcd(-12, 4), 4);
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn example_1_count_visible() {
        let grid = Grid::new(include_str!("inputs/example1.txt"));
        assert_eq!(grid.count_visible((3, 4)), 8);
    }

    #[test]
    fn example1() {
        let grid = Grid::new(include_str!("inputs/example1.txt"));
        let solution = grid.best_position();
        assert_eq!(solution, Some(((3, 4), 8)));
    }

    #[test]
    fn example2() {
        let grid = Grid::new(include_str!("inputs/example2.txt"));
        let solution = grid.best_position();
        assert_eq!(solution, Some(((5, 8), 33)));
    }

    #[test]
    fn example2_count_visible() {
        let grid = Grid::new(include_str!("inputs/example2.txt"));
        let visible = grid.count_visible((5, 8));
        assert_eq!(visible, 33);
    }

    #[test]
    fn example3() {
        let grid = Grid::new(include_str!("inputs/example3.txt"));
        let solution = grid.best_position();
        assert_eq!(solution, Some(((1, 2), 35)));
    }

    #[test]
    fn example3_count_visible() {
        let grid = Grid::new(include_str!("inputs/example3.txt"));
        let visible = grid.count_visible((1, 2));
        assert_eq!(visible, 35);
    }

    #[test]
    fn example4() {
        let grid = Grid::new(include_str!("inputs/example4.txt"));
        let solution = grid.best_position();
        assert_eq!(solution, Some(((6, 3), 41)));
    }

    #[test]
    fn example4_count_visible() {
        let grid = Grid::new(include_str!("inputs/example4.txt"));
        let visible = grid.count_visible((6, 3));
        assert_eq!(visible, 41);
    }

    #[test]
    fn example5() {
        let grid = Grid::new(include_str!("inputs/example5.txt"));
        let solution = grid.best_position();
        assert_eq!(solution, Some(((11, 13), 210)));
    }

    #[test]
    fn example5_count_visible() {
        let grid = Grid::new(include_str!("inputs/example5.txt"));
        let visible = grid.count_visible((11, 13));
        assert_eq!(visible, 210);
    }
}
