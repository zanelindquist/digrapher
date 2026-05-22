use std::{collections::{HashMap, HashSet}, f32::consts::PI};

use gloo_console::log;

use crate::{render::objects::point::Point, services::digraph_services::{types::{GraphTheoryTypes, NodeId, NodeType, ParseError, PointRenderSymbol, PointVector, Relation}}};


// GRAPH THEORY RELATIONS
#[derive(Clone, PartialEq)]
pub struct GraphTheoryRelation {
    pub relation_type: GraphTheoryTypes,
    pub nodes: Vec<Node>,
    pub points: Vec<Point>
}
impl GraphTheoryRelation {
    pub fn position_points(&mut self) {
        match self.relation_type {
            GraphTheoryTypes::CIRCULAR => {
                self.position_points_circle();
            },
            _ => {}
        }
    }

    // POINT POSITIONING


    // Outputs points with logical coordinates in -1 to 1 x, y plane
    fn position_points_circle(&mut self) {
        let n = self.points.len();

        log!(n);

        for (i, p) in self.points.iter_mut().enumerate() {
            // Draw counterclockwise
            let theta = -(i as f32) * (2.0 * PI / n as f32);
            let x = theta.cos();
            let y = theta.sin();
            p.x = x;
            p.y = y;
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphTheoryRelationManager {
    pub subgraphs: Vec<GraphTheoryRelation>,
    pub relation: Relation,
}
impl GraphTheoryRelationManager {
    pub fn get_points(&self) -> PointVector {
        self.subgraphs.iter().flat_map(|s| s.points.iter().cloned()).collect()
    }
    // Mutate and save these points to the points cache
    pub fn position_points(&mut self) {
        for mut graph in &mut self.subgraphs {
            graph.position_points();
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Node {
    pub id: i64,
    pub label: String,
    pub node_type: NodeType,
    pub parents: Vec<NodeId>,
    pub children: Vec<NodeId>,
    pub depth: i32,
}


pub fn process_reltaion(relation: Relation) -> Result<GraphTheoryRelationManager, ParseError> {

    // Create stable node ordering
    let mut sorted_points: Vec<String> = relation.points.clone().into_iter().collect();
    sorted_points.sort();

    // Build nodes
    let mut nodes: Vec<Node> = vec![];
    let mut label_to_id: HashMap<String, usize> = HashMap::new();

    // Create the each point as a node with deault values
    for (i, label) in sorted_points.iter().enumerate() {
        label_to_id.insert(label.clone(), i);

        nodes.push(Node {
            id: i as i64,
            label: label.clone(),
            node_type: NodeType::NORMAL,
            parents: vec![],
            children: vec![],
            depth: 0,
        });
    }

    // Populate graph edges into node structure
    for (a, b) in &relation.values {
        if let Some(parent_id) = label_to_id.get(a) {
        if  let Some(child_id) = label_to_id.get(b) {
            // Reflexive edge
            if parent_id == child_id {
                continue;
            }
            // Add child
            nodes[*parent_id].children.push(*child_id as i64);
            // Assign parent
            nodes[*child_id].parents.push(*parent_id as i64);
        }}
    }

    // Identify roots
    for node in &mut nodes {
        if node.parents.is_empty() {
            node.node_type = NodeType::ROOT;
        }
        if node.children.is_empty() {
            node.node_type = NodeType::END;
        }
    }

    // At this point, nodes are built, and we just need to diagnose the relationship
    
    // First we need to see if the relation is compound
    let mut subgraphs = split_into_components(&nodes);
    // Then for each individual connected part
    for mut relation in &mut subgraphs {
        // See if it is cyclic, because that will tell us a lot
        relation.relation_type =  classify_relation(relation, &relation.nodes);
    }

    let mut gm = GraphTheoryRelationManager {
        relation,
        subgraphs: subgraphs
    };

    gm.position_points();

    Ok(gm)
}

pub fn split_into_components( nodes: &Vec<Node>) -> Vec<GraphTheoryRelation> {
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut components = Vec::new();

    // For every node
    for start_idx in 0..nodes.len() {
        let start_id = start_idx as NodeId;

        // Already assigned to a component
        if visited.contains(&start_id) {
            continue;
        }

        let mut stack = vec![start_id];
        let mut component_nodes = Vec::new();

        while let Some(node_id) = stack.pop() {
            if !visited.insert(node_id) {
                continue;
            }

            let node = &nodes[node_id as usize];

            component_nodes.push(node.clone());

            // Traverse children
            for child_id in &node.children {
                if !visited.contains(child_id) {
                    stack.push(*child_id);
                }
            }

            // Traverse parents
            for parent_id in &node.parents {
                if !visited.contains(parent_id) {
                    stack.push(*parent_id);
                }
            }
        }

        components.push(GraphTheoryRelation {
            relation_type: GraphTheoryTypes::NETWORK,
            nodes: component_nodes,
            points: vec![]
        });
    }

    components
}

fn classify_relation(relation: &GraphTheoryRelation, nodes: &Vec<Node>) -> GraphTheoryTypes {
    log!(
        "{}",
        nodes
            .iter()
            .map(|n| n.label.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    if is_cyclic(&nodes) {
        if is_circular(nodes) {
            return GraphTheoryTypes::CIRCULAR;
        }

        if is_clique(nodes) {
            return GraphTheoryTypes::CLIQUE;
        }

        if is_layered_network(nodes) {
            return GraphTheoryTypes::LAYERED_NETWORK;
        }

    } else {
        if is_chain(nodes) {
            return GraphTheoryTypes::CHAIN;
        }

        if is_tree(nodes) {
            return GraphTheoryTypes::TREE;
        }
    }
    
    GraphTheoryTypes::NETWORK
}



// CLASSIFICATION FUNCTIONS

pub fn is_cyclic(nodes: &Vec<Node>) -> bool {
    fn dfs(node_id: NodeId, nodes: &Vec<Node>, visited: &mut HashSet<NodeId>, in_stack: &mut HashSet<NodeId>) -> bool {
        // Found a back-edge
        if in_stack.contains(&node_id) {
            return true;
        }

        // Already fully explored
        if visited.contains(&node_id) {
            return false;
        }

        visited.insert(node_id);
        in_stack.insert(node_id);

        let Some(node) = nodes.iter().find(|n| n.id == node_id) else {
            return false;
        };

        for child_id in &node.children {
            if dfs(*child_id, nodes, visited, in_stack) {
                return true;
            }
        }

        in_stack.remove(&node_id);

        false
    }

    let mut visited = HashSet::new();
    let mut in_stack = HashSet::new();

    // Must check every component
    for node in nodes {
        if !visited.contains(&node.id) {
            if dfs(node.id, nodes, &mut visited, &mut in_stack) {
                return true;
            }
        }
    }

    false
}

fn is_circular(nodes: &Vec<Node>) -> bool {
    for node in nodes {
        // Reflexive relations are already excluded
        if node.parents.len() != 1 || node.children.len() != 1 {
            return false
        }
    }
    true
}
fn is_clique(nodes: &Vec<Node>) -> bool {
    // Every point has a relation with every other point
    // This means that each point has n - 1 children and n - 1 parents
    for node in nodes {
        if node.parents.len() != nodes.len() - 1 || node.children.len() != nodes.len() - 1 {
            return false
        }
    }
    true
}
fn is_layered_network(nodes: &Vec<Node>) -> bool {
    false
}
fn is_chain(nodes: &Vec<Node>) -> bool {
    // If its a chain, every element except for the start and end have 1 parent and one child
    for node in nodes {
        if node.node_type == NodeType::NORMAL && (node.parents.len() != 1 || node.children.len() != 1) {
            return false
        } else if node.node_type == NodeType::END && (node.parents.len() != 1) {
            return  false
        } else if node.node_type == NodeType::ROOT && (node.children.len() != 1) {
            return false
        }
    }
    true
}
fn is_tree(nodes: &Vec<Node>) -> bool {
    for node in nodes {
        if (node.node_type == NodeType::NORMAL || node.node_type == NodeType::END) && (node.parents.len() != 1) {
            return false
        }
    }
    true
}