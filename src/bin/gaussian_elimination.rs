fn gaussian_elimination(mut matrix: Vec<Vec<f64>>, mut constants: Vec<f64>) -> Option<Vec<f64>> {
    let n = matrix.len();

    // Perform Gaussian Elimination with partial pivoting
    for pivot_row in 0..n {
        // Find the pivot element and swap rows if necessary
        let mut max_row = pivot_row;
        for row in pivot_row + 1..n {
            if matrix[row][pivot_row].abs() > matrix[max_row][pivot_row].abs() {
                max_row = row;
            }
        }
        if max_row != pivot_row {
            matrix.swap(pivot_row, max_row);
            constants.swap(pivot_row, max_row);
        }

        // Eliminate elements below the pivot
        for row in pivot_row + 1..n {
            let factor = matrix[row][pivot_row] / matrix[pivot_row][pivot_row];
            for col in pivot_row..n {
                matrix[row][col] -= factor * matrix[pivot_row][col];
            }
            constants[row] -= factor * constants[pivot_row];
        }
    }

    // Back-substitution to find the solution
    let mut solution = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i + 1..n {
            sum += matrix[i][j] * solution[j];
        }
        solution[i] = (constants[i] - sum) / matrix[i][i];
    }

    Some(solution)
}

fn main() {
    let matrix = vec![
        vec![2.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        vec![1.0, 3.0, 2.0, 1.0, 1.0, 1.0],
        vec![1.0, 2.0, 4.0, 2.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0, 5.0, 2.0, 1.0],
        vec![1.0, 1.0, 1.0, 2.0, 6.0, 2.0],
        vec![1.0, 1.0, 1.0, 1.0, 2.0, 7.0],
    ];

    let constants = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    match gaussian_elimination(matrix, constants) {
        Some(solution) => println!("Solution: {:?}", solution),
        None => println!("Failed to solve the equations."),
    }
}
