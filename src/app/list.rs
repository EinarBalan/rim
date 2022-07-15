use std::collections::LinkedList;

pub fn from(string: &str) -> LinkedList<char> {
    let mut list = LinkedList::<char>::new();
    for c in string.chars() {
        list.push_back(c);
    }
    list
}

pub fn from_vec(lines: &Vec<&str>) -> Vec<LinkedList<char>> {
    let mut vec = Vec::<LinkedList<char>>::new();
    for line in lines {
        vec.push(from(line));
    }
    vec
}

pub fn display(list: &LinkedList<char>) -> String {
    let mut result = String::new();
    for c in list.iter() {
        result.push(*c);
    }
    result
}