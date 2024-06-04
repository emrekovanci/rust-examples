struct MatrixElementIndex {
    row: usize,
    col: usize,
}

fn is_adjacent(source: &MatrixElementIndex, target: &MatrixElementIndex) -> bool {
    let row_diff = (source.row as i32 - target.row as i32).abs();
    let col_diff = (source.col as i32 - target.col as i32).abs();

    row_diff + col_diff == 1
}

fn main() {
    const INPUT_MATRIX_ROWS: usize = 3;
    const INPUT_MATRIX_COLUMNS: usize = 2;
    const OUTPUT_MATRIX_SIZE: usize = INPUT_MATRIX_ROWS * INPUT_MATRIX_COLUMNS;

    let input_matrix: [[i32; INPUT_MATRIX_COLUMNS]; INPUT_MATRIX_ROWS] = [[9, 4], [6, 3], [5, 8]];

    let mut output_matrix: [[i32; OUTPUT_MATRIX_SIZE]; OUTPUT_MATRIX_SIZE] =
        [[0; OUTPUT_MATRIX_SIZE]; OUTPUT_MATRIX_SIZE];

    for (y, output_row) in output_matrix.iter_mut().enumerate() {
        let source_index = MatrixElementIndex {
            row: y / INPUT_MATRIX_COLUMNS,
            col: y % INPUT_MATRIX_COLUMNS,
        };

        for (x, output_col) in output_row.iter_mut().enumerate() {
            let target_index = MatrixElementIndex {
                row: x / INPUT_MATRIX_COLUMNS,
                col: x % INPUT_MATRIX_COLUMNS,
            };

            *output_col = is_adjacent(&source_index, &target_index).into();
        }
    }

    println!("Input Matrix: {:?}", input_matrix);
    println!("Output Matrix: {:?}", output_matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        // [
        //    [index1, index2],
        //    [index3, index4],
        // ]
        let index1 = MatrixElementIndex { row: 0, col: 0 };
        let index2 = MatrixElementIndex { row: 0, col: 1 };
        let index3 = MatrixElementIndex { row: 1, col: 0 };
        let index4 = MatrixElementIndex { row: 1, col: 1 };

        // adjacent indexes
        assert!(is_adjacent(&index1, &index2));
        assert!(is_adjacent(&index2, &index1));
        assert!(is_adjacent(&index1, &index3));
        assert!(is_adjacent(&index3, &index1));
        assert!(is_adjacent(&index2, &index4));
        assert!(is_adjacent(&index4, &index2));
        assert!(is_adjacent(&index3, &index4));
        assert!(is_adjacent(&index4, &index3));

        // non-adjacent indexes
        assert!(!is_adjacent(&index1, &index1));
        assert!(!is_adjacent(&index2, &index2));
        assert!(!is_adjacent(&index3, &index3));
        assert!(!is_adjacent(&index4, &index4));
        assert!(!is_adjacent(&index1, &index4));
        assert!(!is_adjacent(&index4, &index1));
        assert!(!is_adjacent(&index2, &index3));
        assert!(!is_adjacent(&index3, &index2));
    }
}
