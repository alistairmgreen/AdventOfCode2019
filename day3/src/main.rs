use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::BufRead;
use day3::*;


fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("puzzle_input.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut wires = Vec::new();
    for line in reader.lines() {
            let line = line?;
            wires.push(parse_wire(&line)?);
    }

    match solve_part1(&wires) {
        Some(distance) => {
            println!("Manhattan distance to closest intersection = {}", distance);
        }
        None => {
            println!("Wires don't cross.");
        }
    }

    match solve_part2(&wires) {
        Some(steps) => {
            println!("Minimum number of steps to an intersection = {}", steps);
        }
        None => {
            println!("Wires don't cross.");
        }
    }

   Ok(())
}

fn parse_wire(wire: &str) -> Result<Vec<WireSegment>, StepParseError> {
    wire.split(',')
        .map(|segment| segment.parse())
        .collect()
}

fn solve_part1(wires: &[Vec<WireSegment>]) -> Option<i32> {
    let mut grid = HashMap::new();

    for wire in wires {
        let visited = visited_points(&wire);

        for (position, _) in visited {
            *grid.entry(position).or_insert(0) += 1;
        }
    }

    let closest_intersection = grid.into_iter()
        .filter_map(|(position, count)|  if count > 1 { Some(position) } else { None })
        .min_by_key(|position| position.manhattan());

    closest_intersection.map(|position| position.manhattan())
}

fn solve_part2(wires: &[Vec<WireSegment>]) -> Option<u32> {
    let mut grid = HashMap::new();

    for wire in wires {
        let visited = visited_points(&wire);

        for (position, steps) in visited {
            grid.entry(position).or_insert_with(|| {Vec::new()}).push(steps);
        }
    }

    grid.into_iter()
        .filter_map(|(_, steps)| if steps.len() > 1 { Some(steps.into_iter().sum()) } else { None })
        .min()
}

fn visited_points(segments: &[WireSegment]) -> HashMap<Point, u32> {
    let mut visited = HashMap::new();
    let mut position = Point { x: 0, y: 0 };
    let mut steps = 0;

    for segment in segments {
        let displacement = segment.direction.displacement();
        for _ in 0..segment.distance {
            steps += 1;
            position += displacement;
            visited.entry(position.clone())
                .or_insert(steps);
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
            parse_wire("R8,U5,L5,D3").unwrap(),
            parse_wire("U7,R6,D4,L4").unwrap()
        ];

        assert_eq!(solve_part1(&wires), Some(6))
    }

    #[test]
    fn part1_example2() {
        let wires = vec![
            parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L7").unwrap(),
            parse_wire("U62,R66,U55,R34,D71,R55,D58,R83").unwrap()
        ];

        assert_eq!(solve_part1(&wires), Some(159))
    }

    #[test]
    fn part1_example3() {
        let wires = vec![
            parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
            parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()
        ];

        assert_eq!(solve_part1(&wires), Some(135))
    }

    #[test]
    fn part2_example1() {
        let wires = vec![
            parse_wire("R8,U5,L5,D3").unwrap(),
            parse_wire("U7,R6,D4,L4").unwrap()
        ];

        assert_eq!(solve_part2(&wires), Some(30))
    }

    #[test]
    fn part2_example2() {
        let wires = vec![
            parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L7").unwrap(),
            parse_wire("U62,R66,U55,R34,D71,R55,D58,R83").unwrap()
        ];

        assert_eq!(solve_part2(&wires), Some(610))
    }

    #[test]
    fn part2_example3() {
        let wires = vec![
            parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
            parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()
        ];

        assert_eq!(solve_part2(&wires), Some(410))
    }
}