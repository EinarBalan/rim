/// Converts &str vector to String vector and adds new line
pub fn from(lines: Vec<&str>) -> Vec<String> {
    let mut vec: Vec<String> = lines.iter().map(|s| {
        s.to_string()
    }).collect();
    
    // we want a new line at the end of every buffer
    vec.push(String::new()); 
    vec
}