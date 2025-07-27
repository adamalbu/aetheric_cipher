use dioxus::html::ol::start;

use crate::game::Node;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct GameManager {
    pub flux: f64,
    pub nodes: HashMap<String, Node>,
}

impl GameManager {
    pub fn new(starting_flux: f64) -> Self {
        Self {
            flux: starting_flux,
            nodes: HashMap::new(),
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let dt_secs = dt.as_millis() as f64 / 1000.0;

        for (_id, producer) in &self.nodes {
            self.flux += producer.get_flux_per_second() * dt_secs;
        }
    }

    pub fn upgrade_producer(&mut self, id: &str) -> Result<(), String> {
        let producer = self
            .nodes
            .get_mut(id)
            .ok_or(&format!("Producer {} not found", id))?;

        let upgrade_cost = producer.get_cost();

        if self.flux >= upgrade_cost {
            producer.upgrade();
            self.flux -= upgrade_cost;
            Ok(())
        } else {
            Err("Not enough flux".into())
        }
    }
}
