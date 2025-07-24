use std::{collections::HashMap, time::Duration};

#[derive(Debug, PartialEq)]
pub struct Producer {
    pub nodes_per_second: f64,
}

impl Producer {
    pub fn new(nodes_per_second: f64) -> Self {
        Self { nodes_per_second }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub nodes: f64,
    pub producers: HashMap<String, Producer>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            nodes: 0.0,
            producers: HashMap::new(),
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let dt_secs = dt.as_millis() as f64 / 1000.0;

        for (_id, producer) in &self.producers {
            self.nodes += producer.nodes_per_second * dt_secs;
        }
    }
}
