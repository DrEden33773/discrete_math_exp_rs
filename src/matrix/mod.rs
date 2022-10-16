pub fn test() {
    println!("Hello, world!");
}

pub struct adjacency_matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<usize>>,
}

/**
 * public methods
 */
impl adjacency_matrix {
    pub fn new_zero(rows: usize, cols: usize) -> adjacency_matrix {
        let mut data = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(0);
            }
            data.push(row);
        }
        adjacency_matrix { rows, cols, data }
    }
    pub fn new(rows: usize, cols: usize) -> adjacency_matrix {
        Self::new_zero(rows, cols)
    }
    pub fn create_by_vec(input_data: Vec<Vec<usize>>) -> adjacency_matrix {
        let rows = input_data.len();
        let cols = input_data[0].len();
        adjacency_matrix {
            rows,
            cols,
            data: input_data,
        }
    }
    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.data[row][col]);
            }
            println!("");
        }
        println!("");
    }
}

/**
 * calculation check
 */
impl adjacency_matrix {
    // is_symmetric
    pub fn is_symmetric(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] != self.data[col][row] {
                    return false;
                }
            }
        }
        true
    }
    // is_transitive
    pub fn is_transitive(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }
        let mut result = Self::new_zero(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                for k in 0..self.cols {
                    result.data[row][col] += self.data[row][k] * self.data[k][col];
                }
            }
        }
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] != result.data[row][col] {
                    return false;
                }
            }
        }
        true
    }
    // is_reflexive
    pub fn is_reflexive(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }
        for row in 0..self.rows {
            if self.data[row][row] != 1 {
                return false;
            }
        }
        true
    }
    // is_assignable
    pub fn is_assignable(a: &adjacency_matrix, b: &adjacency_matrix) -> bool {
        if a.rows != b.rows || a.cols != b.cols {
            return false;
        }
        true
    }
    // is_multipliable
    pub fn is_multipliable(a: &adjacency_matrix, b: &adjacency_matrix) -> bool {
        if a.cols != b.rows {
            return false;
        }
        true
    }
}
