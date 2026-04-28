use gloo_console::log;
use std::collections::HashSet;

use crate::logic::types::{DigestedValuesResult, RawEdgePairs};

use super::types::{Relation, ParseError, RelationProperties};


pub fn digest_values(values: String) -> DigestedValuesResult {
    if values.is_empty() {
        return Ok(Relation {
            values: HashSet::new(),
            points: HashSet::new(),
            properties: RelationProperties {
                antisymmetric: true,
                symmetric: true,
                reflexive: true,
                transitive: true
            }
        });
    }

    // Validate outer braces
    let trimmed = values.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(ParseError {
            message: "Input must be wrapped in curly braces: {(...)}".to_string()
        });
    }

    // Parse inner content
    let inner = &trimmed[1..trimmed.len()-1];

    let mut points: HashSet<String> = Default::default();
    
    let pairs: RawEdgePairs = inner
        .split("),")
        .enumerate()
        .map(|(i, pair)| {
            let cleaned = pair
                .trim()
                .trim_matches(|c| c == ')' || c == '(');

            let mut parts = cleaned.split(',');
            
            // Check for exactly 2 parts
            let first = match parts.next() {
                Some(p) if !p.trim().is_empty() => p.trim().to_string(),
                _ => return Err(ParseError {
                    message: format!("Pair {} is missing first element", i + 1)
                }),
            };
            
            let second = match parts.next() {
                Some(p) if !p.trim().is_empty() => p.trim().to_string(),
                _ => return Err(ParseError {
                    message: format!("Pair {} is missing second element", i + 1)
                }),
            };
            
            // Check for extra elements
            if parts.next().is_some() {
                return Err(ParseError {
                    message: format!("Pair {} has too many elements (expected 2)", i + 1)
                });
            }

            // Add both points to the set
            points.insert(first.clone());
            points.insert(second.clone());

            Ok((first, second))
        })
        .collect::<Result<RawEdgePairs, _>>()?;

    Ok(Relation {
        values: pairs,
        points: points,
        properties: RelationProperties {
            antisymmetric: true,
            symmetric: true,
            reflexive: true,
            transitive: true
        }
    })
}