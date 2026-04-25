pub struct RelationProperties {
    pub antisymmetric: bool,
    pub symmetric: bool,
    pub reflexive: bool,
    pub transitive: bool
}

pub struct Relation {
    pub values: Vec<(String, String)>,
    pub properties: RelationProperties
}

pub fn digest_values(values: String) -> Relation {
    let mut relation = Relation {
        values: Vec::new(),
        properties: RelationProperties {
            antisymmetric: true,
            symmetric: true,
            reflexive: true,
            transitive: true
        }
    };

    if values.is_empty() {
        return relation;
    }

    relation.values = values
        .trim_matches(|c| c == '{' || c == '}' || c == ' ')
        .split("),")
        .map(|pair| {
            let cleaned = pair
                .trim()
                .trim_matches(|c| c== '(' || c == ')');

            let mut parts = cleaned.split(',');

            let first = parts.next().unwrap().trim().to_string();
            let second = parts.next().unwrap().trim().to_string();

            (first, second)
        })
        .collect();


    return relation;
}