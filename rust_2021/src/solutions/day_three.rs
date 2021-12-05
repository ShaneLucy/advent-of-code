use crate::traits::Solution;

pub struct DayThree {
    pub input_data: std::string::String,
}

impl Solution for DayThree {
    fn part_one(&self) {
        let mut binary_numbers = vec![vec![]; 12];
        let mut most_common_bits: String = String::new();
        let mut least_common_bits: String = String::new();

        for string in self.input_data.split("\n") {
            for (pos, digit) in string.chars().map(|d| d.to_digit(10).unwrap()).enumerate() {
                binary_numbers[pos].push(digit as i32)
            }
        }

        for binary_number in binary_numbers {
            let mut total_zero_bits: i32 = 0;
            let mut total_one_bits: i32 = 0;
            for digit in binary_number {
                if digit == 0 {
                    total_zero_bits += 1
                } else {
                    total_one_bits += 1
                }
            }

            if total_one_bits > total_zero_bits {
                most_common_bits.push('1');
                least_common_bits.push('0')
            } else {
                most_common_bits.push('0');
                least_common_bits.push('1')
            }
        }

        let gamma_rate = isize::from_str_radix(&most_common_bits, 2).unwrap();
        let epsilon_rate = isize::from_str_radix(&least_common_bits, 2).unwrap();
        let power_output = gamma_rate * epsilon_rate;

        println!("Binary Diagnostic - Part 1: \n{}", power_output);
    }

    fn part_two(&self) {
      
    }
}
