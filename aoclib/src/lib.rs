use std::{
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::Write,
};

pub fn input(name: &str) -> String {
    // recreate empty output file
    let _ = File::create("./output");
    fs::read_to_string(format!("./{name}")).unwrap()
}

pub fn output(result: impl Display) {
    println!("{result}");
    let mut file = OpenOptions::new().append(true).open("./output").unwrap();
    file.write_all(format!("{}\n", result).as_bytes()).unwrap();
}
