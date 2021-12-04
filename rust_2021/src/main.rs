mod day_one;
mod day_two;
mod helpers;

fn main() {
    day_one::solution(helpers::read_file("day_one.txt"));

    day_two::solution(helpers::read_file("day_two.txt"));
}
