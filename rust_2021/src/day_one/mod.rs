pub fn solution(input_data: std::string::String) {
    let mut vec = Vec::new();

    for s in input_data.split("\n") {
        vec.push(s.parse::<i32>().unwrap())
    }

    part_one(vec.clone());
    part_two(vec.clone());
}

fn part_one(input: std::vec::Vec<i32>) {
    let mut number_increases: i32 = 0;
    let mut previous_value: i32 = 0;

    for (pos, e) in input.iter().enumerate() {
        if pos > 0 {
            if previous_value < *e {
                number_increases += 1;
            }
        }
        previous_value = *e;
    }

    println!("{}", number_increases);
}

/**
 * TODO fix this, generating incorrect result
 */
fn part_two(input: std::vec::Vec<i32>) {
    let mut number_three_measurement_increases: i32 = 0;
    let mut first_three_measurement_window = vec![];
    let mut second_three_measurement_window = vec![];

    for (pos, e) in input.iter().enumerate() {
        if first_three_measurement_window.len() < 3 {
            first_three_measurement_window.push(*e);
        }

        if second_three_measurement_window.len() == 3 && first_three_measurement_window.len() == 3 {
            let first_window_total: i32 = first_three_measurement_window.iter().sum();
            let second_window_total: i32 = second_three_measurement_window.iter().sum();

            if first_window_total > second_window_total {
                number_three_measurement_increases += 1;
            }

            first_three_measurement_window = second_three_measurement_window.clone();
            second_three_measurement_window.remove(0);
        }
        if pos > 0 {
            if second_three_measurement_window.len() < 3 {
                second_three_measurement_window.push(*e);
            }
        }
    }

    println!("{}", number_three_measurement_increases);
}
