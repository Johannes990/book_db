use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use crate::utils::edit_distance::edit_distance;

struct Node {
    label: Arc<str>,
    children: HashMap<usize, Box<Node>>,
}

impl Node {
    pub fn new(label: Arc<str>) -> Self {
        Self {
            label,
            children: HashMap::new(),
        }
    }
}

pub struct BKTree {
    root: Node,
}

impl BKTree {
    pub fn new(path: &str) -> Self {
        Self {
            root: Node::new(path.into()),
        }
    }

    pub fn insert(&mut self, path: impl Into<Arc<str>>) {
        let path = path.into();
        let mut current = &mut self.root;

        loop {
            let dist = edit_distance(&current.label, &path);

            match current.children.entry(dist) {
                Entry::Occupied(entry) => {
                    current = entry.into_mut();
                }
                Entry::Vacant(entry) => {
                    entry.insert(Box::new(Node::new(path)));
                    break;
                }
            }
        }
    }

    pub fn lookup(&self, element: &str, d_max: usize) -> Vec<Arc<str>> {
        let mut results = Vec::new();
        let mut stack = vec![&self.root];

        while let Some(node) = stack.pop() {
            let dist = edit_distance(&node.label, element);

            if dist <= d_max {
                results.push(Arc::clone(&node.label));
            }

            let lower = dist.saturating_sub(d_max);
            let upper = dist + d_max;

            for key in lower..=upper {
                if let Some(child) = node.children.get(&key) {
                    stack.push(child);
                }
            }
        }

        results
    }
}
