use std::collections::HashMap;
use crate::services::{
    matrix_services::operators::*,
    matrix_services::types::{MathError, Term, TermTypes},
};

pub fn get_supported_operator_by_symbol(operator: &str) -> Option<OperatorType> {
    get_supported_operators().get(operator).cloned()
}

pub fn evaluate_binary_operator(term1: &Term, operator: &Term, term2: &Term) -> Result<Term, MathError> {
    let operator_type = match operator {
        Term::Operator(base_op) => {
            get_supported_operator_by_symbol(&base_op.symbol)
                .ok_or_else(|| MathError::new("Unknown operator"))?
        }
        _ => return Err(MathError::new("Expected operator")),
    };

    match (&operator_type, term1, term2) {
        (OperatorType::Addition(_), Term::Scalar(a), Term::Scalar(b)) => {
            Ok(Term::Scalar(AdditionOperator::s_s(a, b)?))
        }
        (OperatorType::Addition(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(AdditionOperator::m_m(a, b)?))
        }
        (OperatorType::Addition(_), Term::Matrix(a), Term::Scalar(b)) => {
            Ok(Term::Matrix(AdditionOperator::m_s(a, b)?))
        }
        (OperatorType::Addition(_), Term::Scalar(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(AdditionOperator::s_m(a, b)?))
        }

        (OperatorType::Subtraction(_), Term::Scalar(a), Term::Scalar(b)) => {
            Ok(Term::Scalar(SubtractionOperator::s_s(a, b)?))
        }
        (OperatorType::Subtraction(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(SubtractionOperator::m_m(a, b)?))
        }
        (OperatorType::Subtraction(_), Term::Matrix(a), Term::Scalar(b)) => {
            Ok(Term::Matrix(SubtractionOperator::m_s(a, b)?))
        }
        (OperatorType::Subtraction(_), Term::Scalar(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(SubtractionOperator::s_m(a, b)?))
        }

        (OperatorType::Division(_), Term::Scalar(a), Term::Scalar(b)) => {
            Ok(Term::Scalar(DivisionOperator::s_s(a, b)?))
        }

        (OperatorType::Multiplication(_), Term::Scalar(a), Term::Scalar(b)) => {
            Ok(Term::Scalar(MultiplicationOperator::s_s(a, b)?))
        }
        (OperatorType::Multiplication(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(MultiplicationOperator::m_m(a, b)?))
        }
        (OperatorType::Multiplication(_), Term::Matrix(a), Term::Scalar(b)) => {
            Ok(Term::Matrix(MultiplicationOperator::m_s(a, b)?))
        }
        (OperatorType::Multiplication(_), Term::Scalar(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(MultiplicationOperator::s_m(a, b)?))
        }

        (OperatorType::Exponentiation(_), Term::Scalar(a), Term::Scalar(b)) => {
            Ok(Term::Scalar(ExponentiationOperator::s_s(a, b)?))
        }
        (OperatorType::Exponentiation(_), Term::Matrix(a), Term::Scalar(b)) => {
            Ok(Term::Matrix(ExponentiationOperator::m_s(a, b)?))
        }

        (OperatorType::CrossProduct(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(CrossProductOperator::m_m(a, b)?))
        }
        (OperatorType::BooleanMultiplication(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(BooleanMultiplicationOperator::m_m(a, b)?))
        }
        (OperatorType::DotProduct(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Scalar(DotProductOperator::m_m(a, b)?))
        }
        (OperatorType::LogicalAnd(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(LogicalAndOperator::m_m(a, b)?))
        }
        (OperatorType::LogicalOr(_), Term::Matrix(a), Term::Matrix(b)) => {
            Ok(Term::Matrix(LogicalOrOperator::m_m(a, b)?))
        }

        _ => Err(MathError::new("Unsupported operand combination")),
    }
}

