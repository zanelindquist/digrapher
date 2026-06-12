use yew::prelude::*;

use crate::{render::styles::OperatorStyle, services::{digraph_services::types::CanvasPositioning, matrix_services::types::{TermTypes, MathError}, objects::{matrix::Matrix, scalar::Scalar}}};

#[derive(PartialEq, Clone)]
pub enum OperatorType {
    Addition(AdditionOperator),
    Subtraction(SubtractionOperator),
    Division(DivisionOperator),
    Multiplication(MultiplicationOperator),
    Exponentiation(ExponentiationOperator),
    CrossProduct(CrossProductOperator),
    BooleanMultiplication(BooleanMultiplicationOperator),
    DotProduct(DotProductOperator),
    LogicalAnd(LogicalAndOperator),
    LogicalOr(LogicalOrOperator),
    Transpose(TransposeOperator),
    Determinate(DeterminateOperator),
}

impl OperatorType {
    pub fn get_base_operator(&self) -> BaseOperator {
        match self {
            OperatorType::Addition(op) => op.base.clone(),
            OperatorType::Subtraction(op) => op.base.clone(),
            OperatorType::Division(op) => op.base.clone(),
            OperatorType::Multiplication(op) => op.base.clone(),
            OperatorType::Exponentiation(op) => op.base.clone(),
            OperatorType::CrossProduct(op) => op.base.clone(),
            OperatorType::BooleanMultiplication(op) => op.base.clone(),
            OperatorType::DotProduct(op) => op.base.clone(),
            OperatorType::LogicalAnd(op) => op.base.clone(),
            OperatorType::LogicalOr(op) => op.base.clone(),
            OperatorType::Transpose(op) => op.base.clone(),
            OperatorType::Determinate(op) => op.base.clone(),
        }
    }
}

pub struct OperatorPositioning {
    pub offset_vx: i32,
    pub offset_vy: i32,
}
impl OperatorPositioning {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            offset_vx: x as i32,
            offset_vy: y as i32
        }
    }
}


#[derive(PartialEq, Clone)]
pub struct BaseOperator {
    pub supported_operands: Vec<(TermTypes, TermTypes)>,
    pub pemdas_level: i32,
    pub is_unary: bool,
    pub symbol: String,
    pub width: f32,
    pub height: f32,
    pub cursor_translate_l: (f32, f32)
}
impl BaseOperator {
    pub fn width(&self) -> f32 {
        self.width + self.cursor_translate_l.0
    }
    pub fn height(&self) -> f32 {
        self.height + self.cursor_translate_l.1
    }

