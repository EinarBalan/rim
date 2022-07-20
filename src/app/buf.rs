use gapbuf::GapBuffer;

pub fn from_string(str: &str) -> GapBuffer<GapBuffer<char>> {
    let lines = str.lines();

    let mut res: GapBuffer<GapBuffer<char>> = GapBuffer::new();
    for line in lines {
        let mut buf: GapBuffer<char> = GapBuffer::new();
        for c in line.chars() {
            buf.push_back(c);
        }
        res.push_back(buf);
    }
    res.push_back(GapBuffer::new());
    res
}

pub fn to_string(buf: &GapBuffer<char>) -> String {
    let mut s = String::new();
    for c in buf.iter() {
        s.push(*c)
    }        

    s
}

pub fn push_owned<T>(buf: &mut GapBuffer<T>, it: impl IntoIterator<Item = T>){
    for i in it {
        buf.push_back(i);
    }
}

pub fn push_ref<'a, I, T: 'a>(buf: &mut GapBuffer<T>, it: I) where I: Iterator<Item = &'a T>, T: Copy {
    for i in it {
        buf.push_back(*i);
    }
}