pub fn get_supported_operators() -> HashMap<String, OperatorType> {
    let mut operators = HashMap::new();

    // Addition
    operators.insert(
        "+".to_string(),
        OperatorType::Addition(AdditionOperator {
            base: BaseOperator {
                supported_operands: vec![
                    (TermTypes::SCALAR, TermTypes::SCALAR),
                    (TermTypes::MATRIX, TermTypes::MATRIX),
                    (TermTypes::MATRIX, TermTypes::SCALAR),
                    (TermTypes::SCALAR, TermTypes::MATRIX),
                ],
                pemdas_level: 1,
                is_unary: false,
                symbol: "+".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Subtraction
    operators.insert(
        "-".to_string(),
        OperatorType::Subtraction(SubtractionOperator {
            base: BaseOperator {
                supported_operands: vec![
                    (TermTypes::SCALAR, TermTypes::SCALAR),
                    (TermTypes::MATRIX, TermTypes::MATRIX),
                    (TermTypes::MATRIX, TermTypes::SCALAR),
                ],
                pemdas_level: 1,
                is_unary: false,
                symbol: "-".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Division
    operators.insert(
        "/".to_string(),
        OperatorType::Division(DivisionOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::SCALAR, TermTypes::SCALAR)],
                pemdas_level: 2,
                is_unary: false,
                symbol: "/".to_string(),
                width: 0.5,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Multiplication
    operators.insert(
        "*".to_string(),
        OperatorType::Multiplication(MultiplicationOperator {
            base: BaseOperator {
                supported_operands: vec![
                    (TermTypes::SCALAR, TermTypes::SCALAR),
                    (TermTypes::MATRIX, TermTypes::MATRIX),
                    (TermTypes::MATRIX, TermTypes::SCALAR),
                    (TermTypes::SCALAR, TermTypes::MATRIX),
                ],
                pemdas_level: 2,
                is_unary: false,
                symbol: "*".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Exponentiation
    operators.insert(
        "^".to_string(),
        OperatorType::Exponentiation(ExponentiationOperator {
            base: BaseOperator {
                supported_operands: vec![
                    (TermTypes::SCALAR, TermTypes::SCALAR),
                    (TermTypes::MATRIX, TermTypes::SCALAR),
                ],
                pemdas_level: 3,
                is_unary: false,
                symbol: "^".to_string(),
                width: 0.0,
                height: 0.0,
                cursor_translate_l: (0.3, 0.6),
            },
        }),
    );

    // Cross Product
    operators.insert(
        "x".to_string(),
        OperatorType::CrossProduct(CrossProductOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 2,
                is_unary: false,
                symbol: "x".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Boolean Multiplication (⊙)
    operators.insert(
        "⊙".to_string(),
        OperatorType::BooleanMultiplication(BooleanMultiplicationOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 2,
                is_unary: false,
                symbol: "⊙".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Boolean Multiplication (o) - alias for ⊙
    operators.insert(
        "o".to_string(),
        OperatorType::BooleanMultiplication(BooleanMultiplicationOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 2,
                is_unary: false,
                symbol: "⊙".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Dot Product (⋅)
    operators.insert(
        "⋅".to_string(),
        OperatorType::DotProduct(DotProductOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 2,
                is_unary: false,
                symbol: "⋅".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Logical Or (∨)
    operators.insert(
        "∨".to_string(),
        OperatorType::LogicalOr(LogicalOrOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 0,
                is_unary: false,
                symbol: "∨".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Logical Or (v) - alias for ∨
    operators.insert(
        "v".to_string(),
        OperatorType::LogicalOr(LogicalOrOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 0,
                is_unary: false,
                symbol: "∨".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Logical And (∧)
    operators.insert(
        "∧".to_string(),
        OperatorType::LogicalAnd(LogicalAndOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 0,
                is_unary: false,
                symbol: "∧".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Determinant
    operators.insert(
        "det".to_string(),
        OperatorType::Determinate(DeterminateOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 3,
                is_unary: true,
                symbol: "det".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Transpose
    operators.insert(
        "trans".to_string(),
        OperatorType::Transpose(TransposeOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 3,
                is_unary: true,
                symbol: "trans".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.25, 0.0),
            },
        }),
    );

    operators
}
