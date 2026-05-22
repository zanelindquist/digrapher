use std::collections::{HashMap, HashSet};

use crate::{render::objects::point::Point, services::digraph_services::types::{GraphTheoryTypes, NodeId, NodeType, ParseError, PointRenderSymbol, PointVector, Relation}};


// GRAPH THEORY RELATIONS
#[derive(Clone, PartialEq)]
pub struct GraphTheoryRelation {
    pub relation_type: GraphTheoryTypes,
    pub nodes: Vec<Node>
}
impl GraphTheoryRelation {
    pub fn get_from_id(self, id: NodeId) -> Option<Node> {
        if let Some(node) = self.nodes.iter().find(|n| n.id == id) {
            return Some((*node).clone())
        }
        None
    }
    pub fn get_nodes_of_type_cached() {

    }
}

#[derive(Clone, PartialEq)]
pub struct GraphTheoryRelationManager {
    pub subgraphs: Vec<GraphTheoryRelation>,
    pub relation: Relation,
    pub cached_points: Option<Vec<Point>>
}
impl GraphTheoryRelationManager {
    pub fn get_points(&self) -> PointVector {
        if let Some(points) = &self.cached_points {
            return points.clone()
        };
        let mut points: Vec<Point> = vec![];
        for relation in &self.subgraphs {
            for node in relation.nodes.iter() {
                points.push((*node).point.clone())
            }
        }
        points
    }
    pub fn clear_points_cache(self) {
        if let Some(mut cache) = self.cached_points {
            cache.clear();
        }
    }
    // Mutate and save these points to the points cache
    pub fn position_points(&self) {

    }
}

#[derive(Clone, PartialEq)]
pub struct Node {
    pub id: i64,
    pub label: String,
    pub node_type: NodeType,
    pub point: Point,
    pub parents: Vec<NodeId>,
    pub children: Vec<NodeId>,
    pub depth: i32,
    pub dfs_info: Option<DepthFirstSearchInfo>
}
impl Node {
    pub fn add_child(mut self, id: NodeId) {
        self.children.push(id);
    }
    pub fn iterate_children<F>(&self, nodes: &Vec<Node>, callback: &F)
    where
        F: Fn(NodeId),
     {
        for child_id in &self.children {
            callback(*child_id);
            self.iterate_children(nodes, callback);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct DepthFirstSearchInfo {
    pub visited: bool, // For depth-first searching
    pub cycle_found: bool
}
impl DepthFirstSearchInfo {
    pub fn default() -> Self {
        Self {
            visited: false,
            cycle_found: false
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct CachedNodes {
    nodes: Option<Vec<NodeId>>
}
impl CachedNodes {
    pub fn clear_cache(self) {
        if let Some(mut cache) = self.nodes {
            cache.clear();
        }
    }
    pub fn add_cache(self, id: NodeId) {
        if let Some(mut cache) = self.nodes {
            cache.push(id);
        }
    }
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
            point: Point {
                x: 0.0,
                y: 0.0,
                bearing: 0.0,
                label: label.clone(),
                symbol: PointRenderSymbol::CIRCLE,
                index: i as i32
            },
            parents: vec![],
            children: vec![],
            depth: 0,
            dfs_info: Some(DepthFirstSearchInfo::default())
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

    Ok(GraphTheoryRelationManager {
        relation,
        cached_points: None,
        subgraphs: subgraphs
    })
}

pub fn is_cyclic(node_id: &NodeId, nodes: &Vec<Node>, visited: &mut HashSet<NodeId>, in_stack: &mut HashSet<NodeId>) -> bool {
    // If we're currently exploring this node again → cycle
    if in_stack.contains(node_id) {
        return true;
    }

    // If already fully processed, no need to re-check
    if visited.contains(node_id) {
        return false;
    }

    visited.insert(node_id.clone());
    in_stack.insert(node_id.clone());

    let node = match nodes.get(*node_id as usize) {
        Some(n) => n,
        None => return false,
    };

    for child_id in &node.children {
        if is_cyclic(child_id, nodes, visited, in_stack) {
            return true;
        }
    }

    in_stack.remove(node_id);
    false
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
        });
    }

    components
}

fn classify_relation(relation: &GraphTheoryRelation, nodes: &Vec<Node>) -> GraphTheoryTypes {
    let mut visited = HashSet::new();
    let mut in_stack = HashSet::new();
    if is_cyclic(&(0 as NodeId), &nodes, &mut visited, &mut in_stack) {
        if is_circular(nodes) {
            return GraphTheoryTypes::CIRCULAR;
        }

        if is_clique(relation, nodes) {
            return GraphTheoryTypes::CLIQUE;
        }

        if is_layered_network(relation, nodes) {
            return GraphTheoryTypes::LAYERED_NETWORK;
        }

    } else {
        if is_chain(relation, nodes) {
            return GraphTheoryTypes::CHAIN;
        }

        // Connected and acyclic are the criteria for a tree period, so this is the default
        return GraphTheoryTypes::TREE;
    }
    
    GraphTheoryTypes::NETWORK
}



// CLASSIFICATION FUNCTIONS

fn is_circular(nodes: &Vec<Node>) -> bool {
    for node in nodes {
        // Reflexive relations are already excluded
        if node.parents.len() != 1 || node.children.len() != 1 {
            return false
        }
    }
    true
}
fn is_clique(relation: &GraphTheoryRelation, nodes: &Vec<Node>) -> bool {
    // Every point has a relation with every other point
    // This means that each point has n - 1 children and n - 1 parents
    for node in nodes {
        if node.parents.len() != nodes.len() - 1 || node.children.len() != nodes.len() - 1 {
            return false
        }
    }
    true
}
fn is_layered_network(relation: &GraphTheoryRelation, nodes: &Vec<Node>) -> bool {
    false
}
fn is_chain(relation: &GraphTheoryRelation, nodes: &Vec<Node>) -> bool {
    // If its a chain, every element except for the start and end have 1 parent and one child
    for node in nodes {
        if node.node_type == NodeType::NORMAL && (node.parents.len() != 1 || node.children.len() != 1) {
            return false
        }
    }
    true
}