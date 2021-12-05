use crate::traits::Solution;

pub struct DayOne {
    pub input_data: std::string::String,
}

impl Solution for DayOne {
    fn part_one(&self) {
        let mut vec = Vec::new();

        for s in self.input_data.split("\n") {
            vec.push(s.parse::<i32>().unwrap())
        }

        let mut number_increases: i32 = 0;
        let mut previous_value: i32 = 0;

        for (pos, e) in vec.iter().enumerate() {
            if pos > 0 {
                if previous_value < *e {
                    number_increases += 1;
                }
            }
            previous_value = *e;
        }

        println!("Sonar Sweep - Part 1: \n{}\n", number_increases);
    }

    fn part_two(&self) {
        let mut vec = Vec::new();

        for s in self.input_data.split("\n") {
            vec.push(s.parse::<i32>().unwrap())
        }

        let mut number_three_measurement_increases: i32 = 0;
        let mut first_three_measurement_window = vec![];
        let mut second_three_measurement_window = vec![];

        for (pos, e) in vec.iter().enumerate() {
            if first_three_measurement_window.len() < 3 {
                first_three_measurement_window.push(*e);
            }

            if second_three_measurement_window.len() == 3
                && first_three_measurement_window.len() == 3
            {
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

        println!(
            "Sonar Sweep - Part 2: \n{}\n",
            number_three_measurement_increases
        );
    }
}
