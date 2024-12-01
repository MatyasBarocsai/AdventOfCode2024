use std::fs;

pub fn read_input_file(file_path: &str) -> String
{
    let file_contents = fs::read_to_string(file_path)
        .expect("Could not read file...");

    file_contents
}
