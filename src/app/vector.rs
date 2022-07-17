pub fn from(lines: Vec<&str>) -> Vec<String> {
    let mut vec = Vec::<String>::new();
    for line in lines {
        vec.push(line.to_string());
    }
    // we want a new line at the end of every buffer
    vec.push(String::new()); 
    vec
}