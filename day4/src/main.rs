fn main() {
    /*
       Puzzle input:
       Minimum = 248345
       Maximum = 746315

       Rules:
       - Each digit must be greater than or equal to the previous digit.
       - At least one digit must be repeated.

       This implies that the smallest allowed code is 248888 and the largest
       is 699999.
    */

    let mut all_solutions = Vec::new();
    let mut num_codes = continuations(&[2, 4, 8], 6, &mut all_solutions)
        + continuations(&[2, 4, 9], 6, &mut all_solutions);
    for n in 5..=9 {
        num_codes += continuations(&[2, n], 6, &mut all_solutions);
    }

    for n in 3..=6 {
        num_codes += continuations(&[n], 6, &mut all_solutions);
    }

    println!("There are {} possible codes.", num_codes);

    let part2 = all_solutions
        .iter()
        .filter(|code| has_digit_appearing_exactly_twice(code))
        .count();

    println!(
        "{} of these have a digit that appears exactly twice.",
        part2
    );
}

fn has_repeats(code: &[u8]) -> bool {
    for index in 1..code.len() {
        if code[index] == code[index - 1] {
            return true;
        }
    }

    false
}

fn has_digit_appearing_exactly_twice(code: &[u8]) -> bool {
    let mut digit_frequency = vec![0; 10];
    for digit in code {
        digit_frequency[*digit as usize] += 1;
    }

    digit_frequency.into_iter().any(|n| n == 2)
}

fn continuations(code: &[u8], length: usize, all_solutions: &mut Vec<Vec<u8>>) -> u32 {
    let digit_count = code.len();
    let last_digit = code[digit_count - 1];

    // Are we choosing the last digit?
    if digit_count == length - 1 {
        if has_repeats(code) {
            // Last digit is greater than or equal to penultimate
            for next_digit in last_digit..=9 {
                let mut next_code = code.to_owned();
                next_code.push(next_digit);
                all_solutions.push(next_code);
            }

            return 10 - last_digit as u32;
        } else {
            // Last digit has to be same as penultimate
            let mut next_code = code.to_owned();
            next_code.push(last_digit);
            all_solutions.push(next_code);

            return 1;
        }
    }

    let mut possibilities = 0;
    for next_digit in last_digit..=9 {
        let mut next_code = code.to_owned();
        next_code.push(next_digit);

        possibilities += continuations(&next_code, length, all_solutions);
    }

    possibilities
}
