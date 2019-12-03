use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::BufRead;
use day3::*;


fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("puzzle_input.txt")?;
    let reader = io::BufReader::new(file);
    
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

    match solve_part1(&lines)? {
        Some(distance) => {
            println!("Manhattan distance to closest intersection = {}", distance);
        }
        None => {
            println!("Wires don't cross.");
        }
    }

   Ok(())
}

fn solve_part1(lines: &[String]) -> Result<Option<i32>, StepParseError> {
    let mut wires = HashMap::new();

    for line in lines {
        let steps = line.split(',')
        .map(|step| step.parse::<Step>())
        .collect::<Result<Vec<Step>, StepParseError>>()?;

        let visited = visited_points(&steps);

        for position in visited.into_iter() {
            *wires.entry(position).or_insert(0) += 1;
        }
    }

    let closest_intersection = wires.into_iter()
        .filter_map(|(position, count)|  if count > 1 { Some(position) } else { None })
        .min_by_key(|position| position.manhattan());

    Ok(closest_intersection.map(|position| position.manhattan()))
}

fn visited_points(steps: &[Step]) -> HashSet<Point> {
    let mut visited = HashSet::new();

    let mut position = Point { x: 0, y: 0 };

    for step in steps {
        let displacement = step.direction.displacement();
        for _ in 0..step.distance {
            position += displacement;
            visited.insert(position.clone());
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let wires = vec![
            String::from("R8,U5,L5,D3"),
            String::from("U7,R6,D4,L4")
        ];

        assert_eq!(solve_part1(&wires).unwrap(), Some(6))
    }

    #[test]
    fn part1_example2() {
        let wires = vec![
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L7"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83")
        ];

        assert_eq!(solve_part1(&wires).unwrap(), Some(159))
    }

    #[test]
    fn part1_example3() {
        let wires = vec![
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ];

        assert_eq!(solve_part1(&wires).unwrap(), Some(135))
    }
}