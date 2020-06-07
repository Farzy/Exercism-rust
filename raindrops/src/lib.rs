pub fn raindrops(n: u32) -> String {
    let factor3 = n % 3 == 0;
    let factor5 = n % 5 == 0;
    let factor7 = n % 7 == 0;

    match (factor3, factor5, factor7) {
        (false, false, false) => n.to_string(),
        (true, false, false)  => "Pling".to_string(),
        (true, true, false)   => "PlingPlang".to_string(),
        (true, false, true)   => "PlingPlong".to_string(),
        (true, true, true)    => "PlingPlangPlong".to_string(),
        (false, true, false)  => "Plang".to_string(),
        (false, true, true)   => "PlangPlong".to_string(),
        (false, false, true)  => "Plong".to_string(),
    }
}
