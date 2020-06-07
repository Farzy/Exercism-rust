pub fn verse(n: u32) -> String {
    if n != 0 {
        let plural = match n {
            1 => (String::from(""), String::from("it"), String::from("no more bottles")),
            2 => (String::from("s"), String::from("one"), String::from("1 bottle")),
            _ => (String::from("s"), String::from("one"), format!("{} bottles", n-1)),
        };
        return format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", n, plural.0, n, plural.0, plural.1, plural.2)
    } else {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start+1)
        .rev()
        .map(|x| { verse(x) })
        .collect::<Vec<String>>()
        .join("\n")
}
