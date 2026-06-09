use crate::services::objects::matrix::Matrix;

#[derive(PartialEq, Clone)]
pub struct Scalar {
    value: f32,
}
impl Scalar {
    pub fn from_f32(f: f32) -> Self {
        Self {
            value: f
        }
    }

    pub fn add_s(&self, s: &Scalar) -> Scalar {
        Scalar::from_f32(self.value + s.value)
    }
    pub fn add_m(&self, m: &Matrix) -> Matrix {
        let mut matrix = Matrix::create(m.rows, m.cols);
        for row in matrix.data.iter_mut() {
            for val in row.iter_mut() {
                *val += self.value;
            }
        }
        matrix
    }
}