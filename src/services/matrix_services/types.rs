use std::iter::empty;
use yew::prelude::*;
use regex::Regex;

use crate::services::{objects::{scalar::Scalar, matrix::Matrix}};

// ENUMS

#[derive(PartialEq, Clone)]
pub enum Term {
    Matrix(Matrix),
    Scalar(Scalar)
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
        let digit_selector = Regex::new(r"\\matrix\{([^}]+)\}|(\d+\.?\d*)").unwrap();

        for mat in digit_selector.find_iter(&self.raw_text) {
            // Process a scalar
            if let Ok(scalar) = mat.as_str().parse::<f64>() {
                terms.push(Term::Scalar(
                    Scalar::from_f64(scalar)
                ));
            }
            // Process a matrix
            else if mat.len() > 5 {
                let trimmed = mat.as_str().trim();
                let inner = &trimmed[8..trimmed.len()-1];
                // Partition the dimentions and the values
                let partition: Vec<&str> = inner.split("),").collect();
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
            // Process an operator 
            else {

            }
        }

        Ok(terms)
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
}