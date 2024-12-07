mod day_07;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_07.txt");

    day_07::solution(&file_contents);
    
}

