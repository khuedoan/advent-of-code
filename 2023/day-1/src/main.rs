use std::fs;

fn main() {
    let input_path = "./input";
    println!("Reading input file {input_path}");

    let input_content = fs::read_to_string(input_path)
        .expect("Cannot read file");

    print!("{input_content}");
}
