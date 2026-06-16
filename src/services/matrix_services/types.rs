use core::fmt;

use js_sys::Math;
use yew::prelude::*;
use regex::Regex;

use crate::{render::styles::MathErrorStyle, services::{digraph_services::types::CanvasPositioning, matrix_services::{operators::{BaseOperator, OperatorType}, supported_operators::{self, evaluate_binary_operator, get_supported_operators}}, objects::{matrix::Matrix, scalar::Scalar}}};

// ENUMS
#[derive(PartialEq, Clone)]
pub enum TermTypes{ MATRIX, SCALAR }
impl fmt::Display for TermTypes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TermTypes::MATRIX => "matrix",
            TermTypes::SCALAR => "scalar",
        };

        write!(f, "{}", msg)
    }
}

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
        /*
            Returns a result that is either a Vec of Term, or a Parse Error
            Parse error are thrown if the program hits an unrecoverable parsing error
            Math errors occur when there is a syntax issue that we want to clearly communicate to the user
            Math errors are included in the term vector so that they can be displayed to the user.

            While parsing, if the parser hits a math error, it will continue to the next term to evaluate.
            If it hits a parsing error, it will return immediately

            Parse Errors = Symantic errors
            Math Errors = Syntactic errors
        */


        let mut terms: Vec<Term> = vec![];
        // Matrix | Scalar | Binary Operators | Unary Operators | \mat... Incompleted | Text | Unupported operators
        let digit_selector = Regex::new(r"\\matrix\{([^}]+)\}|(\d+\.?\d*)|([\+\-/\*\^xov∨∧⊙⋅])|(det|trans)|\\[A-Za-z]{1,7}\{?[^}]*|(\w+)|(\W+)").unwrap();

        let removed_whitespace = self.raw_text.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        // Parse the text into terms
        for (index, capture) in digit_selector.captures_iter(&removed_whitespace).into_iter().enumerate() {
            // Add an error if we have unexpected text and continue
            if let Some(_incomplete_escape) = capture.get(5) {
                continue
            }
            if let Some(_text) = capture.get(6) {
                return Err(ParseError::new("Unexpected text"))
            }
            if let Some(_unsupported_operators) = capture.get(7) {
                return Err(ParseError::new("Unexpected operator"))
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
                    return Err(ParseError::new("Matrix dimensions must be exactly two integers, e.g. (4, 3)"));
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
        }


        // Now that the terms are parsed correctly, check for Math Errors (syntactic errors)
        // Check every pair to make sure we don't have invalid pairs
        // let mut errors_to_insert: Vec<(usize, MathError)> = vec![];
        for (index, term) in terms.iter().enumerate() {
            // Check for term compatibility
            if index == 0 { continue }
            if let Err(math_error) = self.check_term_compatibility(terms.get(index - 1).unwrap(), term) {
                // If we hit an error, break so that we only display one at a time and don't snowball a ton
                terms.insert(index, Term::Error(math_error));
                break;
            }

            // Check for binary operator compatibility
            if index == 1 || index == terms.len() - 1 { continue }
            if let Term::Operator(operator) = term {
                if !operator.is_unary {
                    if let Err(math_error) = self.check_binary_operator_compatibility(terms.get(index - 1).unwrap(), term, terms.get(index + 1).unwrap()) {
                        terms.insert(index, Term::Error(math_error));
                        break;
                    }
                }
            }
        }

        Ok(terms)
    }

    // Checks the order of two terms and makes sure its ok
    pub fn check_term_compatibility(&self, term1: &Term, term2: &Term) -> Result<bool, MathError> {
        // Don't let two consecutive vectors or scalars exist
        if (matches!(term1, Term::Scalar(_)) || matches!(term1, Term::Matrix(_)))
        && (matches!(term2, Term::Scalar(_)) || matches!(term2, Term::Matrix(_))) {
            Err(MathError::new("Missing an operator"))
        }
        // If there are two operators in a row, and the second is not unary, throw an error
        else if matches!(term1, Term::Operator(_)) && matches!(term2, Term::Operator(op) if !op.is_unary) {
            Err(MathError::new("Missing a term"))
        } else {
            Ok(true)
        }
    }

    pub fn check_binary_operator_compatibility(&self, term1: &Term, operator: &Term, term2: &Term) -> Result<bool, MathError> {
        // If the middle one is not an operator, crash the program
        assert_eq!(matches!(operator, Term::Operator(_)), true);
        // Return if one of the terms is an operator (we have run into a unary operator, which isn't supported yet)
        if matches!(term1, Term::Operator(_)) || matches!(term2, Term::Operator(_)) {
            return Err(MathError::new("Unary operator encountered"))
        }

        if let Term::Operator(op) = operator {
            // Crash the program if this is not a binary operator
            assert_ne!(op.is_unary, true);

            let term1_type = if matches!(term1, Term::Matrix(_)) {TermTypes::MATRIX} else {TermTypes::SCALAR};
            let term2_type = if matches!(term2, Term::Matrix(_)) {TermTypes::MATRIX} else {TermTypes::SCALAR};

            // Check if the operands are compatible
           if op.supported_operands.iter()
            .find(|supported| supported.0 == term1_type && supported.1 == term2_type)
            .is_none() {
                return Err(MathError::new(format!("Operand mismatch: '{}' does not support types: '{}' and '{}'", op.symbol, term1_type, term2_type).as_str()))
           }

            // Perform the operation to see if there are any other errors
            if let Err(math_error) = supported_operators::evaluate_binary_operator(term1, operator, term2) {
                return Err(math_error)
            }
        }

        Ok(true)
    }

    pub fn evaluate(&self) -> Result<Term, MathError> {
        match &mut self.parse_terms() {
            Ok(terms) => {
                // Check for math errors
                if let Some(Term::Error(math_error)) = terms.iter().find(|t| matches!(t, Term::Error(_))) {
                    return Err((*math_error).clone())
                }

                // Now that our vector of terms has no errors in it, we can begin to evaluate it

                // Evaluate unary operators

                // Evaluate binary operators according to pemdas
                let supported_ops = get_supported_operators();
                let mut operands = supported_ops
                    .iter()
                    .collect::<Vec<(&String, &OperatorType)>>();

                operands.sort_by(|a, b| (b.1.get_base_operator().pemdas_level).cmp(&a.1.get_base_operator().pemdas_level));

                for operand in operands {
                    loop {
                        let index = match terms.iter().position(|t| matches!(t, Term::Operator(o) if o.symbol.eq(operand.0))) {
                            Some(i) => i,
                            None => break,
                        };

                        // If there isnt an n + 1 or n - 1, then we must return an error
                        if terms.get(index - 1).is_none() || terms.get(index + 1).is_none() {
                            return Err(MathError::new("Incomplete equation"))
                        }

                        // Remove n - 1, n, and n + 1 and feed them into the evaluation
                        let left = terms.remove(index - 1);
                        let op = terms.remove(index - 1);
                        let right = terms.remove(index - 1);
                        let result = evaluate_binary_operator(&left, &op, &right)?;
                        terms.insert(index - 1, result);
                    }
                }

                // Now the vector of terms should only be one term
                if terms.len() == 1 {
                    if let Some(term) = terms.first() {
                        return Ok((*term).clone())
                    }
                }

                return Err(MathError::new("More than one final term found"))

            },
            Err(parse_error) => {
                return Err(MathError::new("Evaluation failed"))
            }
        }

        Err(MathError::new("Evaluation failed"))
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