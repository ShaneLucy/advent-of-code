mod helpers;
mod traits;
mod solutions;

use crate::helpers::read_file::read_file;
use crate::traits::Solution;
use crate::solutions::day_one::DayOne;
use crate::solutions::day_two::DayTwo;

fn main() {
    
    println!("\n\n\nDay 1:");
    display_solution(&DayOne {
        input_data: read_file("day_one.txt"),
    });

    println!("\n\nDay 2:");
    display_solution(&DayTwo {
        input_data: read_file("day_two.txt"),
    });
}

fn display_solution(solution: &dyn Solution) {
    solution.result()
}
