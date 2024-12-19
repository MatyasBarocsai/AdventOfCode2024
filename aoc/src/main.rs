mod day_16;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_16.txt");
    
    day_16::solution(&file_contents);
    
}

