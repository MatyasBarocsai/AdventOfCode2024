mod day_05;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_05.txt");

    day_05::solution(&file_contents);
    
}

