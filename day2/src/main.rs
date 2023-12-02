#[allow(unused_variables)]

fn part_one(file: &str) -> u32 {
    let input = std::fs::read_to_string(file).expect("Unable to read file");

    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    input
        .split('\n')
        .map(|line| {
            let game_id = line
                .split_once(": ")
                .unwrap()
                .0
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            println!("Game ID: {}", game_id);

            let is_possible = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .flat_map(|set| {
                    let t = set.split(", ").map(|cube| {
                        (
                            cube.split(' ').nth(0).unwrap().parse::<u32>().unwrap(),
                            cube.split(' ').nth(1).unwrap(),
                        )
                    });
                    t
                })
                .all(|(count, colour)| match colour {
                    "blue" => count <= max_blue_cubes,
                    "red" => count <= max_red_cubes,
                    "green" => count <= max_green_cubes,
                    _ => panic!("ok"),
                });
            if is_possible {
                game_id
            } else {
                0u32
            }
        })
        .sum()
}

fn part_two(file: &str) -> u32 {
    let input = std::fs::read_to_string(file).expect("Unable to read file");

    input
        .split('\n')
        .map(|line| {
            let (max_red, max_green, max_blue) = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .flat_map(|set| {
                    let t = set.split(", ").map(|cube| {
                        (
                            cube.split(' ').nth(0).unwrap().parse::<u32>().unwrap(),
                            cube.split(' ').nth(1).unwrap(),
                        )
                    });
                    t
                })
                .fold(
                    (0u32, 0u32, 0u32),
                    |(red, green, blue), (count, color)| match color {
                        "blue" => (red, green, blue.max(count)),
                        "red" => (red.max(count), green, blue),
                        "green" => (red, green.max(count), blue),
                        _ => panic!(),
                    },
                );

            max_red * max_blue * max_green
        })
        .sum()
}

fn main() {
    println!("Hello, world!");
    println!("Sum of possible IDs {}", part_one("puzz_input.txt"));
    println!("Sum of power of sets: {}", part_two("puzz_input.txt"));
}
