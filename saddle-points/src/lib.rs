use std::cmp::min;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_points_by_row: Vec<(usize, usize)> = vec![];
    if input.is_empty() {
        return max_points_by_row;
    }
    let mut min_values_by_col: Vec<u64> = vec![];

    // Find largest elements in each row
    for (r, v) in input.iter().enumerate() {
        // One time initialisation of min_values_by_col
        if min_values_by_col.is_empty() {
            min_values_by_col.resize(v.len(), u64::MAX);
        }

        if let Some(max_in_row) = v.iter().max() {
            for (c, point) in v.iter().enumerate() {
                // Find small value in each column
                min_values_by_col[c] = min(min_values_by_col[c], *point);

                // Collect largest elements in each row
                if *point == *max_in_row {
                    max_points_by_row.push((r, c));
                }
            }
        } else { // Empty line
            return max_points_by_row;
        }
    }

    // Now cleanup the vector max_points_by_row to keep only those with
    // minimal value per col
    max_points_by_row
        .retain(|&point|
            input[point.0][point.1] == min_values_by_col[point.1]);

    max_points_by_row
}
