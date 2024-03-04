use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}
impl Matrix {

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data
        }
    }

    pub fn zeroes(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows, cols, data: vec![vec![0.0; cols]; rows]
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res: Matrix = Matrix::zeroes(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        res
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempting to multply  matrices of incompatible sizes");
        }
        let mut result = Matrix::zeroes(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] *  other.data[k][j];
                }
                result.data[i][j] = sum
            }

        };
        result
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            println!("{}:{} and {}:{}", self.rows, self.cols, other.rows, other.cols);
            panic!("Trying to add two matrices with different dimensions")
        }
        let mut result = Matrix::zeroes(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
    pub fn subtract(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            println!("{}:{} and {}:{}", self.rows, self.cols, other.rows, other.cols);
            panic!("Trying to add two matrices with different dimensions")
        }
        let mut result = Matrix::zeroes(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
    pub fn dot_x(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            println!("{}:{} and {}:{}", self.rows, self.cols, other.rows, other.cols);
            panic!("Trying to add two matrices with different dimensions")
        }
        let mut result = Matrix::zeroes(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                result.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        result
    }
    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from(
            (self.data)
        .clone()
        .into_iter()
        .map(|row| row.into_iter().map(|value| function(value)).collect())
        .collect()
        )
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeroes(self.cols, self.rows); // Transpose dimensions
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
    

}

