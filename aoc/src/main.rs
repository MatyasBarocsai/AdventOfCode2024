mod day_14;
mod util;

fn main(){

    let file_contents = util::read_input_file("/home/god/git/AdventOfCode2024/inputs/day_14.txt");
    
    day_14::solution(&file_contents);
    
}

