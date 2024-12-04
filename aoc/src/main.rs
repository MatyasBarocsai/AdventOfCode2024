mod day_04;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_04.txt");

    day_04::solution(&file_contents);
    
}

