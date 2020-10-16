extern crate rand;

use rand::Rng;

#[derive(Clone)]
pub struct Matrix {
    raw: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn create_zeroes(rows_param: usize, cols_param: usize) -> Matrix {
        let matrix: Matrix = Matrix {
            raw: vec![0.0; rows_param * cols_param],
            rows: rows_param,
            cols: cols_param,
        };
        return matrix.clone();
    }
    pub fn create_randoms(rows_param: usize, cols_param: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let matrix: Matrix = Matrix {
            raw: (0..rows_param * cols_param)
                .map(|_| rng.gen::<f64>() * 10.0 - 5.0)
                .collect(),
            rows: rows_param,
            cols: cols_param,
        };
        return matrix.clone();
    }

    pub fn create_init(rows_param: usize, cols_param: usize, values: Vec<f64>) -> Matrix {
        if rows_param * cols_param != values.len() {
            panic!("Invalid dimensions for the matrix");
        }
        let matrix: Matrix = Matrix {
            raw: values,
            rows: rows_param,
            cols: cols_param,
        };
        return matrix.clone();
    }
    pub fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.rows || col >= self.cols {
            panic!("Invalid number used to index matrix");
        } else {
            return self.raw[row * self.cols + col];
        }
    }
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        if row >= self.rows || col >= self.cols {
            panic!("Invalid number used to index matrix")
        } else {
            self.raw[row * self.cols + col] = val;
        }
    }
    pub fn rows(&self) -> usize {
        return self.rows;
    }
    pub fn cols(&self) -> usize {
        return self.cols;
    }
    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{}  ", self.get(i, j));
            }
            print!("\n");
        }
    }
    pub fn append_rows(&mut self, other: &Matrix) {
        if self.cols() != other.cols() {
            panic!("Cannot append matrices with different number of columns");
        } else {
            self.rows += other.rows();
            for i in 0..other.rows() {
                for j in 0..other.cols() {
                    self.raw.push(other.get(i, j));
                }
            }
        }
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Matrix) -> Matrix {
        if self.rows != _rhs.rows() || self.cols != _rhs.cols() {
            panic!("Cannot add matrices of different sizes!!!");
        } else {
            let mut matrix: Matrix = self.clone();
            for i in 0..self.rows {
                for j in 0..self.cols {
                    matrix.set(i, j, self.get(i, j) + _rhs.get(i, j));
                }
            }
            return matrix;
        }
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, _rhs: Matrix) -> Matrix {
        if self.rows != _rhs.rows() || self.cols != _rhs.cols() {
            panic!("Cannot subtract matrices of different sizes!!!");
        } else {
            let mut matrix: Matrix = self.clone();
            for i in 0..self.rows {
                for j in 0..self.cols {
                    matrix.set(i, j, self.get(i, j) - _rhs.get(i, j));
                }
            }
            return matrix;
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        if self.cols != _rhs.rows() {
            panic!("Attempted to multiply invalid matrices");
        } else {
            let mut matrix: Matrix = Matrix {
                rows: self.rows,
                cols: _rhs.cols,
                raw: vec![0.0; self.rows * _rhs.cols],
            };
            for i in 0..self.rows {
                for j in 0.._rhs.cols {
                    for k in 0..self.cols {
                        matrix.set(i, j, matrix.get(i, j) + self.get(i, k) * _rhs.get(k, j));
                    }
                }
            }
            return matrix;
        }
    }
}
