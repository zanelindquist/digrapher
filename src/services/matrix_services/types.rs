use yew::prelude::*;
use regex::Regex;

use crate::{render::styles::MathErrorStyle, services::{digraph_services::types::CanvasPositioning, matrix_services::operators::BaseOperator, matrix_services::supported_operators, objects::{matrix::Matrix, scalar::Scalar}}};

// ENUMS
#[derive(PartialEq, Clone)]
pub enum TermTypes{ MATRIX, SCALAR }

#[derive(PartialEq, Clone)]
pub enum Term {
    Matrix(Matrix),
    Scalar(Scalar),
    Operator(BaseOperator),
    Error(MathError)
}


// TYPES

pub type MatrixEquationResult = Result<MatrixEquation, ParseError>;
pub type EvaluationTerm = (i32, String);

// STRUCTS

#[derive(Properties, PartialEq)]
pub struct ObjectSelection {
    pub selected_term: Option<Term>,
    pub hovered_term: Option<Term>
}
impl Default for ObjectSelection {
    fn default() -> Self {
        Self { selected_term: None, hovered_term: None }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MatrixEquation {
    pub raw_text: String,
    pub terms: Vec<Term>
}
impl MatrixEquation {
    pub fn default() -> Self {
        Self { raw_text: String::default(), terms: vec![] }
    }
    pub fn from_text(text: String) -> Result<Self, ParseError> {
        let mut new = Self { raw_text: text, terms: vec![] };
        match new.parse_terms() {
            Ok(terms) => {
                new.terms = terms;
                Ok(new)
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn parse_terms(&self) -> Result<Vec<Term>, ParseError> {
        let mut terms: Vec<Term> = vec![];
        // Matrix | Scalar | Binary Operators | Unary Operators | \mat... Incompleted | Text | Unupported operators
        let digit_selector = Regex::new(r"\\matrix\{([^}]+)\}|(\d+\.?\d*)|([\+\-/\*\^xov∨∧⊙⋅\.])|(det|trans)|\\[A-Za-z]{1,7}\{?[^}]*|(\w+)|(\W+)").unwrap();

        let removed_whitespace = self.raw_text.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        for (index, capture) in digit_selector.captures_iter(&removed_whitespace).into_iter().enumerate() {
            // Add an error if we have unexpected text and continue
            if let Some(_incomplete_escape) = capture.get(5) {
                continue
            }
            if let Some(_text) = capture.get(6) {
                terms.push(Term::Error(MathError::new("Unexpected text")));
                continue
            }
            if let Some(_unsupported_operators) = capture.get(7) {
                terms.push(Term::Error(MathError::new("Unexpected operator")));
                continue
            }
            // Parse real terms
            if let Some(matrix) = capture.get(1) {
                // Partition the dimentions and the values
                let partition: Vec<&str> = matrix.as_str().split("),").collect();
                if partition.len() != 2 {
                    return Err(ParseError::new("Partition size must be 2. E.X. \\matrix{(2, 3), (1, 2, 3, 4, 5, 6)}"))
                }
                // Get the dimentions
                let dims = partition
                    .get(0)
                    .ok_or(ParseError::new("Missing dimensions"))?
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .parse::<i32>()
                            .map_err(|_| ParseError::new("Row or column is not an integer"))
                    })
                    .collect::<Result<Vec<i32>, ParseError>>()?;

                if dims.len() != 2 {
                    return Err(ParseError::new("Dimensions must be exactly two integers, e.g. (4, 3)"));
                }

                let (rows, cols) = (dims[0], dims[1]);

                // Extract values
                let values = partition.get(1)
                    .unwrap()
                    .trim()
                    .trim_matches(|v| v == '(' || v == ')')
                    .split(",")
                    .map(|s| {
                        s.trim()
                            .parse::<f64>()
                            .map_err(|_| ParseError::new("Value is not an integer"))
                    })
                     .collect::<Result<Vec<f64>, ParseError>>()?;

                if values.len() as i32 != rows * cols {
                    return Err(ParseError::new("Matrix is improperly filled: values does not equal rows times columns"));
                }

                terms.push(Term::Matrix(
                    crate::services::objects::matrix::Matrix::from_values((rows, cols), values)
                ))
            }
            if let Some(number) = capture.get(2) {
                if let Ok(scalar) = number.as_str().parse::<f64>() {
                    terms.push(Term::Scalar(
                        Scalar::from_f64(scalar)
                    ));
                }
            }
            if let Some(operator) = capture.get(3) {
                let supported_ops = supported_operators::get_supported_operators();
                if let Some(op_type) = supported_ops.get(operator.as_str()) {
                    terms.push(Term::Operator(op_type.get_base_operator()));
                }
            }
            if let Some(unary_operator) = capture.get(4) {
                let supported_ops = supported_operators::get_supported_operators();
                if let Some(op_type) = supported_ops.get(unary_operator.as_str()) {
                    terms.push(Term::Operator(op_type.get_base_operator()));
                }
            }
        
            // Check the term with the previous term to see if the two are compatible
            // If we are the first or second term, just continue because you cant make an equation out of that (TODO: UNLESS ITS UNARY)
            if index == 0 || index == 1 {
                continue;
            }

            // If there is an error in compatiblity, add it
            if let Err(math_error) = self.check_term_compatibility(terms.get(terms.len() - 3).unwrap(),terms.get(terms.len() - 2).unwrap(), terms.get(terms.len() - 1).unwrap()) {
                terms.push(Term::Error(math_error));
            }
        }

        Ok(terms)
    }

    pub fn check_term_compatibility(&self, _term1: &Term, operator: &Term, _term2: &Term) -> Result<bool, MathError> {
        if let Term::Operator(operator) = operator {
            if let Some(specific_op) = supported_operators::get_supported_operator_by_symbol(operator.symbol.as_str()) {
                
                Ok(true)
            } else {
                return Err(MathError::new("Unexpected operator"))
            }
        } else {
            Ok(true)
        }
    }
}


// ERRORS

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub message: String,
}
impl ParseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub struct MathErrorPositioning {
    pub offset_vx: i32,
    pub offset_vy: i32,
}
impl MathErrorPositioning {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            offset_vx: x as i32,
            offset_vy: y as i32
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MathError {
    pub message: String,
}
impl MathError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn width(&self) -> f32 {
        self.message.len() as f32 * 0.5 + 0.3
    }
    pub fn height(&self) -> f32 {
        1.0
    }

    pub fn draw(&self, style: &MathErrorStyle, error_pos: &MathErrorPositioning, canvas_pos: &CanvasPositioning) -> Html {
        let error_vx = error_pos.offset_vx as f32 - self.message.len() as f32 * style.size as f32 * 0.4 / 2.0;
        let error_vy = error_pos.offset_vy  + style.size  * 2;
        html! {
            <>
                <svg
                    viewBox="0 0 24 24"
                    x={(error_pos.offset_vx + canvas_pos.offset_x + canvas_pos.width / 2).to_string()}
                    y={(error_pos.offset_vy + canvas_pos.offset_y + canvas_pos.height / 2).to_string()}
                    width={style.size.to_string()}
                    height={style.size.to_string()}
                    fill="none"
                    stroke={style.fill}
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="M7 15 L12 8 L17 15" />
                </svg>
                <text
                    x={(canvas_pos.width / 2 + canvas_pos.offset_x + error_vx as i32).to_string()}
                    y={(canvas_pos.height / 2 + canvas_pos.offset_y + error_vy).to_string()}
                    font-size={(style.size as f32 * canvas_pos.zoom).to_string()}
                    fill={style.fill}
                >
                    { self.message.clone() }
                </text>
            </>
        }

    }
}