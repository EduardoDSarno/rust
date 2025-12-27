use std::fs;
fn main() 
{
    match fs::read_to_string("src/logs.txt") 
    {
        Ok(success) =>
        {
            println!("{}", success)
        }
        Err(error) =>
        {
            println!("{}", error)
        }
        
    } 
    
}
fn extract_error(log_txt: &str) -> Vec<&str>
{
    let lines: Vec<&str> = log_txt.lines().collect();

    let mut result: Vec<&str> = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            result.push(line);
        }
    }
    result
}




