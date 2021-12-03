use crate::{
    utils
};

pub fn movement() {
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