    pub fn draw(&self, style: &OperatorStyle, position: &OperatorPositioning, canvas_pos: &CanvasPositioning) -> Html {
        let path = match self.symbol.as_str() {
            "+" => html! {
                <path d="M12 4V20M4 12H20" />
            },
            "-" => html! {
                <path d="M4 12H20" />
            },
            "*" => html! {
                <rect x="8.5" y="8.5" width="7" height="7" rx="1.5" fill={style.fill} stroke={style.fill} />
            },
            "/" => html! {
                // simple diagonal stroke for division
                <path d="M7 17 L17 7" />
            },
            "^" => html! {
                // caret-like exponent marker
                <path d="M7 15 L12 8 L17 15" />
            },
            "x" => html! {
                <path d="M6 6L18 18M18 6L6 18" />
            },
            "⊙" | "o" => html! {
                <>
                    <circle cx="12" cy="12" r="7" />
                    <circle cx="12" cy="12" r="2.5" />
                </>
            },
            "⋅" => html! {
                <circle cx="12" cy="12" r="3" />
            },
            "v" => html! {
                <path d="M6 8L12 16L18 8" />
            },
            "∧" => html! {
                <path d="M6 16L12 8L18 16" />
            },
            "trans" => html! {
                // stylized 'T' for transpose
                <>
                    <path d="M6 6 H18" />
                    <path d="M12 6 V18" />
                </>
            },

            "det" => html! {
                // vertical bars with a small 2x2 grid to suggest a matrix
                <>
                    <path d="M7 4 V20" />
                    <path d="M17 4 V20" />
                    <path d="M9 9 H15" />
                    <path d="M9 15 H15" />
                    <path d="M11 7 V17" />
                    <path d="M13 7 V17" />
                </>
            },

            _ => html! {},
        };

        html! {
            <svg
                viewBox="0 0 24 24"
                x={(position.offset_vx + canvas_pos.offset_x + canvas_pos.width / 2).to_string()}
                y={(position.offset_vy + canvas_pos.offset_y + canvas_pos.height / 2).to_string()}
                width={style.size.to_string()}
                height={style.size.to_string()}
                fill="none"
                stroke={style.fill}
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                {path}
            </svg>
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct AdditionOperator {
    pub base: BaseOperator
}
impl AdditionOperator {
    pub fn s_s(s1: &Scalar, s2: &Scalar) -> Result<Scalar, MathError> {
        Ok(Scalar::from_f64(s1.value + s2.value))
    }

    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(MathError::new(&format!("Dimension mismatch: {}x{} vs {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let mut result = Matrix::create(m1.rows, m1.cols);
        for i in 0..m1.rows as usize {
            for j in 0..m1.cols as usize {
                result.data[i][j] = m1.data[i][j] + m2.data[i][j];
            }
        }
        Ok(result)
    }

    pub fn m_s(m: &Matrix, s: &Scalar) -> Result<Matrix, MathError> {
        let mut result = Matrix::create(m.rows, m.cols);
        for i in 0..m.rows as usize {
            for j in 0..m.cols as usize {
                result.data[i][j] = m.data[i][j] + s.value;
            }
        }
        Ok(result)
    }

    pub fn s_m(s: &Scalar, m: &Matrix) -> Result<Matrix, MathError> {
        Self::m_s(m, s)
    }
}

#[derive(PartialEq, Clone)]
pub struct SubtractionOperator {
    pub base: BaseOperator,
}
impl SubtractionOperator {
    pub fn s_s(s1: &Scalar, s2: &Scalar) -> Result<Scalar, MathError> {
        Ok(Scalar::from_f64(s1.value - s2.value))
    }

    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(MathError::new(&format!("Dimension mismatch: {}x{} vs {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let mut result = Matrix::create(m1.rows, m1.cols);
        for i in 0..m1.rows as usize {
            for j in 0..m1.cols as usize {
                result.data[i][j] = m1.data[i][j] - m2.data[i][j];
            }
        }
        Ok(result)
    }

    pub fn m_s(m: &Matrix, s: &Scalar) -> Result<Matrix, MathError> {
        let mut result = Matrix::create(m.rows, m.cols);
        for i in 0..m.rows as usize {
            for j in 0..m.cols as usize {
                result.data[i][j] = m.data[i][j] - s.value;
            }
        }
        Ok(result)
    }

    pub fn s_m(s: &Scalar, m: &Matrix) -> Result<Matrix, MathError> {
        let mut result = Matrix::create(m.rows, m.cols);
        for i in 0..m.rows as usize {
            for j in 0..m.cols as usize {
                result.data[i][j] = s.value - m.data[i][j];
            }
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct DivisionOperator {
    pub base: BaseOperator,
}
impl DivisionOperator {
    pub fn s_s(s1: &Scalar, s2: &Scalar) -> Result<Scalar, MathError> {
        if s2.value.abs() < f64::EPSILON {
            return Err(MathError::new("Division by zero"));
        }
        Ok(Scalar::from_f64(s1.value / s2.value))
    }
}

#[derive(PartialEq, Clone)]
pub struct MultiplicationOperator {
    pub base: BaseOperator,
}
impl MultiplicationOperator {
    pub fn s_s(s1: &Scalar, s2: &Scalar) -> Result<Scalar, MathError> {
        Ok(Scalar::from_f64(s1.value * s2.value))
    }

    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        if m1.cols != m2.rows {
            return Err(MathError::new(&format!("Incompatible dimensions for multiplication: {}x{} and {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let mut result = Matrix::create(m1.rows, m2.cols);
        for i in 0..m1.rows as usize {
            for j in 0..m2.cols as usize {
                let mut sum = 0.0;
                for k in 0..m1.cols as usize {
                    sum += m1.data[i][k] * m2.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        Ok(result)
    }

    pub fn m_s(m: &Matrix, s: &Scalar) -> Result<Matrix, MathError> {
        let mut result = Matrix::create(m.rows, m.cols);
        for i in 0..m.rows as usize {
            for j in 0..m.cols as usize {
                result.data[i][j] = m.data[i][j] * s.value;
            }
        }
        Ok(result)
    }

    pub fn s_m(s: &Scalar, m: &Matrix) -> Result<Matrix, MathError> {
        Self::m_s(m, s)
    }
}

#[derive(PartialEq, Clone)]
pub struct ExponentiationOperator {
    pub base: BaseOperator,
}
impl ExponentiationOperator {
    pub fn s_s(base: &Scalar, exponent: &Scalar) -> Result<Scalar, MathError> {
        Ok(Scalar::from_f64(base.value.powf(exponent.value)))
    }

    pub fn m_s(matrix: &Matrix, exponent: &Scalar) -> Result<Matrix, MathError> {
        let exponent_int = exponent.value.round() as i32;
        if exponent.value.fract().abs() > f64::EPSILON {
            return Err(MathError::new("Exponent must be an integer"));
        }
        if exponent_int < 0 {
            return Err(MathError::new("Negative exponents not supported"));
        }
        if matrix.rows != matrix.cols {
            return Err(MathError::new(&format!("Matrix must be square, got {}x{}", matrix.rows, matrix.cols)));
        }

        if exponent_int == 0 {
            let mut result = Matrix::create(matrix.rows, matrix.cols);
            for i in 0..matrix.rows as usize {
                result.data[i][i] = 1.0;
            }
            return Ok(result);
        }

        let mut result = matrix.clone();
        for _ in 1..exponent_int {
            result = MultiplicationOperator::m_m(&result, matrix)?;
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct CrossProductOperator {
    pub base: BaseOperator,
}
impl CrossProductOperator {
    fn to_vector3(matrix: &Matrix) -> Option<[f64; 3]> {
        if matrix.rows == 1 && matrix.cols == 3 {
            Some([matrix.data[0][0], matrix.data[0][1], matrix.data[0][2]])
        } else if matrix.rows == 3 && matrix.cols == 1 {
            Some([matrix.data[0][0], matrix.data[1][0], matrix.data[2][0]])
        } else {
            None
        }
    }

    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        let a = Self::to_vector3(m1);
        let b = Self::to_vector3(m2);
        if a.is_none() || b.is_none() {
            return Err(MathError::new(&format!("Cross product requires 3D vectors, got {}x{} and {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let a = a.unwrap();
        let b = b.unwrap();
        let cross = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];

        let mut result = if m1.rows == 1 { Matrix::create(1, 3) } else { Matrix::create(3, 1) };
        if result.rows == 1 {
            for j in 0..3 {
                result.data[0][j] = cross[j];
            }
        } else {
            for i in 0..3 {
                result.data[i][0] = cross[i];
            }
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct BooleanMultiplicationOperator {
    pub base: BaseOperator,
}
impl BooleanMultiplicationOperator {
    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        if m1.cols != m2.rows {
            return Err(MathError::new(&format!("Incompatible dimensions for boolean multiplication: {}x{} and {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let mut result = Matrix::create(m1.rows, m2.cols);
        for i in 0..m1.rows as usize {
            for j in 0..m2.cols as usize {
                let mut value = 0.0;
                for k in 0..m1.cols as usize {
                    if m1.data[i][k] != 0.0 && m2.data[k][j] != 0.0 {
                        value = 1.0;
                        break;
                    }
                }
                result.data[i][j] = value;
            }
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct DotProductOperator {
    pub base: BaseOperator,
}
impl DotProductOperator {
    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Scalar, MathError> {
        let mut total = 0.0;
        if m1.rows == m2.rows && m1.cols == m2.cols {
            for i in 0..m1.rows as usize {
                for j in 0..m1.cols as usize {
                    total += m1.data[i][j] * m2.data[i][j];
                }
            }
            return Ok(Scalar::from_f64(total));
        }

        if m1.rows == 1 && m2.cols == 1 && m1.cols == m2.rows {
            for k in 0..m1.cols as usize {
                total += m1.data[0][k] * m2.data[k][0];
            }
            return Ok(Scalar::from_f64(total));
        }

        if m1.cols == 1 && m2.rows == 1 && m1.rows == m2.cols {
            for k in 0..m1.rows as usize {
                total += m1.data[k][0] * m2.data[0][k];
            }
            return Ok(Scalar::from_f64(total));
        }

        Err(MathError::new(&format!("Incompatible dimensions for dot product: {}x{} and {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)))
    }
}

#[derive(PartialEq, Clone)]
pub struct LogicalAndOperator {
    pub base: BaseOperator,
}
impl LogicalAndOperator {
    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(MathError::new(&format!("Dimension mismatch: {}x{} vs {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let mut result = Matrix::create(m1.rows, m1.cols);
        for i in 0..m1.rows as usize {
            for j in 0..m1.cols as usize {
                result.data[i][j] = if m1.data[i][j] != 0.0 && m2.data[i][j] != 0.0 { 1.0 } else { 0.0 };
            }
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct LogicalOrOperator {
    pub base: BaseOperator,
}
impl LogicalOrOperator {
    pub fn m_m(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MathError> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(MathError::new(&format!("Dimension mismatch: {}x{} vs {}x{}", m1.rows, m1.cols, m2.rows, m2.cols)));
        }

        let mut result = Matrix::create(m1.rows, m1.cols);
        for i in 0..m1.rows as usize {
            for j in 0..m1.cols as usize {
                result.data[i][j] = if m1.data[i][j] != 0.0 || m2.data[i][j] != 0.0 { 1.0 } else { 0.0 };
            }
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct TransposeOperator {
    pub base: BaseOperator,
}
impl TransposeOperator {
    pub fn m(matrix: &Matrix) -> Result<Matrix, MathError> {
        let mut result = Matrix::create(matrix.cols, matrix.rows);
        for i in 0..matrix.rows as usize {
            for j in 0..matrix.cols as usize {
                result.data[j][i] = matrix.data[i][j];
            }
        }
        Ok(result)
    }
}

#[derive(PartialEq, Clone)]
pub struct DeterminateOperator {
    pub base: BaseOperator,
}
impl DeterminateOperator {
    pub fn m(matrix: &Matrix) -> Result<Scalar, MathError> {
        if matrix.rows != matrix.cols {
            return Err(MathError::new(&format!("Matrix must be square, got {}x{}", matrix.rows, matrix.cols)));
        }

        Ok(Scalar::from_f64(Self::determinant(&matrix.data)))
    }

    fn determinant(data: &[Vec<f64>]) -> f64 {
        match data.len() {
            0 => 0.0,
            1 => data[0][0],
            2 => data[0][0] * data[1][1] - data[0][1] * data[1][0],
            n => {
                let mut det = 0.0;
                for col in 0..n {
                    let sign = if col % 2 == 0 { 1.0 } else { -1.0 };
                    let mut minor = Vec::with_capacity(n - 1);
                    for row in 1..n {
                        let mut minor_row = Vec::with_capacity(n - 1);
                        for c in 0..n {
                            if c != col {
                                minor_row.push(data[row][c]);
                            }
                        }
                        minor.push(minor_row);
                    }
                    det += sign * data[0][col] * Self::determinant(&minor);
                }
                det
            }
        }
    }
}