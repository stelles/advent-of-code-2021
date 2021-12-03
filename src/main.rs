
// use depth;

// mod utils;
// use depth;

fn num_increases(depths: Vec<i64>) -> i64 {
    let mut previous_depth = f64::INFINITY as i64;
    let mut increase_count = 0;
    for current_depth in depths {
        if current_depth > previous_depth {
            increase_count += 1;
        }
        previous_depth = current_depth;
    }
    return increase_count;
}

fn sum_three(depths: Vec<i64>) -> Vec<i64> {

    let mut three_sums: Vec<i64> = Vec::new();

    for i in 2..depths.len() {
        three_sums.push(depths[i-2] + depths[i-1] + depths[i]);
    }
    return three_sums;
}

fn count_depth() {
    let lines = utils::lines_from_file("data/day1");

    let depths = lines.iter().map(|l| l.parse::<i64>().expect("Cannot parse int")).collect();
    
    let three_sum_depths = sum_three(depths);

    let increase_count = num_increases(three_sum_depths);    

    println!("Number of increases: {}", increase_count);
}

fn movement() {
    let moves = utils::lines_from_file("data/day2");

    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal_position = 0;

    for m in moves {
        let mut move_command = m.split_whitespace();
        let direction = move_command.next().expect("Bad directions");
        let distance = move_command.next().expect("How much").parse::<i64>().expect("Amount number");
        match direction {
            "forward" => {
                horizontal_position += distance;
                depth += aim * distance;
            },
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => println!("wtf")
        }
    }
    
    println!("Depth: {}", depth);
    println!("Distanct: {}", horizontal_position);
    println!("Total: {}", depth * horizontal_position);

}


fn main() {
    count_depth();
    movement();
}
