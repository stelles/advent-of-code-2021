use crate::{
    utils
};

fn bit_to_number(bits: String) -> i64 {
    let bits = String::from(bits);
    let num = i64::from_str_radix(&bits.to_string(), 2).expect("Bad bits");
    return num;
}

fn get_bit_counts(binary_inputs: Vec<String>) -> Vec<i64> {
    let mut bit_counts = vec![0; binary_inputs.first().expect("No str").len()];
    for binary in binary_inputs {
        for (bit_index, bit) in binary.chars().enumerate() {
            if bit == '1' {
                bit_counts[bit_index] += 1;
            } else {
                bit_counts[bit_index] -= 1;
            }
        }
    }
    return bit_counts;
}

fn get_primary_bits(ones: Vec<String>, zeros: Vec<String>, default_bit: char) -> Vec<String> {
    if ones.len() == zeros.len() {
        match default_bit {
            '0' => return zeros,
            '1' => return ones,
            _ => println!("Biting off the wrong bit")
        }
    }

    let operator = match default_bit {
        '1' => std::cmp::PartialOrd::gt,
        '0' => std::cmp::PartialOrd::lt,
        _ => panic!()
    };
    // println!("{} - {}-{} = {}", default_bit, ones.len(), zeros.len(), operator(&ones.len(), &zeros.len()));
    if operator(&ones.len(), &zeros.len()) {
        return ones;
    } else {
        return zeros;
    };
}

fn deduce_bits(binary_inputs: Vec<String>, default_bit: char) -> String {

    let mut current_inputs = binary_inputs;

    for bit_index in 0..12 {
        let mut ones: Vec<String> = Vec::new();
        let mut zeros: Vec<String> = Vec::new();
        for binary in current_inputs.as_mut_slice() {
            let bit = binary.chars().nth(bit_index).unwrap();
            if bit == '1' {
                ones.push(binary.to_string());
            } else {
                zeros.push(binary.to_string());
            }
        }

        current_inputs = get_primary_bits(ones, zeros, default_bit);
        if current_inputs.len() == 1 { break }
    }
    return current_inputs.first().unwrap().to_string();
}

fn get_ratings() {
    let binary_inputs = utils::lines_from_file("data/day3");
    let oxygen = deduce_bits(binary_inputs.clone(), '1');
    let co2 = deduce_bits(binary_inputs.clone(), '0');
    let oxy_val = bit_to_number(oxygen);
    let co2_val = bit_to_number(co2);
    println!("Air rating: {}", oxy_val * co2_val);
}

pub fn binary_diagnostic() {
    let binary_inputs = utils::lines_from_file("data/day3");

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    let bit_counts = get_bit_counts(binary_inputs);

    for bit_count in bit_counts {
        if bit_count >= 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma_val = bit_to_number(gamma);
    let epsilon_val = bit_to_number(epsilon);

    println!("Power Consumption: {}", gamma_val*epsilon_val);
    get_ratings();

}