use std::cmp::Ordering;

fn main() {
    let position = vec![(17, -12, 13), (2, 1, 1), (-1, -17, 7), (12, -14, 18)];
    let mut velocity = vec![(0, 0, 0); 4];

    let energy = simulate(&mut position.clone(), &mut velocity, 1000);
    println!("Total energy after 1000 steps = {}", energy);
    
    let t = repeat_time(position);
    println!("The state of the universe repeats at t = {}", t);
}

fn gravity(position: &[(i32, i32, i32)], velocity: &mut [(i32, i32, i32)]) {
    for (p, v) in position.iter().zip(velocity.iter_mut()) {
        for other_moon in position {
            v.0 += match p.0.cmp(&other_moon.0) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            v.1 += match p.1.cmp(&other_moon.1) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            v.2 += match p.2.cmp(&other_moon.2) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
        }
    }
}

fn apply_velocity(position: &mut [(i32, i32, i32)], velocity: &[(i32, i32, i32)]) {
    for (p, v) in position.iter_mut().zip(velocity) {
        p.0 += v.0;
        p.1 += v.1;
        p.2 += v.2;
    }
}

fn simulate(
    position: &mut[(i32, i32, i32)],
    velocity: &mut[(i32, i32, i32)],
    steps: usize,
) -> i32 {
    for _ in 0..steps {
        gravity(position, velocity);
        apply_velocity(position, velocity);
    }

    position.iter()
        .zip(velocity.iter())
        .map(|(&(x, y, z), &(vx, vy, vz))| (x.abs() + y.abs() + z.abs()) * (vx.abs() + vy.abs() + vz.abs()))
        .sum()
}

fn find_repeat(mut position: Vec<i32>) -> usize {
    let mut iterations = 0;
    let initial_position = position.clone();
    let mut velocity = vec![0; position.len()];
    let initial_velocity = velocity.clone();

    loop {
        iterations += 1;

        for (p1, v) in position.iter().zip(velocity.iter_mut()) {
            for p2 in &position {
                *v += match p1.cmp(p2) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
            }
        }

        for (p, v) in position.iter_mut().zip(&velocity) {
            *p += *v;
        }

        if position == initial_position && velocity == initial_velocity {
            break iterations;
        }
    }
}

fn repeat_time(position: Vec<(i32, i32, i32)>) -> usize {
    let x: Vec<i32> = position.iter().map(|&(x, _, _)| x).collect();
    let repeat_x = find_repeat(x);

    let y: Vec<i32> = position.iter().map(|&(_, y, _)| y).collect();
    let repeat_y = find_repeat(y);

    let z: Vec<i32> = position.iter().map(|&(_, _, z)| z).collect();
    let repeat_z = find_repeat(z);

    lcm(lcm(repeat_x, repeat_y), repeat_z)
}

// Find greatest common divisor of two integers by the Euclidean Algorithm.
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a
}

// Lowest common multiple
fn lcm(a: usize, b: usize) -> usize {
    (a * b)/gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_1_step() {
        let mut position = vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
        let mut velocity = vec![(0, 0, 0); 4];
        simulate(&mut position, &mut velocity, 1);

        assert_eq!(
            position,
            vec![(2, -1, 1), (3, -7, -4), (1, -7, 5), (2, 2, 0)]
        );

        assert_eq!(
            velocity,
            vec![(3, -1, -1), (1, 3, 3), (-3, 1, -3), (-1, -3, 1)]
        );
    }

    #[test]
    fn simulate_10_steps() {
        let mut position = vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
        let mut velocity = vec![(0, 0, 0); 4];
        let energy = simulate(&mut position, &mut velocity, 10);

        assert_eq!(
            position,
            vec![(2, 1, -3), (1, -8, 0), (3, -6, 1), (2, 0, 4)]
        );

        assert_eq!(
            velocity,
            vec![(-3, -2, 1), (-1, 1, 3), (3, 2, -3), (1, -1, -1)]
        );

        assert_eq!(energy, 179);
    }

    #[test]
    fn part2_example1() {
        let position = vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
        assert_eq!(repeat_time(position), 2772);
    }

    #[test]
    fn part2_example2() {
        let position = vec![(-8, -10, 0), (5, 5, 10), (2, -7, 3), (9, -8, -3)];
        assert_eq!(repeat_time(position), 4_686_774_924);
    }
}
