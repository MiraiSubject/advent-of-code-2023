use std::fs;

fn sum_vec(vec: Vec<u32>) -> u32 {
    let mut sum = 0;
    for val in vec {
        sum += val;
    }
    sum
}

fn part_one(file: &str) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let part_one: u32 = contents
        .lines()
        .map(|line| {
            let vec_car = line
                .chars()
                .filter(|car| car.is_numeric())
                .collect::<Vec<_>>();
            vec_car[0].to_digit(10).unwrap() * 10 + vec_car[vec_car.len() - 1].to_digit(10).unwrap()
        })
        .sum();

    println!("Sum: {}", part_one);
}

const NUMBER_WORDS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_two(file: &str) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let part_two: u32 = contents
        .lines()
        .map(|line| {
            let mut vec_car = vec![];

            for (i, car) in line.chars().enumerate() {
                if let Some(number) = car.to_digit(10) {
                    vec_car.push(number);
                    continue;
                }

                for (u, number) in NUMBER_WORDS.into_iter().enumerate() {
                    let end_point = i + number.len();
                    if end_point <= line.len() && &line[i..end_point] == number {
                        vec_car.push(u as u32 + 1);
                    }
                }
            }

            vec_car.first().unwrap() * 10 + vec_car.last().unwrap()
        })
        .sum();

    println!("Sum 2: {}", part_two)
}

fn main() {
    part_one("puzz_input.txt");
    part_two("puzz_input.txt");
}
