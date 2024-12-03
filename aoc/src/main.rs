mod day_03;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_03.txt");

    day_03::solution(&file_contents);
    
}

