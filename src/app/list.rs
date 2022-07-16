use std::collections::LinkedList;

pub fn from(lines: &LinkedList<&str>) -> LinkedList<String> {
    let mut list = LinkedList::<String>::new();
    for line in lines {
        list.push_back(line.to_string());
    }
    list
}