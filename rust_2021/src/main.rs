mod helpers;
mod solutions;
mod traits;

use crate::helpers::read_file::read_file;
use crate::solutions::day_one::DayOne;
use crate::solutions::day_three::DayThree;
use crate::solutions::day_two::DayTwo;
use crate::traits::Solution;

fn main() {
    println!("\n\n\nDay 1:");
    display_solution(&DayOne {
        input_data: read_file("day_one.txt"),
    });

    println!("\n\nDay 2:");
    display_solution(&DayTwo {
        input_data: read_file("day_two.txt"),
    });

    println!("\n\nDay 3:");
    display_solution(&DayThree {
        input_data: read_file("day_three.txt"),
    });
}

fn display_solution(solution: &dyn Solution) {
    solution.result()
}
