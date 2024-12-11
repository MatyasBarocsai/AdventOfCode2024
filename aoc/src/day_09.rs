use std::{ops::Index, thread::available_parallelism};


#[derive(
    Debug,
    PartialEq
)]
enum BlockType {
    FILE,
    EMPTY
}

struct ContSpace {
    owner: i32,
    blocks: Vec<usize>
}

struct MemoryBlock {
    owner: i32, 
    block_type: BlockType
}

/*
fn defragged(harddrive: &Vec<FileBlock>) -> bool {
    
    let mut fragment: usize = 0;

    for i in 1..harddrive.len() {
        if harddrive[i].block_type != harddrive[i-1].block_type {fragment += 1;}
        if fragment > 1 {return false;}
    }
    
    fragment == 1
}*/

/*
fn part2(harddrive: &mut Vec<FileBlock>, headers: &Vec<File>) {

    for src in (0..headers.len()).rev() {
        
        if headers[src].sector_type == BlockType::EMPTY {continue;}

        for target in 0..headers.len() {

            if headers[target].sector_type == BlockType::FILE {continue;}
            else if headers[target].length < headers[src].length {continue;}
            else if headers[target].position > headers[src].position {continue;}

            //src += harddrive[src]
            //harddrive.swap(src, target);

        }
    }
}*/

/*
fn part1(harddrive: &mut Vec<FileBlock>) {
    
    // Loop backwards and move each FileBlock
    'outer: for i in (0..harddrive.len()).rev() {
        match harddrive[i].block_type { 
            BlockType::FILE => {
                for k in 0..harddrive.len() {
                    match harddrive[k].block_type {
                        BlockType::EMPTY => {
                            
                            //println!("Swapping {i} with {k}");
                            harddrive.swap(i,k);
                            
                            if defragged(harddrive) {return;}

                            continue 'outer;
                        }
                        BlockType::FILE => {continue;}
                    }
                }
            }
            BlockType::EMPTY => {continue;}
        }
        //println!("Pos: {}, Owner: {}, Type: {:?}", block.position, block.owner, block.block_type);
    }

}*/

fn part2(harddrive: &mut Vec<MemoryBlock>, filesystem: &mut Vec<ContSpace>) {

    for i in (0..filesystem.len()).rev() {

        let free_space = first_free_space(harddrive, filesystem[i].blocks.len());
        
        if free_space.is_empty() || free_space[0] > filesystem[i].blocks[0] {continue;}

        move_file(harddrive, &mut filesystem[i], &free_space);
    }

}

fn first_free_space(harddrive: &Vec<MemoryBlock>, n: usize) -> Vec<usize> {

    let empty = Vec::<usize>::new();
    'outer: for (i,block) in harddrive.iter().enumerate() {
         
        if i + n > harddrive.len() {return empty;}

        let mut result = Vec::<usize>::new();
        for k in 0..n {
            if harddrive[i+k].block_type == BlockType::EMPTY { result.push(i+k); }
            else{continue 'outer;}
        }

        return result;
    }
    empty
}

fn move_file(harddrive: &mut Vec<MemoryBlock>, from: &mut ContSpace, to: &Vec<usize>) {

    if to.len() < from.blocks.len() {return;}

    // Temporary values
    //let from_blocks_copy = from.blocks.clone();
    //let to_blocks_copy = to.blocks.clone();

    let mut new_blocks = Vec::<usize>::new();
    for (i,mbi) in from.blocks.iter().enumerate() {
       
        new_blocks.push(to[i]);
        harddrive.swap(*mbi,to[i]);

        //println!("{:?},{:?}", c,i);
    }

    from.blocks = new_blocks;
}

pub fn solution(input: &str){

    //println!("{:?}", input);

    //let mut harddrive = Vec::<FileBlock>::new();
    //let mut file_table = Vec::<File>::new();
    //let mut file_owner: i32 = 0;
    //let mut harddrive_index: usize = 0;
    //let mut file_index: usize = 0;

    let mut harddrive = Vec::<MemoryBlock>::new();
    let mut filesystem = Vec::<ContSpace>::new();
    let mut freesectors = Vec::<ContSpace>::new();
    let mut file_id: i32 = 0; 
    let mut memory_block_index: usize = 0;

    for (i,c) in input.chars().enumerate() {
        
        if c == '\n' {continue;}

        let mut memory_blocks = Vec::<usize>::new();

        for _k in 0..(c as i32 - 0x30) {

            memory_blocks.push(memory_block_index);
            memory_block_index += 1;
        
            // 0 and every other char, this is a FILE-type block
            if i % 2 == 0 {
                let memory = MemoryBlock{owner: file_id, block_type: BlockType::FILE};
                harddrive.push(memory);
            }
            
            // 1 and every other char, this is a EMPTY-type block
            else if i % 2 == 1 {
                let memory = MemoryBlock{owner: -1, block_type: BlockType::EMPTY};
                harddrive.push(memory);
            }

        }

        if i % 2 == 0 {
            filesystem.push(ContSpace{owner: file_id, blocks: memory_blocks});
            file_id += 1;
        }
        else {
            freesectors.push(ContSpace{owner: -1, blocks: memory_blocks});
        }
    }
   
    println!("All memory blocks:\n");
    for mem_block in &harddrive { println!("Memblock: {:?}, Owner: {}", mem_block.block_type, mem_block.owner);}
    //println!("\n\nAll files:\n");
    //for file in &filesystem {println!("File: {:?}, blocks={:?}", file.owner, file.blocks)}
    //for space in &freesectors {println!("Space: {:?}, blocks={:?}", space.owner, space.blocks)}
    //println!("\n\nAvailable Space:\n");
    //for file in &filesystem {println!("File: {:?}, blocks={:?}", file.owner, file.blocks)}

    

    part2(&mut harddrive, &mut filesystem);
    //let free_space = first_free_space(&harddrive, filesystem[8].blocks.len() );
    //move_file(&mut harddrive, &mut filesystem[8], &free_space);


    println!("\n\nAll memory blocks:\n");
    for mem_block in &harddrive { println!("Memblock: {:?}, Owner: {}", mem_block.block_type, mem_block.owner);}
    //println!("\n\nAll files:\n");
    //for file in &filesystem {println!("File: {:?}, blocks={:?}", file.owner, file.blocks)}


    //println!("First free space [n={}], {:?}", 3, first_free_space(&harddrive, 3));


    //part1(&mut harddrive);

    //println!("\n\n");

    let mut result: i64 = 0;
    let mut index: i64 = -1;
    for block in harddrive {
        //println!("Owner: {}, Type: {:?} Pos: {}", block.owner, block.block_type, block.position);
        //println!("{}", block.owner);
        
        index += 1;
        if block.owner == -1 {continue;}
 
        result += index * (i64::from(block.owner));
    }

    println!("Result: {result}");
}
