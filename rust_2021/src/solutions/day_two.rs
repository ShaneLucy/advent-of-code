use crate::traits::Solution;

pub struct DayTwo {
    pub input_data: std::string::String,
}

impl Solution for DayTwo {
    fn part_one(&self){
         let split = self.input_data.split("\n");

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

    println!("Dive - Part 1: \n{}\n", depth * horizontal_position)
    }

    fn part_two(&self){
        let split = self.input_data.split("\n");

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

    println!("Dive - Part 2: \n{}\n", depth * horizontal_position)
    }

}

