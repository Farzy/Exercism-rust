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
