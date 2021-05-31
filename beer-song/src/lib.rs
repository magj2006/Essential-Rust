pub fn verse(n: u32) -> String {
    assert!(n < 100);

    let (a, b, c, d) = ("No", "no", "bottle", "bottles");
    let (e, f, g) = ("Go to the store and buy some more", "Take it down and pass it around", "Take one down and pass it around");

    match n {
        0 => format!("{} more {} of beer on the wall, {} more {} of beer.\n{}, 99 {} of beer on the wall.\n", a, d, b, d, e, d),
        1 => format!("{} {} of beer on the wall, {} {} of beer.\n{}, {} more {} of beer on the wall.\n", n, c, n, c, f, b, d),
        2 => format!("{} {} of beer on the wall, {} {} of beer.\n{}, {} {} of beer on the wall.\n", n, d, n, d, g, n - 1, c),
        _ => format!("{} {} of beer on the wall, {} {} of beer.\n{}, {} {} of beer on the wall.\n", n, d, n, d, g, n - 1, d),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut sing_str = String::new();
    for n in end ..= start {
        sing_str.insert_str(0, &verse(n));
        if n != start {
            sing_str.insert(0, '\n')
        }
    }

    sing_str
}
