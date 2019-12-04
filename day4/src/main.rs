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
    
    let mut num_codes = continuations(&[2, 4, 8], 6)
        + continuations(&[2, 4, 9], 6);
    
    for n in 5..=9 {
        num_codes += continuations(&[2, n], 6);
    }

    for n in 3..=6 {
        num_codes += continuations(&[n], 6);
    }

    println!("There are {} possible codes.", num_codes);
}

fn has_repeats(code: &[u8]) -> bool {
    for index in 1..code.len() {
        if code[index] == code[index - 1] {
            return true;
        }
    }

    false
}

fn continuations(code: &[u8], length: usize) -> u32 {
    let digit_count = code.len();
    let last_digit = code[digit_count - 1];

    // Are we choosing the last digit?
    if digit_count == length - 1 { //&& !has_repeats(code) {
        if has_repeats(code) {
            // Last digit is greater than or equal to penultimate
            for next_digit in last_digit..=9 {
                let mut next_code = code.to_owned();
                next_code.push(next_digit);
                println!("{:?}", next_code);
            }

            return 10 - last_digit as u32;
        } else {
            // Last digit has to be same as penultimate
            let mut next_code = code.to_owned();
            next_code.push(last_digit);
            println!("{:?}", next_code);

            return 1;
        }
    }

    let mut possibilities = 0;
    

    for next_digit in last_digit..=9 {
        let mut next_code = code.to_owned();
        next_code.push(next_digit);

        possibilities += continuations(&next_code, length);
    }

    possibilities
}