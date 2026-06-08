use yew::prelude::*;

use crate::services::objects::{matrix::Matrix, scalar::Scalar};

// ENUMS

#[derive(PartialEq)]
pub enum Term {
    Marix(Matrix),
    Scalar(Scalar)
}


// TYPES

pub type MatrixEquasionResult = Result<MatrixEquation, ParseError>;
pub type EvaluationTerm = (i32, String);

// STRUCTS

#[derive(Properties, PartialEq)]
pub struct MatrixEquation {
    pub raw_text: String,
    pub terms: Vec<Term>
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