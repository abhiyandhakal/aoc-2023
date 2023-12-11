#[allow(dead_code)]
pub fn part1() {
    let red_cubes: u8 = 12;
    let green_cubes: u8 = 13;
    let blue_cubes: u8 = 14;

    let input: Vec<_> = std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.to_string())
        .collect();

    let input_splited: Vec<_> = input
        .iter()
        .map(|f| f.split(':').collect::<Vec<_>>())
        .collect();

    let mut combos_list: Vec<Vec<(u8, u8, u8)>> = Vec::new();
    let mut invalid_sum = 0;

    for row in &input_splited {
        if row.len() == 2 {
            let line = row[1];

            let line_splitted: Vec<_> = line
                .split(';')
                .collect::<Vec<_>>()
                .iter()
                .map(|f| f.split(',').collect::<Vec<_>>())
                .collect();

            let mut combos: Vec<(u8, u8, u8)> = vec![];

            for draw in &line_splitted {
                let mut combo: (u8, u8, u8) = (0, 0, 0);

                for i in draw.iter() {
                    let mut digits = vec![];
                    let mut number = 0;

                    for ch in i.chars() {
                        if ch.is_digit(10) {
                            digits.push(ch.to_digit(10).unwrap());
                        }
                    }

                    for (idx, digit) in digits.iter().enumerate() {
                        number += *digit as u8 * 10_u8.pow((digits.len() - 1 - idx) as u32);
                    }

                    if i.contains("red") {
                        combo.0 = number;
                    } else if i.contains("green") {
                        combo.1 = number;
                    } else if i.contains("blue") {
                        combo.2 = number;
                    }
                }

                combos.push(combo);
            }

            combos_list.push(combos);
        }
    }

    for (idx, combos) in combos_list.iter().enumerate() {
        for combo in combos {
            if combo.0 > red_cubes || combo.1 > green_cubes || combo.2 > blue_cubes {
                invalid_sum += idx + 1;
                println!("{}", idx + 1);
                break;
            }
        }
    }

    let sum: usize = combos_list
        .iter()
        .enumerate()
        .map(|f| f.0 + 1)
        .sum::<usize>()
        - invalid_sum;

    println!("{sum}");
}

pub fn part2() {
    let red_cubes: u8 = 12;
    let green_cubes: u8 = 13;
    let blue_cubes: u8 = 14;

    let input: Vec<_> = std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.to_string())
        .collect();

    let input_splited: Vec<_> = input
        .iter()
        .map(|f| f.split(':').collect::<Vec<_>>())
        .collect();

    let mut combos_list: Vec<Vec<(u8, u8, u8)>> = Vec::new();
    let mut power_sum = 0;

    for row in &input_splited {
        if row.len() == 2 {
            let line = row[1];

            let line_splitted: Vec<_> = line
                .split(';')
                .collect::<Vec<_>>()
                .iter()
                .map(|f| f.split(',').collect::<Vec<_>>())
                .collect();

            let mut combos: Vec<(u8, u8, u8)> = vec![];

            for draw in &line_splitted {
                let mut combo: (u8, u8, u8) = (0, 0, 0);

                for i in draw.iter() {
                    let mut digits = vec![];
                    let mut number = 0;

                    for ch in i.chars() {
                        if ch.is_digit(10) {
                            digits.push(ch.to_digit(10).unwrap());
                        }
                    }

                    for (idx, digit) in digits.iter().enumerate() {
                        number += *digit as u8 * 10_u8.pow((digits.len() - 1 - idx) as u32);
                    }

                    if i.contains("red") {
                        combo.0 = number;
                    } else if i.contains("green") {
                        combo.1 = number;
                    } else if i.contains("blue") {
                        combo.2 = number;
                    }
                }

                combos.push(combo);
            }

            combos_list.push(combos);
        }
    }

    for combos in combos_list.iter().enumerate() {
        let mut max = (0, 0, 0);

        for combo in combos.1.iter() {
            if combo.0 > max.0 {
                max.0 = combo.0;
            }
            if combo.1 > max.1 {
                max.1 = combo.1;
            }
            if combo.2 > max.2 {
                max.2 = combo.2;
            }
        }

        power_sum += max.0 as usize * max.1 as usize * max.2 as usize;
    }

    println!("{power_sum}");
}
