use std::{collections::HashSet, ops::Deref};

use gloo_console::log;

use crate::logic::types::{DigestedValuesResult, ParseError, RawEdgePairs, Relation, RelationProperties};


pub fn digest_values(values: String) -> DigestedValuesResult {
    if values.is_empty() {
        return Ok(Relation {
            values: HashSet::new(),
            points: HashSet::new(),
            properties: RelationProperties {
                antisymmetric: true,
                symmetric: true,
                reflexive: false,
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

    let mut symmetric_set: HashSet<(String, String)> = HashSet::new();
    let mut reflexive_num = 0;
    let mut transitive = true;

    for (a, b) in pairs.iter() {
        // For symmetric, we just need to reduce a and b to an unordered pairing
        
        if a.deref().eq(b) {
            // Increment the reflexive number
            reflexive_num += 1;
        } else {
            let mut pair = (a.clone(), b.clone());

            // Normalize order
            if pair.0 > pair.1 {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }

            // Only add it to the symmetric set if the lengths are different
            symmetric_set.insert(pair);
        }
    }

    // In a symmetric relationship, when you unorder the pairs, the number of non-reflexive pairs should collapse twofold
    // = pairs.len - symmetric_set.len - reflexive_num = 0
    let symmetric = pairs.len() - reflexive_num == 2 * symmetric_set.len();
    // Every point must be reflexive
    let reflexive = reflexive_num == points.len();
    // Antisymmetric if its not symmetric, or it is vacuously symmetric (the symmetric set is empty)
    let antisymmetric = symmetric_set.len() == 0;

    // {(a, b), (b, c), (a, c), (c, a)}
    log!(format!("Plen SYMlen REFlen = {} {} {} {}", pairs.len(), symmetric_set.len(), reflexive_num, pairs.len() - symmetric_set.len() - reflexive_num));
    log!(format!("PAIRS {:?}", pairs));
    log!(format!("SYM {:?}", symmetric_set));

    Ok(Relation {
        values: pairs,
        points: points,
        properties: RelationProperties {
            antisymmetric: antisymmetric,
            symmetric: symmetric,
            reflexive: reflexive,
            transitive: true
        }
    })
}