use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn count_depth() {
    let lines = lines_from_file("data/day1");

    let depths = lines.iter().map(|l| l.parse::<i64>().expect("Cannot parse int"));
    
    let mut previous_depth = f64::INFINITY as i64;
    let mut increase_count = 0;
    for current_depth in depths {
        if current_depth > previous_depth {
            increase_count += 1;
        }
        previous_depth = current_depth;
    }
    println!("Number of increases: {}", increase_count);
}
