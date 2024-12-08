mod day_08;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_08.txt");

    day_08::solution(&file_contents);
    
}

