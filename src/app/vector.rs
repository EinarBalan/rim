pub fn from(lines: Vec<&str>) -> Vec<String> {
    let mut vec = Vec::<String>::new();
    for line in lines {
        vec.push(line.to_string());
    }
    vec
}