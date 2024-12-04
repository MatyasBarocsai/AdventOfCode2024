use regex::Regex;

fn create_rows(grid: &Vec<Vec<char>>) -> Vec<String> {
    
    let mut rows = Vec::<String>::new();
    
    for y in 0..grid.len() {
        let mut row = String::new();
        for x in 0..grid[0].len() {
            row.push(grid[y][x]);
        }
        rows.push(row);
    }
    rows
}

fn create_cols(grid: &Vec<Vec<char>>) -> Vec<String> {
    
    let mut cols = Vec::<String>::new();
    
    for x in 0..grid[0].len() {
        let mut col = String::new();
        for y in 0..grid.len() {
            col.push(grid[y][x]);
        }
        cols.push(col);
    }
    cols
}

fn create_diagonals(grid: &Vec<Vec<char>>) -> Vec<String> {
    
    let mut diagonals = Vec::<String>::new();  

    for i in 0..grid.len() {
        let mut d1 = String::new();
        let mut d2 = String::new();
        
        for j in 0..(grid.len()-i) {
            d1.push(grid[i+j][j]);
            d2.push(grid[j][i+j]);
        }

        diagonals.push(d1);
        diagonals.push(d2);

    }

    let n = grid.len();
    for i in 0..n {
        let mut d1 = String::new();
        let mut d2 = String::new();
        
        for j in 0..(i+1) {
            d1.push(grid[i-j][j]);
            d2.push(grid[n-1-j][n-1-i+j]);
        }

        diagonals.push(d1);
        diagonals.push(d2);

    }

    diagonals.remove(0);
    diagonals.remove(diagonals.len() - 1);

    diagonals
} 

fn part1(haystack: &String) -> usize {
    
    let re_forward = Regex::new("XMAS").unwrap();
    let re_backward = Regex::new("SAMX").unwrap();

    let mut result: usize = 0;
        
    for _cap in re_forward.captures_iter(haystack) {
        result += 1;
    }

    for _cap in re_backward.captures_iter(haystack) {
        result += 1;
    }


    result
}

fn part2(grid: &Vec<Vec<char>>) -> usize {

    let mut count = 0;

    for y in 1..grid.len()-1 {
        
        for x in 1..grid[0].len()-1 {
            
            if grid[y][x] != 'A' {continue;}
            
            let mut correct_diagonal = 0;

            //check left-right diagonal
            if (grid[y+1][x-1] == 'S' && grid[y-1][x+1] == 'M') || (grid[y+1][x-1] == 'M' && grid[y-1][x+1] == 'S') {
                correct_diagonal += 1;
            }

            if (grid[y-1][x-1] == 'S' && grid[y+1][x+1] == 'M') || (grid[y-1][x-1] == 'M' && grid[y+1][x+1] == 'S') {
                correct_diagonal += 1;
            }

            if correct_diagonal == 2 { count += 1; }
        }
    }
    
    count
}

pub fn solution(input: &String){
    
    let lines: Vec<&str> = input.split("\n")
        .collect();

    let mut grid = Vec::<Vec<char>>::new();  

    for line in lines{
        
        if line == "" {continue;}
        
        let mut row = Vec::<char>::new();

        for c in line.chars() {
            
            if "XMAS".contains(c) { row.push(c); }
            else { row.push('.'); }

        }
        grid.push(row);
    }

    //for row in &grid { println!("{:?}", row); }
   
    let mut result: usize = 0;

    let mut diagonals = create_diagonals(&grid);
    let mut rows = create_rows(&grid);
    let mut cols = create_cols(&grid);

    diagonals.append(&mut rows);
    diagonals.append(&mut cols);

    for i in 0..diagonals.len() { 
        result += part1(&diagonals[i]); 
       //println!("{:?}", diagonals[i]); 
    } 

    println!("Result Part 1: {result}");

    let result = part2(&grid);
    

    println!("Result Part 2: {result}");


}
