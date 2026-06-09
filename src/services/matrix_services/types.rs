use std::iter::empty;
use yew::prelude::*;
use regex::Regex;

use crate::services::objects::{matrix::Matrix, scalar::Scalar};

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
    pub fn from_text(text: String) -> Self {
        let mut new = Self { raw_text: text, terms: vec![] };
        new.parse_terms();
        new
    }

    pub fn parse_terms(&mut self) {
        let digit_selector = Regex::new(r"\\matrix\{([^}]+)\}|(\d+\.?\d*)").unwrap();

        for mat in digit_selector.find_iter(&self.raw_text) {
            gloo_console::log!(mat.as_str());
            // Process a scalar
            if let Ok(scalar) = mat.as_str().parse::<f64>() {
                self.terms.push(Term::Scalar(
                    Scalar::from_f64(scalar)
                ));
            }
            // Process a matrix
            else {

            }
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