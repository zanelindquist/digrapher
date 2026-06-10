use std::collections::HashMap;
use crate::services::{
    matrix_services::operators::*,
    matrix_services::types::TermTypes,
};

pub fn get_supported_operator_by_symbol(operator: &str) -> Option<OperatorType> {
    get_supported_operators().get(operator).cloned()
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
                symbol: "/".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.0, -1.0),
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
                symbol: "⋅".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.5, 0.0),
            },
        }),
    );

    // Dot Product (.) - alias for ⋅
    operators.insert(
        ".".to_string(),
        OperatorType::DotProduct(DotProductOperator {
            base: BaseOperator {
                supported_operands: vec![(TermTypes::MATRIX, TermTypes::MATRIX)],
                pemdas_level: 2,
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
                symbol: "trans".to_string(),
                width: 1.0,
                height: 1.0,
                cursor_translate_l: (0.25, 0.0),
            },
        }),
    );

    operators
}
