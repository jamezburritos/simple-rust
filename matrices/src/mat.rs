use std::{fmt, ops};

pub struct Matrix {  
    data: Vec<f32>,
    pub rows: usize,
    pub cols: usize
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows, cols
        }
    }

    pub fn from(data: Vec<f32>, rows: usize, cols: usize) -> Self {
        if data.len() != rows * cols { 
            panic!("Matrix data does not match the given dimensions: {rows}x{cols}")
        }

        Matrix { data, rows, cols }
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output { 
        &mut self.data[index.0 * self.cols + index.1]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "| ")?;

            for j in 0..self.cols {
                write!(f, "{} ", self[(i, j)])?; 
            }

            write!(f, "|")?;
            if i != self.rows - 1 { writeln!(f)? }
        }

        Ok(())
    } 
}

impl ops::Neg for Matrix {
    type Output = Self; 

    fn neg(self) -> Self::Output {
        Matrix {
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);

                for i in 0..(self.cols * self.rows) {
                    data[i] = -self.data[i]
                }

                data
            },

            ..self
        } 
    }
}

impl ops::Add for Matrix {
    type Output = Self; 

    fn add(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            panic!("Matrices incompatible for addition: {}x{} and {}x{}",
                   self.rows, self.cols,
                   rhs.rows, rhs.cols)
        }

        Matrix {
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);

                for i in 0..(self.cols * self.rows) {
                    data[i] = self.data[i] + rhs.data[i]
                }

                data
            },
            ..self
        }
    }
}

impl ops::AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            panic!("Matrices incompatible for addition: {}x{} and {}x{}",
                   self.rows, self.cols,
                   rhs.rows, rhs.cols)
        }

        for i in 0..(self.cols * self.rows) {
            self.data[i] += rhs.data[i]
        } 
    }
}

impl ops::Sub for Matrix {
    type Output = Self; 

    fn sub(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            panic!("Matrices incompatible for subtraction: {}x{} and {}x{}",
                   self.rows, self.cols,
                   rhs.rows, rhs.cols)
        }

        Matrix {
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);

                for i in 0..(self.cols * self.rows) {
                    data[i] = self.data[i] - rhs.data[i]
                }

                data
            },

            ..self
        }
    }
}

impl ops::SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            panic!("Matrices incompatible for subtraction: {}x{} and {}x{}",
                   self.rows, self.cols,
                   rhs.rows, rhs.cols)
        }

        for i in 0..(self.cols * self.rows) {
            self.data[i] += rhs.data[i]
        } 
    }
}

impl ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("Matrices incompatible for multiplication: {}x{} and {}x{}",
                   self.rows, self.cols,
                   rhs.rows, rhs.cols)
        }  

        Matrix { 
            data: {
                let mut data = vec![0.0; self.rows * rhs.cols];

                for i in 0..self.rows {
                    for j in 0..rhs.cols {
                        for k in 0..self.cols {
                            data[i * self.cols + j] += self[(i, k)] * rhs[(k, j)];
                        }
                    }
                }

                data
            }, 

            cols: rhs.cols, rows: self.rows 
        }
    }
}

impl ops::MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        if self.cols != rhs.rows {
            panic!("Matrices incompatible for multiplication: {}x{} and {}x{}",
                   self.rows, self.cols,
                   rhs.rows, rhs.cols)
        }  

        self.data = {
            let mut data = vec![0.0; self.rows * rhs.cols];

            for i in 0..self.rows {
                for j in 0..rhs.cols {
                    for k in 0..self.cols {
                        data[i * self.cols + j] += self[(i, k)] * rhs[(k, j)];
                    }
                }
            }

            data
        };

        self.cols = rhs.cols
    }
}

impl ops::Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix {
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);

                for i in 0..(self.cols * self.rows) {
                    data[i] = self.data[i] * rhs
                }

                data
            },

            ..self
        }
    }
}

impl ops::MulAssign<f32> for Matrix {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..(self.cols * self.rows) {
            self.data[i] += rhs
        } 
    }
}

