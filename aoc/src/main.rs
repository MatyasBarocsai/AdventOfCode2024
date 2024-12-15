mod day_15;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_15.txt");
    
    day_15::solution(&file_contents);
    
}

