mod day_02;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_02.txt");

    day_02::solution(&file_contents);
    
}

