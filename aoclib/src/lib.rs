use std::fmt::Display;

pub fn output(result: impl Display) {
    println!("{result}");
    std::fs::write("./output", result.to_string()).unwrap();
}
