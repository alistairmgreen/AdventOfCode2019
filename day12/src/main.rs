use std::cmp::Ordering;

fn main() {
    let mut position = vec![(17, -12, 13), (2, 1, 1), (-1, -17, 7), (12, -14, 18)];
    let mut velocity = vec![(0, 0, 0); 4];

    let energy = simulate(&mut position, &mut velocity, 1000);
    println!("Total energy after 1000 steps = {}", energy);   
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
}
