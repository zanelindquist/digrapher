use std::{collections::{HashMap, HashSet, VecDeque}, f32::consts::PI, vec};

use crate::{render::{objects::point::Point, styles::GraphTheoryLayoutSettings}, services::digraph_services::{digest_values::digest_values, point_layout::create_points, types::{GraphTheoryTypes, NodeId, NodeType, ParseError, PointManagementError, PointRenderSymbol, PointVector, Relation}}};


// GRAPH THEORY RELATIONS
#[derive(Clone, PartialEq)]
pub struct GraphTheoryRelation {
    pub relation_type: GraphTheoryTypes,
    pub nodes: Vec<Node>,
    pub points: Vec<Point>,
    pub positioning_settings: GraphTheoryLayoutSettings,
    // This relation's layout dimentions in logical units
    pub width_l: f32,
    pub height_l: f32
}
impl GraphTheoryRelation {
    // Position this relation's points based on the 
    pub fn position_points(&mut self) {
        match self.relation_type {
            GraphTheoryTypes::LAYERED_NETWORK => {
                self.position_points_layered();
            },
            GraphTheoryTypes::CHAIN => {
                self.position_points_chain();
            },
            GraphTheoryTypes::TREE => {
                self.position_points_tree();
            },
            // ALl of these still use the circular positioning, which is fine for circular and clique, but TODO, optimize the network layout
            GraphTheoryTypes::CIRCULAR => {
                self.position_points_circle();
            },
            GraphTheoryTypes::CLIQUE => {
                self.position_points_circle();
            },
            GraphTheoryTypes::NETWORK => {
                self.position_points_fruchterman_reingold();
            },
            GraphTheoryTypes::DISCONNECTED => {
                // Do nothing, we don't position free-standing points
            }
            _ => {
                // Do nothing
            }
        }
    }

    // POINT POSITIONING


