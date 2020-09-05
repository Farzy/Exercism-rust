pub fn encode(n: u64) -> String {
    if n == 0 {
        "zero".into()
    } else if n < 1000 {
        thousands(n)
    } else
    {
        unimplemented!("Say {} in English.", n);
    }
}

fn thousands(n: u64) -> String {
    let mut res: Vec<String> = Vec::new();
    if n / 100 != 0 {
        res.push(unit(n /100).into());
        res.push("hundred".into());
    }
    let n = n % 100;
    if n < 10 {
        res.push(unit(n).into());
    } else if n < 20 {
        res.push(ten_something(n).into())
    } else if n < 100 {
        if n % 10 == 0 {
            res.push(dozen(n / 10).into())
        } else {
            res.push(format!("{}-{}", dozen(n / 10), unit(n % 10)))
        }
    }
    res.join(" ")
}

fn unit(n: u64) -> &'static str {
    match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("Units greater than 9"),
    }
}

fn ten_something(n: u64) -> &'static str {
    match n {
        1 => "eleven",
        2 => "twelve",
        3 => "thirteen",
        4 => "fourteen",
        5 => "fifteen",
        6 => "sixteen",
        7 => "seventeen",
        8 => "eighteen",
        9 => "nineteen",
        _ => panic!(format!("Invalid number: {}", n)),
    }
}

fn dozen(n: u64) -> &'static str {
    match n {
        2 => "twenty",
        3 => "thirty",
        4 => "fourty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => panic!(format!("Invalid dozen: {}", n)),
    }
}
