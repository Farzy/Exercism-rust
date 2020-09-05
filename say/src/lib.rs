pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".into();
    }

    let mut res: Vec<String> = Vec::new();

    // Count the number of 3 digits blocks
    let blocks = (((n as f64).log10().floor()) / 3.0) as u64;
    // Treat them starting at the largest block
    for block in (0..=blocks).rev() {
        let i = (n / 1000u64.pow(block as u32)) % 1000;
        if i != 0 {
            res.push(thousands(i as u64));
            if block != 0 {
                res.push(thousand_multiples(block).into());
            }
        }
    }

    res.join(" ")
}

fn thousand_multiples(n: u64) -> &'static str {
    match n {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => panic!(format!("Thousand multiple {} not implemented", n)),
    }
}

fn thousands(n: u64) -> String {
    let mut res: Vec<String> = Vec::new();
    if n / 100 != 0 {
        res.push(unit(n /100).into());
        res.push("hundred".into());
    }
    let n = n % 100;
    if n == 0 {
        // Do nothing
    } else if n < 10 {
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
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => panic!(format!("Invalid number: {}", n)),
    }
}

fn dozen(n: u64) -> &'static str {
    match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => panic!(format!("Invalid dozen: {}", n)),
    }
}
