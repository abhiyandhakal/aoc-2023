#[allow(dead_code)]
pub fn part1() {
    let input: Vec<String> = std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.to_string())
        .collect();

    let mut sum = 0;

    for row in input.iter() {
        let mut digits = Vec::new();

        for ch in row.clone().chars() {
            if ch.is_digit(10) {
                digits.push(ch.to_digit(10).unwrap());
            }
        }

        let mut pair = [0, 0];

        if digits.len() != 0 {
            pair[0] = digits[0];
            pair[1] = digits[digits.len() - 1];
        }

        let number = pair[0] * 10 + pair[1];
        sum += number;
    }

    println!("{sum}")
}

#[allow(dead_code)]
pub fn part2() {
    let input: Vec<String> = std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.to_string())
        .collect();

    let mut sum = 0;

    let digits_worded = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for row in input.iter() {
        let mut digits = Vec::new();

        for (i, ch) in row.char_indices() {
            if ch.is_digit(10) {
                digits.push(ch.to_digit(10).unwrap());
            }

            for (idx, digit) in digits_worded.iter().enumerate() {
                if i + digit.len() > row.len() {
                    continue;
                }

                if row[i..digit.len() + i].to_string() == digit.to_string() {
                    digits.push(idx as u32);
                }
            }
        }

        let mut pair = [0, 0];

        if digits.len() != 0 {
            pair[0] = digits[0];
            pair[1] = digits[digits.len() - 1];
        }

        let number = pair[0] * 10 + pair[1];
        sum += number;
    }

    println!("{sum}")
}
