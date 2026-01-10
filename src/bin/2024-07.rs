use std::fs;

fn equation_possible(test_value: u64, values: &[u64], with_concat: bool) -> bool{
    if values.len() == 1 {
        if values[0] == test_value {
            true
        } else  {
            false
        }
    } else {
        let val_1 = values[0];
        let val_2 = values[1];

        if val_1 > test_value {
            return false;
        }

        // Build new vectors: (val1 + val2) or (val1 * val2) followed by the rest
        let mut vec_plus = Vec::new();
        vec_plus.push(val_1 + val_2);
        vec_plus.extend_from_slice(&values[2..]);

        let mut vec_prod = Vec::new();
        vec_prod.push(val_1 * val_2);
        vec_prod.extend_from_slice(&values[2..]);

        if !with_concat {
            equation_possible(test_value, &vec_plus, with_concat)
            || equation_possible(test_value, &vec_prod, with_concat)
        } else {
            let mut vec_concat = Vec::new();
            let combined: u64 = format!("{}{}", val_1, val_2).parse().unwrap();

            vec_concat.push(combined);
            vec_concat.extend_from_slice(&values[2..]);
            equation_possible(test_value, &vec_plus, with_concat)
                || equation_possible(test_value, &vec_prod, with_concat)
                || equation_possible(test_value, &vec_concat, with_concat)
        }
    }     
}

fn main() {
    let input = fs::read_to_string("data/2024/07/input.txt").expect("Should have been able to read the file");
    let mut total_calibration_result: u64= 0;
    let mut total_calibration_result_concat: u64= 0;

    for line in input.lines(){
        let (test_value_str, numbers_str) = line.split_once(":").unwrap();
        // println!("{} {}", test_value_str, numbers_str);
        let test_value: u64 = test_value_str.parse().unwrap();
        let numbers: Vec<u64> = numbers_str
                .trim()
                .split(' ')
                .map(|s| s.trim().parse().unwrap())
                .collect();

        
        if equation_possible(test_value, &numbers, false){
            total_calibration_result += test_value;
            total_calibration_result_concat += test_value;
        } else if equation_possible(test_value, &numbers, true) {
            total_calibration_result_concat += test_value;
        }
    }

    println!("Part 1 Answer: {}", total_calibration_result);
    println!("Part 2 Answer: {}", total_calibration_result_concat);
}