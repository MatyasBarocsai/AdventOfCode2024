mod day_01;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_01.txt");

    day_01::solution(&file_contents);
}

