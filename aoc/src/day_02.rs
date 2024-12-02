

struct Report 
{
    levels: Vec<i32> 
}

fn determine_safety(report: &Report) -> bool
{


    let mut is_positive = true;

    for i in 1..report.levels.len()
    {
        
        // Check if decreasing
        if i == 1 && (report.levels[i-1] - report.levels[i]) > 0 { is_positive = false; }

        // Check for level change
        if (report.levels[i] - report.levels[i-1]).abs() < 1 || (report.levels[i] - report.levels[i-1]).abs() > 3 { return false; }

        // If values reverse
        if is_positive && (report.levels[i-1] - report.levels[i] > 0 ) { return false; }     
    
        // If values revese
        if !is_positive && (report.levels[i-1] - report.levels[i] < 0 ) { return false; } 
    
    }

    println!("{:?}", report.levels);
    
    true
}

fn part1(reports: &Vec<Report>)
{
    
    let mut result = 0;
    for r in reports 
    { 
        if determine_safety(&r) { result += 1; }
    };

    println!("\nPart_1 Result: {}\n", result);
}


fn part2(reports: &Vec<Report>)
{
    let mut result = 0;
    'report: for r in reports
    {
        if !determine_safety(&r)
        {
            for l in 0..r.levels.len()
            {
                let mut tmp = Report { levels: r.levels.clone() };
                tmp.levels.remove(l);
                
                if determine_safety(&tmp) 
                {
                    result += 1;
                    continue 'report; 
                }
            }
        }
        else
        {
            result += 1;
        }
    }

    println!("\nPart_2 Result: {}\n", result);
}


pub fn solution(input: &String){
    
    // Split the input file into its lines
    let lines = input.split("\n");

    let mut reports = Vec::<Report>::new();

    for line in lines
    {
    
        if line == "" {continue;}

        let result: Vec<i32> = line.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse().expect("Could no parse!") ).collect::<Vec<i32>>();
   
        reports.push( Report {levels: result} );

    }

    part1(&reports);
    part2(&reports);    

}
