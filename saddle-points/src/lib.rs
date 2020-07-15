use std::cmp::min;

const SADDLE_REAL_RANGE: (f64, f64) = (-1.0, 1.0);

pub fn create_saddle(row_range: (u64, u64), col_range: (u64, u64)) -> Vec<Vec<u64>> {
    let saddle_scale = 1000.0 *
        (((row_range.1 - row_range.0 + col_range.1 - col_range.0) / 2) as f64).log10();
    let saddle_offset = saddle_scale;
    let row_step =
        (SADDLE_REAL_RANGE.1 - SADDLE_REAL_RANGE.0) /
            (row_range.1 - row_range.0) as f64;
    let col_step =
        (SADDLE_REAL_RANGE.1 - SADDLE_REAL_RANGE.0) /
            (col_range.1 - col_range.0) as f64;
    let mut saddle = Vec::new();
    for row in row_range.0..=row_range.1 {
        let mut saddle_row = Vec::new();
        for col in col_range.0..=col_range.1 {
            let real_row = ((row - row_range.0) as f64) * row_step + SADDLE_REAL_RANGE.0;
            let real_col = ((col - col_range.0) as f64) * col_step + SADDLE_REAL_RANGE.0;
            let height = ((real_row.powi(2) - real_col.powi(2))
                * saddle_scale + saddle_offset) as u64;
            saddle_row.push(height);
        }
        saddle.push(saddle_row);
    }
    saddle
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_points_by_row: Vec<(usize, usize)> = vec![];
    if input.is_empty() {
        return max_points_by_row;
    }
    let mut min_values_by_col: Vec<u64> = vec![];

    // Find largest elements in each row and smallest value per column
    // at the same time.
    for (row_num, row) in input.iter().enumerate() {
        // One time initialisation of min_values_by_col
        if min_values_by_col.is_empty() {
            min_values_by_col.resize(row.len(), u64::MAX);
        }

        if let Some(max_in_row) = row.iter().max() {
            for (col_num, point) in row.iter().enumerate() {
                // Find smallest value in each column
                min_values_by_col[col_num] = min(min_values_by_col[col_num],
                                                 *point);

                // Collect largest elements in each row
                if *point == *max_in_row {
                    max_points_by_row.push((row_num, col_num));
                }
            }
        } else { // Empty line
            return max_points_by_row;
        }
    }

    // Now cleanup the vector max_points_by_row to keep only those points with
    // minimal value per col.
    max_points_by_row.retain(|&point|
        input[point.0][point.1] == min_values_by_col[point.1]);

    max_points_by_row
}

// I'm comparing my solution to the most starred solutions on Exercism.
// They are all very compact and functional style, but all of them are
// VERY slow because they nest up to 3 loops and iterates/calculates
// the sames values again and again…
//
// Use "cargo bench" to compare them.
//
// Samples results:
//
// Farzad Saddle 1000      time:   [2.1375 ms 2.1898 ms 2.2418 ms]
//
// Benchmarking seungha-kim Saddle 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.4s, or reduce sample count to 40.
// seungha-kim Saddle 1000 time:   [110.46 ms 111.32 ms 112.35 ms]
// Found 12 outliers among 100 measurements (12.00%)
// 2 (2.00%) high mild
// 10 (10.00%) high severe
//
// Benchmarking akiekintveld Saddle 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 42.9s, or reduce sample count to 10.
// akiekintveld Saddle 1000 time:   [423.08 ms 427.99 ms 433.17 ms]
// Found 2 outliers among 100 measurements (2.00%)
// 2 (2.00%) high mild
//
// Benchmarking amdrwe Saddle 1000: Warming up for 3.0000 s
// Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 284.1s, or reduce sample count to 10.
// amdrwe Saddle 1000      time:   [2.7762 s 2.7982 s 2.8191 s]
// Found 16 outliers among 100 measurements (16.00%)
// 4 (4.00%) low severe
// 2 (2.00%) low mild
// 5 (5.00%) high mild
// 5 (5.00%) high severe

// Solution https://exercism.io/tracks/rust/exercises/saddle-points/solutions/062e8486d1954e00a5af8475eef26950
pub fn find_saddle_points_seungha_kim(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let row_count = input.len();
    for (row_num, row) in input.iter().enumerate() {
        for (col_num, item) in row.iter().enumerate() {
            if row.iter().all(|x| x <= item) && (0..row_count).all(|x| input[x][col_num] >= *item) {
                found.push((row_num, col_num));
            }
        }
    }
    found
}

// Solution https://exercism.io/tracks/rust/exercises/saddle-points/solutions/1312ec0b737c4b259156cbedf58f3bf0
pub fn find_saddle_points_akiekintveld(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input.iter().enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().map(move |(c, &v)| (r, c, v))
        })
        .filter_map(|(r, c, v)| {
            if input.iter().all(|row| v <= row[c]) && input[r].iter().all(|&x| v >= x) {
                Some((r, c))
            } else {
                None
            }
        })
        .collect()
}

// Solution https://exercism.io/tracks/rust/exercises/saddle-points/solutions/ef445ac491424d6d8fd6da25a61213c4
pub fn find_saddle_points_amdrwe(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = vec![];
    for (y, row) in input.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            let col: Vec<u64> = input.iter().map(|v| v[x]).collect();
            if row.iter().all(|m| n>=m) && col.iter().all(|m| n<=m){
                points.push((y, x));
            }
        }
    }
    points
}