    // Outputs points with logical coordinates in -1 to 1 x, y plane
    fn position_points_circle(&mut self) {
        let n = self.points.len();

        // If there are no points, then we don't have anything to do
        if n == 0 {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        for (i, p) in self.points.iter_mut().enumerate() {
            // Draw counterclockwise
            // Put the points on the circle with the logical unit radius 1.0
            let theta = -(i as f32) * (2.0 * PI / n as f32);
            let x = theta.cos();
            let y = theta.sin();
            p.x = x;
            p.y = y;
        }

        // Update the layout size at the end
        self.update_layout_size();
    }

    fn position_points_chain(&mut self) {
        let n = self.nodes.len();

        // If there are no points, then we don't have anything to do
        if n == 0 {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        // If there is only one point, then put it in the middle and update the layout size
        if n == 1 {
            if let Some(p) = self.points.first_mut() {
                p.x = 0.0;
                p.y = 0.0;
                p.bearing = 0.0;
            }
            self.update_layout_size();
            return;
        }

        // Find the root node, but if we can't find it then do nothing
        let Some(start_node) = self.nodes.iter().find(|node| node.node_type == NodeType::ROOT) else {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        };

        // Walk the chain in order
        let mut ordered_labels = Vec::with_capacity(n);
        let mut current = start_node;

        // Create the ordered chain of nodes
        loop {
            ordered_labels.push(current.label.clone());

            // Chain should only have one child
            if current.children.len() != 1 {
                break;
            }

            let child_id = current.children[0];
            let Some(next) = self.nodes.iter().find(|node| node.id == child_id) else {
                break;
            };

            current = next;
        }

        // Position each point based on ordered chain traversal
        for (i, label) in ordered_labels.iter().enumerate() {
            if let Some(point) = self.points.iter_mut().find(|p| p.label == *label) {
                point.x = 0.0;

                // Top to bottom
                point.y = - 1.0 + (i as f32 * self.positioning_settings.chain_settings.point_seperation_l);

                // So that labels are always displayed to the right of the point
                point.bearing = 0.0;
            }
        }

        self.update_layout_size();
    }

    fn position_points_tree(&mut self) {
        // If the tree is empty, don't do anything
        if self.nodes.is_empty() {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        // If we don't have a root, don't do anything
        let Some(root) = self.nodes.iter().find(|node| node.node_type == NodeType::ROOT) else {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        };

        // Create a map of the children that each node has
        let children_map: HashMap<NodeId, Vec<NodeId>> = self.nodes
            .iter()
            .map(|node| (node.id, node.children.clone()))
            .collect();

        let mut leaf_order: Vec<NodeId> = Vec::new();

        // Collects the nodes that don't have children
        fn collect_leaves(node_id: NodeId, children_map: &HashMap<NodeId, Vec<NodeId>>, leaf_order: &mut Vec<NodeId>) {
            if let Some(children) = children_map.get(&node_id) {
                if !children.is_empty() {
                    for child_id in children {
                        collect_leaves(*child_id, children_map, leaf_order);
                    }
                    return;
                }
            }
            leaf_order.push(node_id);
        }

        // Collect all childless nodes
        collect_leaves(root.id, &children_map, &mut leaf_order);

        let leaf_count = leaf_order.len().max(1);
        // These make it so that the tree's triangles don't get horizontally squished
        let layer_height = self.positioning_settings.tree_settings.layer_height_l.max(0.1);
        let horizontal_point_spacing = layer_height * 2.0 / (3.0 as f32).sqrt();
        let leftmost_x = leaf_count as f32 * horizontal_point_spacing / 2.0 * -1.0;

        let mut x_positions: HashMap<NodeId, f32> = HashMap::new();
        let mut y_positions: HashMap<NodeId, f32> = HashMap::new();

        // If there is only one node, put it in the center
        if leaf_count == 1 {
            x_positions.insert(leaf_order[0], 0.0);
        } else {
            for (i, leaf_id) in leaf_order.iter().enumerate() {
                x_positions.insert(*leaf_id, leftmost_x + i as f32 * horizontal_point_spacing);
            }
        }

        fn assign_positions(
            node_id: NodeId,
            depth: i32,
            children_map: &HashMap<NodeId, Vec<NodeId>>,
            x_positions: &mut HashMap<NodeId, f32>,
            y_positions: &mut HashMap<NodeId, f32>,
            layer_height: f32,
        ) -> f32 {
            // Get this node's children
            let children = children_map.get(&node_id).map(|children| children.as_slice()).unwrap_or(&[]);
            
            // If this is an end node, then get its predetermined position we decided earlier
            let x = if children.is_empty() {
                *x_positions.get(&node_id).unwrap_or(&0.0)
            }
            // Else, iterate downward to see how many final nodes originiate from this node and take the average, which will give us a centerpoint
            // so essentially, x positions are built from the bottom up, because the end nodes we have already positioned tell the parent nodes where to stand
            else {
                let sum: f32 = children
                    .iter()
                    .map(|child_id| assign_positions(*child_id, depth + 1, children_map, x_positions, y_positions, layer_height))
                    .sum();
                sum / children.len() as f32
            };

            x_positions.insert(node_id, x);
            // Assign the depth of this point
            y_positions.insert(node_id, -1.0 + depth as f32 * layer_height);
            x
        }

        // Assign positions for all of the points
        assign_positions(root.id, 0, &children_map, &mut x_positions, &mut y_positions, layer_height);

        // Create a way to look up a point by the label since the points aren't linked to the node's id
        let label_to_id: HashMap<String, NodeId> = self.nodes
            .iter()
            .map(|node| (node.label.clone(), node.id))
            .collect();

        // Finally, position each point based on it's node's position
        for point in self.points.iter_mut() {
            if let Some(node_id) = label_to_id.get(&point.label) {
                if let (Some(x), Some(y)) = (x_positions.get(node_id), y_positions.get(node_id)) {
                    point.x = *x;
                    point.y = *y;
                    // Causes label to render to the right of the point
                    point.bearing = 0.0;
                }
            }
        }

        // Update the layout size at the end
        self.update_layout_size();
    }

    fn position_points_layered(&mut self) {
        // If we have no nodes, do nothing
        if self.nodes.is_empty() {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        // Build parent counts and adjacency
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut children_map: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for node in &self.nodes {
            in_degree.insert(node.id, node.parents.len());
            children_map.insert(node.id, node.children.clone());
        }

        // Kahn's algorithm to assign layers (level = max parent level + 1)
        let mut level_map: HashMap<NodeId, i32> = HashMap::new();
        // Start with roots in the queue
        let mut queue: Vec<NodeId> = self.nodes.iter().filter(|n| n.node_type == NodeType::ROOT).map(|n| n.id).collect();

        // Map the roots to layer 0
        for &r in &queue {
            level_map.insert(r, 0);
        }

        let mut idx = 0;
        while idx < queue.len() {
            let u = queue[idx];
            idx += 1;
            let u_level = *level_map.get(&u).unwrap_or(&0);
            if let Some(children) = children_map.get(&u) {
                for &v in children {
                    let entry = level_map.entry(v).or_insert(i32::MIN);
                    if *entry < u_level + 1 {
                        *entry = u_level + 1;
                    }
                    // decrement in-degree and push when all parents processed
                    if let Some(d) = in_degree.get_mut(&v) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            queue.push(v);
                        }
                    }
                }
            }
        }

        let max_level = *level_map.values().max().unwrap_or(&0);
        let num_layers = (max_level + 1) as usize;
        if num_layers == 0 {
            self.position_points_circle();
            return;
        }

        // group nodes by level
        let mut layers: Vec<Vec<NodeId>> = vec![Vec::new(); num_layers];
        for (&id, &lvl) in level_map.iter() {
            let i = lvl as usize;
            layers[i].push(id);
        }

        // For each layer, assign y positions (vertical distribution) and x by layer index
        for (layer_idx, layer_nodes) in layers.iter().enumerate() {
            let x = if num_layers == 1 { 0.0 } else { -1.0 + layer_idx as f32 * (2.0 / (num_layers - 1) as f32) };
            let count = layer_nodes.len();
            if count == 0 { continue; }

            // Taper first and last layers: vertically centered and slightly closer together
            let is_taper_layer = layer_idx == 0 || layer_idx + 1 == num_layers;
            let taper_factor: f32 = self.positioning_settings.layered_settings.bookend_taper_scale_l;

            if count == 1 {
                let nid = layer_nodes[0];
                for point in self.points.iter_mut() {
                    if let Some(node) = self.nodes.iter().find(|n| n.id == nid) {
                        if point.label == node.label {
                            point.x = x;
                            point.y = 0.0;
                            point.bearing = 0.0;
                        }
                    }
                }
            } else {
                let span = if is_taper_layer { 2.0 * taper_factor } else { 2.0 };
                let start_y = -span / 2.0;
                let spacing = span / (count - 1) as f32;

                for (i, nid) in layer_nodes.iter().enumerate() {
                    let y = start_y + i as f32 * spacing;
                    if let Some(node) = self.nodes.iter().find(|n| n.id == *nid) {
                        for point in self.points.iter_mut() {
                            if point.label == node.label {
                                point.x = x;
                                point.y = y;
                                point.bearing = 0.0;
                            }
                        }
                    }
                }
            }
        }

        // Update the layout size at the end
        self.update_layout_size();
    }

    // This function is direction agnostic (undirected)
    fn position_points_network_rings(&mut self) {
        // If there are no nodes, do nothing
        if self.nodes.len() == 0 {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        let mut node_depths: HashMap<NodeId, i32> = HashMap::default();

        fn bfs(start: NodeId, nodes: &[Node]) -> HashMap<NodeId, i32> {
            let mut distances = HashMap::new();
            let mut queue = VecDeque::new();

            distances.insert(start, 0);
            queue.push_back(start);

            while let Some(current) = queue.pop_front() {
                let depth = distances[&current];

                let node = nodes.iter().find(|n| n.id == current).unwrap();

                for neighbor in node.children.iter().chain(node.parents.iter()) {
                    if !distances.contains_key(neighbor) {
                        distances.insert(*neighbor, depth + 1);
                        queue.push_back(*neighbor);
                    }
                }
            }

            distances
        }

        // Determine the outside points by who has the most conenctions
        let mut outer_nodes: Vec<(NodeId, usize)>  = vec![];
        for node in &self.nodes {
            let current_connections = node.children.len() + node.parents.len();
            // If we haven't reached the quota yet
            if outer_nodes.len() < (self.nodes.len() as f32 / 5.0).max(3.0) as usize {
                outer_nodes.push((node.id, current_connections));
                // Sort the points
                outer_nodes.sort_by(|a, b| b.1.cmp(&a.1));
                continue
            }
            // Check and see if this point has more connections that the least now
            if current_connections > outer_nodes.get(outer_nodes.len() - 1).unwrap().1 {
                outer_nodes.pop();
                outer_nodes.push((node.id, current_connections));
                // Sort the points
                outer_nodes.sort_by(|a, b| b.1.cmp(&a.1));
            }
        }

        gloo_console::log!(format!("{:?}", outer_nodes));

        // Create the point depths table
        for node in outer_nodes {
            for (id, depth) in bfs(node.0.clone(), &self.nodes) {
                node_depths
                .entry(id)
                .and_modify(|d| *d = (*d)
                .min(depth))
                .or_insert(depth);
            }
        }

        gloo_console::log!(format!("{:?}", node_depths));

        // Group points by depth
        let mut depth_groups: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, point) in self.points.iter().enumerate() {
            if let Some(node) = self.nodes.iter().find(|n| n.label == point.label) {
                if let Some(depth) = node_depths.get(&node.id) {
                    depth_groups.entry(*depth).or_default().push(i);
                }
            }
        }

        let max_depth = *node_depths.values().max().unwrap_or(&0);

        // Position each ring independently
        for (depth, point_indices) in depth_groups {
            let count = point_indices.len();

            // depth 0 = outer ring
            // max_depth = center ring
            let radius = if max_depth == 0 {
                2.0
            } else {
                1.5 - (depth as f32 / max_depth as f32)
            };

            for (i, point_index) in point_indices.iter().enumerate() {
                let theta = -(i as f32) * (2.0 * PI / count as f32);

                self.points[*point_index].x = radius * theta.cos();
                self.points[*point_index].y = radius * theta.sin();
            }
        }

        // Update the layout size at the end
        self.update_layout_size();
    }

    fn position_points_fruchterman_reingold(&mut self) {
        let n = self.points.len();
        if n == 0 {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        let width = self.positioning_settings.network_settings.max_width_l.max(0.1);
        let height = self.positioning_settings.network_settings.max_height_l.max(0.1);
        let max_iter = self.positioning_settings.network_settings.max_fr_iterations;
        
        let area = width * height;
        let k = (area / n as f32).sqrt();
        let mut temperature = ((width + height) / 2.0).max(0.05);
        let cooling = temperature / max_iter as f32;

        // Use existing positions when available, otherwise initialize on a circle.
        if self.points.iter().all(|p| p.x == 0.0 && p.y == 0.0) {
            for (i, point) in self.points.iter_mut().enumerate() {
                let theta = 2.0 * PI * i as f32 / n as f32;
                point.x = theta.cos() * width * 0.5;
                point.y = theta.sin() * height * 0.5;
            }
        }

        let mut positions: Vec<(f32, f32)> = self.points.iter().map(|p| (p.x, p.y)).collect();
        let mut displacements: Vec<(f32, f32)> = vec![(0.0, 0.0); n];

        // Build a quick edge lookup for undirected attraction
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n];
        let label_to_index: HashMap<String, usize> = self.points
            .iter()
            .enumerate()
            .map(|(idx, p)| (p.label.clone(), idx))
            .collect();

        for node in &self.nodes {
            for child_id in &node.children {
                if let Some(child) = self.nodes.iter().find(|n| n.id == *child_id) {
                    if let (Some(&u), Some(&v)) = (label_to_index.get(&node.label), label_to_index.get(&child.label)) {
                        adjacency[u].push(v);
                        adjacency[v].push(u);
                    }
                }
            }
        }

        for _iteration in 0..max_iter {
            // Reset displacements
            for disp in displacements.iter_mut() {
                *disp = (0.0, 0.0);
            }

            // Repulsive forces for every pair of vertices
            for i in 0..n {
                for j in (i + 1)..n {
                    let dx = positions[i].0 - positions[j].0;
                    let dy = positions[i].1 - positions[j].1;
                    let distance = (dx * dx + dy * dy).sqrt().max(std::f32::EPSILON);
                    let force = (k * k) / distance;
                    let direction_x = dx / distance;
                    let direction_y = dy / distance;
                    let displacement_x = direction_x * force;
                    let displacement_y = direction_y * force;

                    displacements[i].0 += displacement_x;
                    displacements[i].1 += displacement_y;
                    displacements[j].0 -= displacement_x;
                    displacements[j].1 -= displacement_y;
                }
            }

            // Attractive forces along edges
            for u in 0..n {
                for &v in &adjacency[u] {
                    if u == v {
                        continue;
                    }
                    let dx = positions[u].0 - positions[v].0;
                    let dy = positions[u].1 - positions[v].1;
                    let distance = (dx * dx + dy * dy).sqrt().max(std::f32::EPSILON);
                    let force = (distance * distance) / k;
                    let direction_x = dx / distance;
                    let direction_y = dy / distance;
                    let displacement_x = direction_x * force;
                    let displacement_y = direction_y * force;

                    displacements[u].0 -= displacement_x;
                    displacements[u].1 -= displacement_y;
                    displacements[v].0 += displacement_x;
                    displacements[v].1 += displacement_y;
                }
            }

            // Move vertices and cool
            for i in 0..n {
                let dx = displacements[i].0;
                let dy = displacements[i].1;
                let length = (dx * dx + dy * dy).sqrt().max(std::f32::EPSILON);
                let limited = temperature.min(length);
                positions[i].0 += dx / length * limited;
                positions[i].1 += dy / length * limited;

                positions[i].0 = positions[i].0.clamp(-width / 2.0, width / 2.0);
                positions[i].1 = positions[i].1.clamp(-height / 2.0, height / 2.0);
            }

            temperature = (temperature - cooling).max(0.01);
        }

        for (point, (x, y)) in self.points.iter_mut().zip(positions.into_iter()) {
            point.x = x;
            point.y = y;
            point.bearing = 0.0;
        }

        self.update_layout_size();
    }

    fn update_layout_size(&mut self) {
        if self.points.is_empty() {
            self.width_l = 0.0;
            self.height_l = 0.0;
            return;
        }

        let min_x = self.points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let max_x = self.points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = self.points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let max_y = self.points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);

        self.width_l = (max_x - min_x).max(0.0);
        self.height_l = (max_y - min_y).max(0.0);
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphTheoryRelationManager {
    pub subgraphs: Vec<GraphTheoryRelation>,
    pub relation: Relation,
}
impl GraphTheoryRelationManager {
    pub fn empty() -> Self {
        Self {
            subgraphs: vec![],
            relation: Relation::empty()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.subgraphs.is_empty()
    }

    // RELATION MODIFICATION

    pub fn create_point(&mut self, label: String, lx: f32, ly: f32) -> Result<Self, PointManagementError> {
        // Add the point to our relation
        let point = Point::new(lx, ly, 0.0, label.clone(), PointRenderSymbol::CIRCLE, 0);

        self.relation.points.insert(point.label.clone());

        // Ensure disconnected graph exists, if not, create one to put the new point into
        if !self.subgraphs.iter().any(|s| s.relation_type == GraphTheoryTypes::DISCONNECTED) {
            let dc_layer = GraphTheoryRelation {
                relation_type: GraphTheoryTypes::DISCONNECTED,
                nodes: vec![],
                points: vec![],
                positioning_settings: GraphTheoryLayoutSettings::default(),
                width_l: 0.0,
                height_l: 0.0,
            };
            self.subgraphs.push(dc_layer);
        }

        // Find and add point to the disconnected graph
        if let Some(dc) = self.subgraphs.iter_mut().find(|s| s.relation_type == GraphTheoryTypes::DISCONNECTED) {
            dc.points.push(point.clone());
            Ok((*self).clone())
        } else {
            Err(PointManagementError::new("Point creation failed"))
        }
    }
    // This should only alter cosmetics of position and label
    pub fn edit_point(&mut self, label: String, lx: f32, ly: f32) -> Result<Self, PointManagementError> {
        for graph in &mut self.subgraphs {
            for point in graph.points.iter_mut() {
                if point.label == label {
                    point.x = lx;
                    point.y = ly;

                    return Ok((*self).clone())
                }
            }
        }

        Err(PointManagementError::new("Point not found"))
    }

    pub fn delete_point(&mut self, label: String) -> Result<Self, PointManagementError> {
        // Find the point to delete
        for graph in &mut self.subgraphs {
            let initial_len = graph.points.len();

            // Remove the point object
            graph.points.retain(|p| p.label != label);
            // Remove relations that contained the point
            self.relation.values.retain(|v| v.0 != label && v.1 != label);
            // Remove the retained point in 
            self.relation.points.retain(|v| *v != label);

            if graph.points.len() != initial_len {
                // If the relation successfully changed, we we need to re-evaluate the manager's logical structure
                return self.reevaluate_strucure()
            }
        }
        Err(PointManagementError::new("Point not found"))
    }

    pub fn connect_edge(&mut self, from_label: String, to_label: String) -> Result<Self, PointManagementError> {
        // Add the new edge to the relation
        self.relation.values.insert((from_label, to_label));

        // Re-evaluating the manager's logical structure will do the rest for us
        self.reevaluate_strucure()
    }

    // Collect all of the points from subgraphs
    pub fn get_points(&self) -> PointVector {
        let mut points = Vec::new();
        for graph in &self.subgraphs {
            for point in graph.points.iter() {
                points.push(point.clone());
            }
        }
        points
    }
    // Position points relatively within each subgraph, then absolutely across all subgraphs
    pub fn position_points(&mut self) {
        for graph in &mut self.subgraphs {
            graph.position_points();
        }

        // Apply absolute positioning to all points based on subgraph layout
        let horizontal_gap = 1.0;
        let subgraph_widths: Vec<f32> = self.subgraphs.iter().map(|g| g.width_l.max(0.1)).collect();
        let total_width: f32 = subgraph_widths.iter().sum::<f32>() + horizontal_gap * (subgraph_widths.len().saturating_sub(1) as f32);
        let mut x_cursor = -total_width / 2.0;

        for (graph, width) in self.subgraphs.iter_mut().zip(subgraph_widths.iter()) {
            let center_x = x_cursor + width / 2.0;
            x_cursor += width + horizontal_gap;

            for point in graph.points.iter_mut() {
                point.x += center_x;
            }
        }
    }

    pub fn reevaluate_strucure(&mut self) -> Result<Self, PointManagementError> {
        // Rebuild our subgraphs by
        // rebuilding node strucutre
        // reevaluating strucutre
        // linking existing points to their correct relation

        // Build the new subgraphs and link existing point positions
        let mut subgraphs = build_subgraphs_from_relation(&self.relation);
        self.link_existing_points(&mut subgraphs)?;
        self.subgraphs = subgraphs;

        // Rebuild the relation's properties
        if let Ok(new_relation) = digest_values(self.to_string()) {
            self.relation.properties = new_relation.properties;
        }

        Ok((*self).clone())
    }

    fn link_existing_points(&self, subgraphs: &mut [GraphTheoryRelation]) -> Result<(), PointManagementError> {
        let mut existing_points = self.get_points();
        for relation in subgraphs.iter_mut() {
            for point in &mut relation.points {
                if let Some(index) = existing_points.iter().position(|p| p.label == point.label) {
                    let existing_point = existing_points.remove(index);
                    point.x = existing_point.x;
                    point.y = existing_point.y;
                } else {
                    return Err(PointManagementError::new("Failed to link points"));
                }
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut items: Vec<String> = self.relation.values
            .iter()
            .map(|(a, b)| format!("({}, {})", a, b))
            .collect();

        // optional: make output deterministic
        items.sort();

        format!("{{{}}}", items.join(", "))
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
    let subgraphs = build_subgraphs_from_relation(&relation);

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

        let mut component_labels: Vec<String> = component_nodes.iter().map(|n| n.label.clone()).collect();
        component_labels.sort();

        // Return a generic graph theory relation object
        components.push(GraphTheoryRelation {
            relation_type: GraphTheoryTypes::NETWORK,
            nodes: component_nodes,
            points: create_points(&component_labels),
            positioning_settings: GraphTheoryLayoutSettings::default(),
            width_l: 1.0,
            height_l: 1.0
        });
    }

    components
}

// Build subgraphs (nodes -> components) and classify each component.
fn build_subgraphs_from_relation(relation: &Relation) -> Vec<GraphTheoryRelation> {
    // Create stable node ordering
    let mut sorted_points: Vec<String> = relation.points.clone().into_iter().collect();
    sorted_points.sort();

    // Build nodes
    let mut nodes: Vec<Node> = Vec::with_capacity(sorted_points.len());
    let mut label_to_id: HashMap<String, usize> = HashMap::new();

    // Create each point as a node with default values
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

    // Populate edges into node structure
    for (a, b) in &relation.values {
        if let Some(parent_id) = label_to_id.get(a) {
            if let Some(child_id) = label_to_id.get(b) {
                // Reflexive edge
                if parent_id == child_id {
                    continue;
                }
                // Add child
                nodes[*parent_id].children.push(*child_id as i64);
                // Assign parent
                nodes[*child_id].parents.push(*parent_id as i64);
            }
        }
    }

    // Identify roots/ends
    for node in &mut nodes {
        if node.parents.is_empty() {
            node.node_type = NodeType::ROOT;
        }
        if node.children.is_empty() {
            node.node_type = NodeType::END;
        }
    }

    // Split into components and classify each
    let mut subgraphs = split_into_components(&nodes);
    for relation in &mut subgraphs {
        relation.relation_type = classify_relation(&relation.nodes);
    }

    subgraphs
}


fn classify_relation(nodes: &Vec<Node>) -> GraphTheoryTypes {
    if is_isolated(&nodes) {
        return GraphTheoryTypes::ISOLATED_POINT
    }

    if is_cyclic(&nodes) {
        if is_circular(nodes) {
            return GraphTheoryTypes::CIRCULAR;
        }

        if is_clique(nodes) {
            return GraphTheoryTypes::CLIQUE;
        }

    } else {
        if is_chain(nodes) {
            return GraphTheoryTypes::CHAIN;
        }

        if is_tree(nodes) {
            return GraphTheoryTypes::TREE;
        }

        if is_layered_network(nodes) {
            return GraphTheoryTypes::LAYERED_NETWORK;
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

fn is_isolated(nodes: &Vec<Node>) -> bool {
    nodes.len() == 1
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
    if nodes.is_empty() {
        return false;
    }

    // Must be acyclic to be a layered feed-forward network
    if is_cyclic(nodes) {
        return false;
    }

    // Roots = nodes with no parents
    let roots: Vec<&Node> = nodes.iter().filter(|n| n.parents.is_empty()).collect();
    if roots.is_empty() {
        return false;
    }

    // BFS from roots and assign integer layer indices
    let mut level_map: HashMap<NodeId, i32> = HashMap::new();
    let mut q: VecDeque<NodeId> = VecDeque::new();
    for r in roots {
        level_map.insert(r.id, 0);
        q.push_back(r.id);
    }

    while let Some(id) = q.pop_front() {
        let level = *level_map.get(&id).unwrap();
        let node = match nodes.iter().find(|n| n.id == id) {
            Some(n) => n,
            None => return false,
        };

        for child in &node.children {
            let expected = level + 1;
            if let Some(existing) = level_map.get(child) {
                // child already assigned — must match expected layer
                if *existing != expected {
                    return false;
                }
            } else {
                level_map.insert(*child, expected);
                q.push_back(*child);
            }
        }
    }

    // All nodes should be assigned a layer (connected feed-forward structure)
    if level_map.len() != nodes.len() {
        return false;
    }

    // Need at least three layers: start, intermediate, end
    let max_level = level_map.values().cloned().max().unwrap_or(0);
    if max_level < 2 {
        return false;
    }

    // Verify every edge goes strictly from level L to L+1
    for node in nodes {
        let node_level = match level_map.get(&node.id) { Some(l) => *l, None => return false };
        for child in &node.children {
            match level_map.get(child) {
                Some(cl) if *cl == node_level + 1 => (),
                _ => return false,
            }
        }
    }

    true
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