fn main() {
    let input: Vec<u8> = include_str!("puzzle_input.txt")
            .chars()
            .map(|c| c as u8 - b'0')
            .collect();
        
    let result = repeat_fft(input, 100);
    for digit in &result[0..8] {
        print!("{}", digit);
    }
    println!();
}

fn pattern(index: usize) -> Vec<i32> {
    let mut p = Vec::with_capacity(index * 4);

    for &coefficient in &[0, 1, 0, -1] {
        for _ in 0..index {
            p.push(coefficient);
        }
    }

    p
}

// The "Flawed Frequency Transmission"
fn fft(digits: &[u8]) -> Vec<u8> {
    let length = digits.len();
    let mut output = Vec::with_capacity(length);

    for index in 0..length {
        let coefficients = pattern(index + 1);
        let value: i32 = digits
            .iter()
            .zip(coefficients.into_iter().cycle().skip(1))
            .map(|(&a, b)| (a as i32) * (b as i32))
            .sum();
        output.push((value.abs() % 10) as u8);
    }

    output
}

fn repeat_fft(mut digits: Vec<u8>, phases: usize) -> Vec<u8> {
    for _ in 0..phases {
        digits = fft(&digits);
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_example1() {
        assert_eq!(fft(&[1, 2, 3, 4, 5, 6, 7, 8]), vec![4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn fft_example1_4phases() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(repeat_fft(input, 4), vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn fft_example2() {
        let input: Vec<u8> = String::from("80871224585914546619083218645595")
            .chars()
            .map(|c| c as u8 - b'0')
            .collect();
        
        let result = repeat_fft(input, 100);
        
        assert_eq!(&result[0..8], &[2, 4, 1, 7, 6, 1, 7, 6]);
    }
}
