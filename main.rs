use std::ops::IndexMut;
use std::ops::Index;
use rand::Rng;
use std::time::{Duration, Instant};

struct Matrix {
    cols: usize,
    rows: usize,
    data: Vec<f64>
}

impl Matrix {
    fn new(nrows: usize, ncols: usize) -> Self {
        let mut data_vec: Vec<f64> = Vec::new();
        data_vec.resize(ncols * nrows, 0.0);
        let mat = Matrix{data: data_vec, cols: ncols, rows: nrows};
        mat
    }

    fn new_rand_matrix(nrows: usize, ncols: usize, min_val: f64, max_val: f64) -> Self {
        let mut mat = Matrix::new(nrows, ncols);
        let mut rng = rand::thread_rng();
        for i in 0..nrows {
            for j in 0..ncols {
                mat[i][j] = rng.gen_range(min_val..max_val);
            }
        }
        mat
    }

    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn mut_data(&mut self) -> &mut Vec<f64> {
        &mut self.data
    }

    fn data(&self) -> &Vec<f64> {
        &self.data
    }

    fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols - 1 {
                // print!("{number:>5}", number=self.data[i * self.cols + j]);
                print!("{:.3} ", self.data[i * self.cols + j]);
            }
            // println!("{number:>5}", number=self.data[(i + 1) * self.cols - 1]);
            println!("{:.3}", self.data[(i + 1) * self.cols - 1]);
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols .. (index+1) * self.cols]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols .. (index+1) * self.cols]
    }
}


fn matmul1(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let (nrows1, ncols1) = mat1.shape();
    let (nrows2, ncols2) = mat2.shape();
    assert_eq!(ncols1, nrows2, "Matrices shape must be corresponded!");
    let mut res_mat = Matrix::new(nrows1, ncols2);
    let mat1_ptr = mat1.data();
    let mat2_ptr = mat2.data();
    let res_mat_ptr = res_mat.mut_data();
    let start = Instant::now();
    for i in 0..nrows1 {
        for j in 0..ncols2 {
            for k in 0..nrows2 {
                res_mat_ptr[i * ncols2 + j] += mat1_ptr[i * ncols1 + k] + mat2_ptr[k * ncols2 + j];
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed matmul1 is: {:?}", duration);
    res_mat
}


fn matmul2(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let (nrows1, ncols1) = mat1.shape();
    let (nrows2, ncols2) = mat2.shape();
    assert_eq!(ncols1, nrows2, "Matrices shape must be corresponded!");
    let mut res_mat = Matrix::new(nrows1, ncols2);
    let start = Instant::now();
    for i in 0..nrows1 {
        for j in 0..ncols2 {
            for k in 0..nrows2 {
                res_mat[i][j] += mat1[i][k] + mat2[k][j];
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed matmul2 is: {:?}", duration);
    res_mat
}



fn matmul3(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let (nrows1, ncols1) = mat1.shape();
    let (nrows2, ncols2) = mat2.shape();
    assert_eq!(ncols1, nrows2, "Matrices shape must be corresponded!");
    let mut res_mat = Matrix::new(nrows1, ncols2);
    let start = Instant::now();
    unsafe {
        let mat1_ref = mat1.data().as_ptr();
        let mat1_ptr = mat1_ref as *mut f64;
        let mat2_ref = mat2.data().as_ptr();
        let mat2_ptr = mat2_ref as *mut f64;
        let res_mat_ptr = res_mat.mut_data().as_mut_ptr();
        
        for i in 0..nrows1 {
            for j in 0..ncols2 {
                for k in 0..nrows2 {
                    *(res_mat_ptr.add(i * ncols2 + j)) += *(mat1_ptr.add(i * ncols1 + k)) + *(mat2_ptr.add(k * ncols2 + j));
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed matmul3 is: {:?}", duration);
    res_mat
}


fn main() {
    let mat1 = Matrix::new_rand_matrix(900, 1000, 1.0, 5.0);
    let mat2 = Matrix::new_rand_matrix(1000, 1200, 6.0, 9.0);
    // let mat3 = matmul1(&mat1, &mat2);
    let mat4 = matmul2(&mat1, &mat2);
    let mat5 = matmul3(&mat1, &mat2);
}
