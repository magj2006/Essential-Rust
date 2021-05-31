pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v = Vec::new();

    for ch in string.chars() {
        match ch {
            '(' | '[' | '{' => v.push(ch),
            ')' | ']' | '}' => match v.pop() {
                Some(c) => {
                    match (c, ch) {
                        ('(', ')') => (),
                        ('[', ']') => (),
                        ('{', '}') => (),
                        _ => return false,
                    }
                },
                None => return false,
            } 
            _ => (),
        };
    }
    v.is_empty() 
}
