use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::f32::INFINITY;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cost {
    previous: String,
    total: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String, i32)>, // (start, end, weight)

    #[serde(default)]
    costs: HashMap<String, Cost>
}

impl Graph {
    pub fn solve_it(&self, target: &String) -> Vec<String> {
        let head = &self.nodes[0];

        let mut queue: VecDeque<(String, String, i32)> = VecDeque::new();
        let mut costs: HashMap<String, Cost> = self.costs.clone();

        for node in self.nodes.clone() { costs.insert(node.clone(), Cost::default()); }
        
        let mut edges: Vec<(String, String, i32)> = self.get_edges_from_node(head);    
        self.order_edges_by_weight(&mut edges);
        for edge in edges { queue.push_back(edge); }

        while !queue.is_empty() { 
            if let Some(edge) = queue.pop_front() {
                let (start, end, weight) = edge;
                
                if let Some(cost) = costs.get_mut(&end) {
                    if cost.total > weight {
                        cost.previous = start.clone();
                        cost.total = weight;
                    }
                }

                let mut edges: Vec<(String, String, i32)> = self.get_edges_from_node(&end);    
                self.order_edges_by_weight(&mut edges);
                for edge in edges { queue.push_back((edge.0, edge.1, edge.2 + weight)); }
            } else { break; }
        }

        let mut target = target.clone();
        let mut path = Vec::<String>::new();

        loop {
            if let Some(node) = costs.get(&target) {
                path.push(target);
                if node.previous.is_empty() { break; }
                target = node.previous.clone();
            } else { break; }
        }

        path      
    }

    fn get_edges_from_node(&self, node: &String) -> Vec<(String, String, i32)> {
        let edges = self.edges
            .clone()
            .into_iter()
            .filter(|(start, _end, _weight)| start == node)
            .collect::<Vec<(String, String, i32)>>();
        edges   
    }

    fn order_edges_by_weight(&self, edges: &mut Vec<(String, String, i32)>) {
        edges.sort_by(|(_, _, first_weight), (_, _, second_weight)| first_weight.cmp(&second_weight))
    }
}

impl Default for Cost {
    fn default() -> Self {
        Cost {
            previous: "".to_string(),
            total: INFINITY as i32,
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            nodes: Vec::<String>::new(),
            edges: Vec::<(String, String, i32)>::new(),
            costs: HashMap::<String, Cost>::new()
        }
    }
}