pub fn solution(input_data: std::string::String) {
    part_one(input_data.clone());
    part_two(input_data.clone())
}

fn part_one(input_data: std::string::String) {
    let split = input_data.split("\n");

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;

    for s in split {
        let instructions: Vec<&str> = s.split_whitespace().collect();
        let direction = instructions[0];
        let units = instructions[1].parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal_position += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => panic!("Why are we here?"),
        };
    }

    println!("{}", depth * horizontal_position)
}

fn part_two(input_data: std::string::String) {
    let split = input_data.split("\n");

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;
    let mut aim: i32 = 0;

    for s in split {
        let instructions: Vec<&str> = s.split_whitespace().collect();
        let direction = instructions[0];
        let units = instructions[1].parse::<i32>().unwrap();

        match direction {
            "forward" => {
                horizontal_position += units;
                depth += aim * units
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => panic!("Why are we here?"),
        };
    }

    println!("{}", depth * horizontal_position)
}
